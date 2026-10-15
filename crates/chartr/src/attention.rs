//! Best-effort unread attention, independent of focus discovery and I/O.

use std::collections::{BTreeMap, HashSet};

use chartr_herdr::control::{Session, SessionStatus};
use serde::{Deserialize, Serialize};

const VERSION: u32 = 2;

mod service;
pub(crate) use service::service;
pub use service::{acknowledge, connect, init, retry_navigation, unread};

/// A notification addresses one unread episode of one terminal lifetime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target {
    pub terminal_id: String,
    pub revision: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
enum Activity {
    Inactive,
    Working,
    Blocked,
    #[default]
    Unknown,
}

impl Activity {
    fn of(session: &Session) -> Self {
        if session.agent.as_deref().is_none_or(|agent| agent.trim().is_empty()) {
            return Self::Unknown;
        }
        match session.status {
            // Herdr's done/idle distinction reflects its own seen policy.
            SessionStatus::Idle | SessionStatus::Done => Self::Inactive,
            SessionStatus::Working => Self::Working,
            SessionStatus::Blocked => Self::Blocked,
            SessionStatus::Unknown => Self::Unknown,
        }
    }

    fn needs_attention_after(self, previous: Self) -> bool {
        self == Self::Blocked && previous != Self::Blocked
            || self == Self::Inactive && previous == Self::Working
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Tracked {
    pane_id: String,
    label: String,
    activity: Activity,
    unread: bool,
    revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    version: u32,
    terminals: BTreeMap<String, Tracked>,
}

impl Default for Model {
    fn default() -> Self {
        Self { version: VERSION, terminals: BTreeMap::new() }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Change {
    pub notify: Vec<Target>,
    pub changed: bool,
}

impl Model {
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        let value: serde_json::Value = serde_json::from_str(json)?;
        // Version 1 belonged to the unreleased strict-outcome prototype.
        if value["version"].as_u64() != Some(VERSION.into()) {
            return Ok(Self::default());
        }
        serde_json::from_value(value)
    }

    pub fn unread(&self, terminal: &str) -> bool {
        self.terminals.get(terminal).is_some_and(|tracked| tracked.unread)
    }

    pub fn target(&self, terminal: &str) -> Option<Target> {
        let tracked = self.terminals.get(terminal)?;
        tracked
            .unread
            .then(|| Target { terminal_id: terminal.to_owned(), revision: tracked.revision })
    }

    fn resolve(&self, target: &Target) -> Option<&Tracked> {
        self.terminals
            .get(&target.terminal_id)
            .filter(|tracked| tracked.unread && tracked.revision == target.revision)
    }

    pub fn acknowledge(&mut self, target: &Target) -> bool {
        if self.resolve(target).is_none() {
            return false;
        }
        self.terminals.get_mut(&target.terminal_id).unwrap().unread = false;
        true
    }

    /// Startup and the first successful read after a disconnect establish a
    /// silent baseline. Failed reads must not be passed here as an empty list.
    pub fn apply(
        &mut self,
        sessions: Vec<Session>,
        baseline: bool,
        focused: &HashSet<String>,
    ) -> Change {
        let present: HashSet<_> = sessions.iter().map(|s| s.terminal.0.as_str()).collect();
        let count = self.terminals.len();
        self.terminals.retain(|id, _| present.contains(id.as_str()));
        let mut change = Change { changed: count != self.terminals.len(), ..Change::default() };
        for session in sessions {
            let activity = Activity::of(&session);
            let id = session.terminal.0.clone();
            let previous = self.terminals.get(&id);
            let mut tracked = Tracked {
                pane_id: session.id.0.clone(),
                label: session.title().to_owned(),
                activity,
                unread: previous.is_some_and(|old| old.unread),
                revision: previous.map_or(0, |old| old.revision),
            };
            if focused.contains(&id) {
                tracked.unread = false;
            } else if !baseline
                && !tracked.unread
                && previous.is_some_and(|old| activity.needs_attention_after(old.activity))
            {
                tracked.unread = true;
                tracked.revision += 1;
                change.notify.push(Target { terminal_id: id.clone(), revision: tracked.revision });
            }
            if previous != Some(&tracked) {
                change.changed = true;
                self.terminals.insert(id, tracked);
            }
        }
        change
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chartr_herdr::{PaneId, TerminalId, WorkspaceId};

    fn session(status: SessionStatus) -> Session {
        Session {
            id: PaneId("pane".into()),
            terminal: TerminalId("terminal".into()),
            workspace: WorkspaceId("workspace".into()),
            label: "Terminal".into(),
            running: Some("Codex".into()),
            status,
            agent: Some("codex".into()),
            agent_session: None,
            conversation_title: None,
            foreground_pid: None,
            cwd: None,
        }
    }
    fn apply(model: &mut Model, status: SessionStatus) -> Change {
        model.apply(vec![session(status)], false, &HashSet::new())
    }

    #[test]
    fn notify_once_until_visited_then_rearm() {
        let mut model = Model::default();
        assert!(apply(&mut model, SessionStatus::Working).notify.is_empty());
        let first = apply(&mut model, SessionStatus::Done).notify.remove(0);
        assert!(model.unread("terminal"));
        assert!(!apply(&mut model, SessionStatus::Done).changed);
        assert!(apply(&mut model, SessionStatus::Idle).notify.is_empty());
        assert!(apply(&mut model, SessionStatus::Working).notify.is_empty());
        assert!(apply(&mut model, SessionStatus::Blocked).notify.is_empty());
        assert!(model.unread("terminal"));
        assert!(model.acknowledge(&first));
        assert!(apply(&mut model, SessionStatus::Blocked).notify.is_empty());
        apply(&mut model, SessionStatus::Working);
        let second = apply(&mut model, SessionStatus::Blocked).notify.remove(0);
        assert_ne!(first, second);
        assert!(!model.acknowledge(&first));
        assert!(model.resolve(&first).is_none());
        assert!(model.acknowledge(&second));
    }

    #[test]
    fn exact_foreground_focus_suppresses_and_acknowledges() {
        let mut model = Model::default();
        apply(&mut model, SessionStatus::Working);
        let other = HashSet::from(["adjacent-split".into()]);
        assert_eq!(
            model.apply(vec![session(SessionStatus::Blocked)], false, &other).notify.len(),
            1
        );
        let focus = HashSet::from(["terminal".into()]);
        model.apply(vec![session(SessionStatus::Blocked)], false, &focus);
        assert!(!model.unread("terminal"));
        model.apply(vec![session(SessionStatus::Working)], false, &focus);
        assert!(model.apply(vec![session(SessionStatus::Done)], false, &focus).notify.is_empty());
        assert!(!model.unread("terminal"));
    }

    #[test]
    fn restore_and_reconnect_keep_dots_without_catchup() {
        let mut model = Model::default();
        apply(&mut model, SessionStatus::Working);
        apply(&mut model, SessionStatus::Blocked);
        let mut restored = Model::from_json(&serde_json::to_string(&model).unwrap()).unwrap();
        let mut moved = session(SessionStatus::Working);
        moved.id = PaneId("moved-pane".into());
        assert!(restored.apply(vec![moved], true, &HashSet::new()).notify.is_empty());
        assert!(restored.unread("terminal"));
        assert_eq!(
            restored.resolve(&restored.target("terminal").unwrap()).unwrap().pane_id,
            "moved-pane"
        );
        restored.acknowledge(&restored.target("terminal").unwrap());
        assert!(
            restored
                .apply(vec![session(SessionStatus::Done)], true, &HashSet::new())
                .notify
                .is_empty()
        );
        assert!(!restored.unread("terminal"));
    }

    #[test]
    fn no_startup_shell_unknown_or_presentation_alerts() {
        let mut model = Model::default();
        assert!(apply(&mut model, SessionStatus::Blocked).notify.is_empty());
        let mut renamed = session(SessionStatus::Blocked);
        renamed.label = "Renamed".into();
        assert!(model.apply(vec![renamed], false, &HashSet::new()).notify.is_empty());
        apply(&mut model, SessionStatus::Working);
        apply(&mut model, SessionStatus::Unknown);
        assert!(apply(&mut model, SessionStatus::Done).notify.is_empty());
        apply(&mut model, SessionStatus::Working);
        let mut shell = session(SessionStatus::Done);
        shell.agent = None;
        assert!(model.apply(vec![shell], false, &HashSet::new()).notify.is_empty());
        assert!(!model.unread("terminal"));
    }

    #[test]
    fn closed_and_replaced_terminals_cannot_receive_old_clicks() {
        let mut model = Model::default();
        apply(&mut model, SessionStatus::Working);
        let old = apply(&mut model, SessionStatus::Done).notify.remove(0);
        let mut replacement = session(SessionStatus::Blocked);
        replacement.terminal = TerminalId("new-terminal".into());
        let change = model.apply(vec![replacement], false, &HashSet::new());
        assert!(change.notify.is_empty());
        assert!(!model.unread("terminal"));
        assert!(model.resolve(&old).is_none());
        assert!(model.apply(vec![], false, &HashSet::new()).changed);
    }
}
