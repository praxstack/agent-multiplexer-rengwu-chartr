//! App-owned effects: one activity poll, persistent unread state and native delivery.

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

use chartr_herdr::control::{Client, Session};
use gpui::{App, AppContext as _, Context, Entity, Global, SystemNotification, Task};

use super::{Model, Target};
use crate::persistence::{self, StateStore};

struct GlobalService(Entity<Service>);
impl Global for GlobalService {}

struct Writer {
    store: StateStore,
    revision: u64,
    requested: u64,
}

pub(crate) struct Service {
    model: Model,
    writer: Option<Arc<Mutex<Writer>>>,
    revision: u64,
    connected: bool,
    monitor: Option<Task<()>>,
    pending_navigation: Option<Target>,
    pending_save: Option<Task<()>>,
}

pub fn init(cx: &mut App) {
    let mut model = Model::default();
    let writer = match persistence::state_file().and_then(StateStore::open) {
        Ok(store) => {
            match store.load_attention() {
                Ok(Some(json)) => match Model::from_json(&json) {
                    Ok(saved) => model = saved,
                    Err(error) => eprintln!("Could not restore agent attention: {error}"),
                },
                Ok(None) => {}
                Err(error) => eprintln!("Could not read agent attention: {error}"),
            }
            Some(Arc::new(Mutex::new(Writer { store, revision: 0, requested: 0 })))
        }
        Err(error) => {
            eprintln!("Could not open agent attention storage: {error}");
            None
        }
    };
    let attention_entity = cx.new(|_| Service {
        model,
        writer,
        revision: 0,
        connected: false,
        monitor: None,
        pending_navigation: None,
        pending_save: None,
    });
    cx.set_global(GlobalService(attention_entity));
    cx.on_system_notification_response(|response, cx| {
        let Some(json) = response.tag.strip_prefix("chartr-attention:") else { return };
        let Ok(target) = serde_json::from_str::<Target>(json) else { return };
        let Some(service) = service(cx) else { return };
        if service.read(cx).model.resolve(&target).is_none() {
            return;
        }
        service.update(cx, |service, _| service.pending_navigation = Some(target));
        if !cx
            .windows()
            .iter()
            .any(|window| window.downcast::<crate::app::WorkspaceWindow>().is_some())
        {
            let cwd = std::env::current_dir().unwrap_or_default();
            if let Err(error) = crate::open_workspace_window(cwd, None, cx) {
                eprintln!("Could not open attention target: {error}");
            }
        }
        retry_navigation(cx);
    });
    cx.on_app_quit(|cx| {
        if let Some(service) = service(cx) {
            service.update(cx, |service, _| {
                service.monitor.take();
                service.pending_save.take();
                service.flush();
            });
        }
        async {}
    })
    .detach();
}

pub(crate) fn service(cx: &App) -> Option<Entity<Service>> {
    cx.try_global::<GlobalService>().map(|global| global.0.clone())
}

pub fn unread(terminal: &str, cx: &App) -> bool {
    service(cx).is_some_and(|service| service.read(cx).model.unread(terminal))
}

pub fn acknowledge(terminal: &str, cx: &mut App) {
    let Some(service) = service(cx) else { return };
    service.update(cx, |service, cx| {
        let Some(target) = service.model.target(terminal) else { return };
        if service.model.acknowledge(&target) {
            cx.dismiss_system_notification(&tag(&target));
            service.save(cx);
            cx.notify();
        }
    });
}

pub fn connect(client: Client, cx: &mut App) {
    let Some(service) = service(cx) else { return };
    service.update(cx, |service, cx| {
        if service.monitor.is_some() {
            return;
        }
        let executor = cx.background_executor().clone();
        service.monitor = Some(cx.spawn(async move |this, cx| {
            loop {
                let probe = client.clone();
                let snapshot = executor.spawn(async move { probe.session_states() }).await;
                if this
                    .update(cx, |service, cx| match snapshot {
                        Ok(sessions) => service.receive(sessions, cx),
                        // Keep existing unread state. The next read is a silent baseline.
                        Err(_) => service.connected = false,
                    })
                    .is_err()
                {
                    break;
                }
                executor.timer(Duration::from_secs(1)).await;
            }
        }));
    });
}

pub fn retry_navigation(cx: &mut App) {
    let Some(service) = service(cx) else { return };
    if !service.read(cx).connected {
        return;
    }
    let Some(target) = service.read(cx).pending_navigation.clone() else { return };
    let terminal = service.read(cx).model.resolve(&target).cloned();
    let Some(terminal) = terminal else {
        service.update(cx, |service, _| service.pending_navigation = None);
        return;
    };
    for window in cx.windows() {
        let Some(window) = window.downcast::<crate::app::WorkspaceWindow>() else { continue };
        if window
            .update(cx, |workspace, window, cx| {
                workspace.navigate_attention(&target.terminal_id, &terminal.pane_id, window, cx)
            })
            .unwrap_or(false)
        {
            service.update(cx, |service, _| service.pending_navigation = None);
            cx.activate(true);
            break;
        }
    }
}

impl Service {
    fn receive(&mut self, sessions: Vec<Session>, cx: &mut Context<Self>) {
        let focused: HashSet<String> = cx
            .windows()
            .into_iter()
            .filter_map(|window| {
                let window = window.downcast::<crate::app::WorkspaceWindow>()?;
                window
                    .update(cx, |workspace, window, cx| {
                        workspace.focused_attention_terminal(window, cx)
                    })
                    .ok()
                    .flatten()
            })
            .collect();
        let previous: Vec<_> =
            self.model.terminals.keys().filter_map(|id| self.model.target(id)).collect();
        let change = self.model.apply(sessions, !self.connected, &focused);
        self.connected = true;
        for target in previous {
            if self.model.resolve(&target).is_none() {
                cx.dismiss_system_notification(&tag(&target));
            }
        }
        if self.pending_navigation.is_some() {
            cx.defer(super::retry_navigation);
        }
        for target in change.notify {
            let Some(terminal) = self.model.resolve(&target) else { continue };
            cx.show_system_notification(SystemNotification {
                tag: tag(&target).into(),
                title: terminal.label.clone().into(),
                body: "Agent may need your attention".into(),
                actions: Vec::new(),
            });
        }
        if change.changed {
            self.save(cx);
            cx.notify();
        }
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        let Some(writer) = self.writer.clone() else { return };
        let Ok(json) = serde_json::to_string(&self.model) else { return };
        self.revision += 1;
        let revision = self.revision;
        if let Ok(mut writer) = writer.lock() {
            writer.requested = revision;
        }
        let executor = cx.background_executor().clone();
        self.pending_save = Some(executor.clone().spawn(async move {
            loop {
                match save(writer.clone(), revision, &json) {
                    Ok(()) => break,
                    Err(error) => eprintln!("Could not save agent attention; retrying: {error}"),
                }
                executor.timer(Duration::from_secs(1)).await;
            }
        }));
    }

    fn flush(&mut self) {
        let Some(writer) = self.writer.clone() else { return };
        let Ok(json) = serde_json::to_string(&self.model) else { return };
        self.revision += 1;
        if let Ok(mut writer) = writer.lock() {
            writer.requested = self.revision;
        }
        if let Err(error) = save(writer, self.revision, &json) {
            eprintln!("Could not flush agent attention: {error}");
        }
    }
}

fn save(writer: Arc<Mutex<Writer>>, revision: u64, json: &str) -> anyhow::Result<()> {
    let mut writer =
        writer.lock().map_err(|_| anyhow::anyhow!("attention writer lock poisoned"))?;
    if revision <= writer.revision || revision < writer.requested {
        return Ok(());
    }
    writer.store.save_attention(json)?;
    writer.revision = revision;
    Ok(())
}

fn tag(target: &Target) -> String {
    format!("chartr-attention:{}", serde_json::to_string(target).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failed_attention_write_can_retry_and_old_tasks_cannot_overwrite_flush() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("attention.sqlite");
        let store = StateStore::open(&path).unwrap();
        let control = rusqlite::Connection::open(&path).unwrap();
        control.execute_batch("CREATE TRIGGER reject_attention BEFORE INSERT ON app_state WHEN NEW.key = 'attention' BEGIN SELECT RAISE(FAIL, 'fixture'); END;").unwrap();
        let writer = Arc::new(Mutex::new(Writer { store, revision: 0, requested: 1 }));
        assert!(save(writer.clone(), 1, "first").is_err());
        assert_eq!(writer.lock().unwrap().revision, 0);
        control.execute_batch("DROP TRIGGER reject_attention").unwrap();
        save(writer.clone(), 1, "first").unwrap();
        writer.lock().unwrap().requested = 3;
        save(writer.clone(), 2, "stale").unwrap();
        save(writer.clone(), 3, "latest").unwrap();
        save(writer.clone(), 1, "delayed").unwrap();
        assert_eq!(
            writer.lock().unwrap().store.load_attention().unwrap().as_deref(),
            Some("latest")
        );
    }
}
