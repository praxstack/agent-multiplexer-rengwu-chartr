//! Notification presentation. No terminal scraping or attention detection.

use chartr_conversations::{NativeSession, Provider, ProviderPaths};
use chartr_herdr::control::Session;
use unicode_segmentation::UnicodeSegmentation as _;

use crate::persistence::{PersistedItem, Snapshot};

pub(super) const FALLBACK_BODY: &str = "Agent may need your attention";
const PREVIEW_LENGTH: usize = 160;

pub(super) fn saved_space(snapshot: &Snapshot, session: &Session) -> Option<String> {
    snapshot.spaces.iter().find_map(|space| {
        space
            .items
            .iter()
            .any(|item| matches!(item, PersistedItem::Terminal { backend_id, .. } if backend_id == &session.id.0))
            .then(|| space.name.clone())
    })
}

pub(super) fn title(session: &Session, space: Option<&str>) -> String {
    let space = space
        .map(str::to_owned)
        .or_else(|| session.cwd.as_deref().map(crate::spaces::display_name));
    let agent = session.agent.as_deref().and_then(Provider::detect);
    let label = first_line(agent.map(Provider::name).unwrap_or_else(|| session.title()), 60)
        .unwrap_or_else(|| "Agent".into());
    match space.and_then(|name| first_line(&name, 80)) {
        Some(space) => format!("{space} · {label}"),
        None => label,
    }
}

pub(super) fn preview(session: &Session, paths: &ProviderPaths) -> Option<String> {
    let provider = Provider::detect(session.agent.as_deref()?)?;
    let identity = session.agent_session.as_ref()?;
    if Provider::detect(&identity.agent) != Some(provider) {
        return None;
    }
    let native = NativeSession::from_identity(provider, &identity.kind, &identity.value)?;
    let reply = paths.latest_reply(provider, &native).ok()??;
    first_line(&reply, PREVIEW_LENGTH)
}

fn first_line(text: &str, limit: usize) -> Option<String> {
    let line = text.lines().map(str::trim).find(|line| !line.is_empty())?;
    let clean: String = line.chars().filter(|ch| !ch.is_control() || *ch == '\t').collect();
    let clean = clean.split_whitespace().collect::<Vec<_>>().join(" ");
    if clean.is_empty() {
        return None;
    }
    let mut graphemes = clean.graphemes(true);
    let mut shortened: String = graphemes.by_ref().take(limit).collect();
    if graphemes.next().is_some() {
        shortened = shortened.trim_end().to_owned();
        shortened.push('…');
    }
    Some(shortened)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::{PersistedSpace, SpaceKind};
    use chartr_herdr::control::SessionStatus;

    #[test]
    fn preview_is_one_line_and_does_not_split_unicode_graphemes() {
        assert_eq!(
            first_line("\n  Fixed\tthe issue.\r\nDetails", 160).as_deref(),
            Some("Fixed the issue.")
        );
        assert_eq!(first_line("👩🏽‍💻e\u{301}中🙂", 3).as_deref(), Some("👩🏽‍💻e\u{301}中…"));
        assert_eq!(first_line("exact", 5).as_deref(), Some("exact"));
        assert_eq!(first_line(" \n\t", 160), None);
    }

    #[test]
    fn title_prefers_the_owning_space_over_the_current_directory() {
        let mut session = super::super::tests::session(SessionStatus::Done);
        session.cwd = Some("/work/another-directory".into());
        assert_eq!(title(&session, Some("My project")), "My project · Codex");
        assert_eq!(title(&session, Some("Free sessions")), "Free sessions · Codex");
        assert_eq!(title(&session, None), "another-directory · Codex");
        session.cwd = None;
        assert_eq!(title(&session, None), "Codex");
    }

    #[test]
    fn closed_window_context_uses_persisted_terminal_ownership() {
        let session = super::super::tests::session(SessionStatus::Done);
        let mut snapshot = Snapshot::default();
        snapshot.spaces.push(PersistedSpace {
            key: "folder".into(),
            name: "Saved project name".into(),
            path: Some("/work/project".into()),
            kind: SpaceKind::Folder,
            layout: Default::default(),
            items: vec![PersistedItem::Terminal { item_id: 1, backend_id: session.id.0.clone() }],
            expanded: true,
        });
        assert_eq!(saved_space(&snapshot, &session).as_deref(), Some("Saved project name"));
        snapshot.spaces[0].items.clear();
        assert_eq!(saved_space(&snapshot, &session), None);
    }

    #[test]
    fn preview_reads_only_the_identified_session_and_falls_back_on_missing_history() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("2026-10-19_session-a.jsonl");
        let paths = ProviderPaths {
            codex: root.path().into(),
            claude: root.path().into(),
            opencode: root.path().into(),
            pi: root.path().into(),
            kimi: root.path().into(),
            grok: root.path().into(),
            omp: root.path().into(),
            cursor: root.path().into(),
            antigravity: root.path().into(),
            antigravity_cli: root.path().into(),
        };
        let mut session = super::super::tests::session(SessionStatus::Done);
        session.agent = Some("pi".into());
        session.agent_session = Some(chartr_herdr::protocol::AgentSession {
            source: "fixture".into(),
            agent: "pi".into(),
            kind: "path".into(),
            value: file.to_string_lossy().into_owned(),
        });
        let log = concat!(
            "{\"type\":\"session\",\"id\":\"session-a\"}\n",
            "{\"type\":\"message\",\"id\":\"u1\",\"message\":{\"role\":\"user\",\"content\":\"Fix it\"}}\n",
            "{\"type\":\"message\",\"id\":\"a1\",\"message\":{\"role\":\"assistant\",\"content\":\"Fixed the issue.\\nMore details.\"}}\n",
        );
        std::fs::write(&file, log).unwrap();
        assert_eq!(preview(&session, &paths).as_deref(), Some("Fixed the issue."));
        std::fs::write(&file, format!("{log}{{\"type\":\"message\",\"id\":\"u2\",\"message\":{{\"role\":\"user\",\"content\":\"Next task\"}}}}\n")).unwrap();
        assert_eq!(preview(&session, &paths), None);
        std::fs::write(&file, log.replace("session-a", "different-session")).unwrap();
        assert_eq!(preview(&session, &paths), None);
        std::fs::write(&file, log).unwrap();
        session.agent = Some("codex".into());
        assert_eq!(preview(&session, &paths), None);
        session.agent = Some("pi".into());
        std::fs::remove_file(file).unwrap();
        assert_eq!(preview(&session, &paths), None);
        session.agent_session = None;
        assert_eq!(preview(&session, &paths), None);
    }
}
