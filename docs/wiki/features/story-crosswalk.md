---
title: Legacy story crosswalk
section: Features
updated: 2026-10-05
---
# Legacy story crosswalk

The archived [workspace rewrite specification](../archive/workspace-rewrite-spec.md#user-stories)
keeps its original 97 story numbers and text. This table maps each to current
feature records; it is a coverage index, not a claim that every story was executed
or that historical expectations override the current implementation. Cross-cutting
stories may link to several records.

| Story | Original intent | Current feature records |
| --- | --- | --- |
| 1 | every open tab to belong to one space | [Spaces and folder ownership](spaces.md) |
| 2 | every tab to belong to one pane | [Spaces and folder ownership](spaces.md) |
| 3 | one permanent Free sessions space | [Spaces and folder ownership](spaces.md) |
| 4 | Free sessions terminals to start in my home directory by default | [Spaces and folder ownership](spaces.md) |
| 5 | to configure the Free sessions working directory | [Spaces and folder ownership](spaces.md), [Settings, themes and keybindings](settings.md) |
| 6 | at most one space per canonical folder | [Spaces and folder ownership](spaces.md) |
| 7 | to rename a space's displayed label without changing its folder identity | [Spaces and folder ownership](spaces.md) |
| 8 | missing folders retained as unavailable spaces | [Spaces and folder ownership](spaces.md) |
| 9 | to locate a missing space folder | [Spaces and folder ownership](spaces.md) |
| 10 | removing a space to leave its folder untouched | [Spaces and folder ownership](spaces.md) |
| 11 | new sessions to open as standalone outer tabs in the targeted space | [Spaces and folder ownership](spaces.md) |
| 12 | an inactive space's add control to activate that space before creating its session | [Spaces and folder ownership](spaces.md) |
| 13 | nested horizontal and vertical splits | [Tabs, groups and pane layout](panes.md) |
| 14 | to resize split dividers | [Tabs, groups and pane layout](panes.md) |
| 15 | directional pane focus | [Tabs, groups and pane layout](panes.md) |
| 16 | to reorder tabs within a pane | [Tabs, groups and pane layout](panes.md) |
| 17 | to drag a tab between panes | [Tabs, groups and pane layout](panes.md) |
| 18 | to drop a tab on a pane edge to create a split | [Tabs, groups and pane layout](panes.md) |
| 19 | joining a pane to move its items into an adjacent pane | [Tabs, groups and pane layout](panes.md) |
| 20 | a split pane removed when its last item leaves, following Zed's default pane lifecycle | [Tabs, groups and pane layout](panes.md) |
| 21 | an emptied outer tab removed while the space remains usable through its New action | [Tabs, groups and pane layout](panes.md) |
| 22 | to zoom or maximize a pane | [Tabs, groups and pane layout](panes.md) |
| 23 | terminals never to be cloned or mirrored | [Tabs, groups and pane layout](panes.md), [Terminal interaction and search](terminals.md) |
| 24 | to declare whether my item supports cloning | [Tabs, groups and pane layout](panes.md) |
| 25 | pane layouts and split ratios restored after switching spaces | [Tabs, groups and pane layout](panes.md) |
| 26 | pane layouts and active items restored after relaunch | [Tabs, groups and pane layout](panes.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 27 | tabbed mode to show only the active space | [Views, chrome and navigation](workspace-views.md) |
| 28 | every non-empty workspace pane in either presentation mode to retain its own draggable tab bar | [Tabs, groups and pane layout](panes.md) |
| 29 | sidebar mode to show all spaces | [Views, chrome and navigation](workspace-views.md) |
| 30 | Spaces to be the initial view | [Views, chrome and navigation](workspace-views.md) |
| 31 | standalone tabs and any number of pane groups mixed in one space, with each group collapsed to one outer entry using its saved name or item count | [Tabs, groups and pane layout](panes.md) |
| 32 | only the active non-empty pane to expose compact Zed-style split and zoom controls while all pane tab bars remain visible | [Tabs, groups and pane layout](panes.md) |
| 33 | selecting an item in an inactive space to activate its space, pane, and item together | [Spaces and folder ownership](spaces.md) |
| 34 | the sidebar width and presentation modes persisted | [Views, chrome and navigation](workspace-views.md) |
| 35 | each top-level pane group to be closable | [Tabs, groups and pane layout](panes.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 36 | confirmation before an operation kills multiple sessions | [Tabs, groups and pane layout](panes.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 37 | closing one terminal tab to terminate its Herdr session immediately | [Terminal interaction and search](terminals.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 38 | `Cmd+W` on macOS and `Ctrl+W` on Linux to close the active tab | [Terminal interaction and search](terminals.md) |
| 39 | a closed session's explicitly bound plugin items to close too | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 40 | closing a plugin tab to destroy that view instance | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 41 | normal application exit to detach sessions | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 42 | a setting that can terminate sessions on application exit | [Session lifecycle, recovery and persistence](session-lifecycle.md), [Settings, themes and keybindings](settings.md) |
| 43 | removing a space to terminate everything it owns after confirmation | [Spaces and folder ownership](spaces.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 44 | closing Settings with `Cmd/Ctrl+W` to return to my previous item | [Settings, themes and keybindings](settings.md) |
| 45 | plugin contributions to be globally discoverable but opened instances to remain space-owned | [Plugin discovery and installation](plugin-packages.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 46 | separate plugin instances in different spaces when supported | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 47 | to declare singleton or multi-instance behavior | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 48 | a per-space singleton plugin to focus its existing pane when reopened | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 49 | a stable owning-space context | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 50 | to bind explicitly to one session when required | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 51 | restorable plugin items to return after relaunch | [Session lifecycle, recovery and persistence](session-lifecycle.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 52 | unrestorable plugin items omitted with a summary | [Session lifecycle, recovery and persistence](session-lifecycle.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 53 | to contribute a lazy settings page | [Settings, themes and keybindings](settings.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 54 | native plugins labeled as fully trusted code | [Plugin discovery and installation](plugin-packages.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 55 | web plugin permissions visible before enablement | [Plugin discovery and installation](plugin-packages.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 56 | declared read/write access within my owning project's folder | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 57 | plugin-specific data storage | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 58 | folderless-space web plugins restricted to plugin data in safe mode | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 59 | to grant unrestricted filesystem access to one web plugin through unsafe mode | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 60 | no global unsafe switch | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 61 | declared host actions for network and process access | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 62 | declared access to metadata and terminal input for only my bound session | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 63 | permission revocation to close live plugin instances and revoke their broker | [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 64 | native and web plugin panes both to work | [Plugin discovery and installation](plugin-packages.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 65 | disabling a plugin to remove its contributions and prevent future loading | [Plugin discovery and installation](plugin-packages.md), [Plugin surfaces, services and permissions](plugin-runtime.md) |
| 66 | one shared native Settings window | [Settings, themes and keybindings](settings.md) |
| 67 | General, Appearance, Terminal, Hotkeys, and Plugins settings pages | [Settings, themes and keybindings](settings.md) |
| 68 | Settings to show only implemented controls | [Settings, themes and keybindings](settings.md) |
| 69 | settings changes applied immediately where safe | [Settings, themes and keybindings](settings.md) |
| 70 | settings updates written atomically | [Settings, themes and keybindings](settings.md) |
| 71 | keyboard shortcuts editable in the ordinary Hotkeys page | [Settings, themes and keybindings](settings.md) |
| 72 | contextual shortcut conflict detection | [Settings, themes and keybindings](settings.md) |
| 73 | a command palette exposing workspace and pane actions | [Views, chrome and navigation](workspace-views.md) |
| 74 | `chartr Dark` as the fixed initial theme | [Settings, themes and keybindings](settings.md) |
| 75 | `chartr Light`, fixed theme, and light/dark/system theme-pair options | [Settings, themes and keybindings](settings.md) |
| 76 | user theme files loaded and refreshed | [Settings, themes and keybindings](settings.md) |
| 77 | IBM Plex Sans and IBM Plex Mono as configurable defaults | [Settings, themes and keybindings](settings.md) |
| 78 | semantic theme colors and Zed UI components everywhere | [Settings, themes and keybindings](settings.md) |
| 79 | every drag operation to have an action-based alternative | [Tabs, groups and pane layout](panes.md) |
| 80 | reliable focus order, focus restoration, labels, contrast, and reduced-motion behavior | [Views, chrome and navigation](workspace-views.md), [Settings, themes and keybindings](settings.md) |
| 81 | an affected terminal to show a clear state when its Herdr attach client closes | [Terminal interaction and search](terminals.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 82 | reattachment offered only when Herdr confirms the same session exists | [Terminal interaction and search](terminals.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 83 | chartr to restart its private Herdr once after unexpected death | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 84 | repeated backend death to become a stable crash-loop state with Retry | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 85 | surviving space layouts and space-bound plugins retained after backend loss | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 86 | Herdr's live session list to override stale local terminal records | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 87 | orphaned live Herdr sessions adopted into their owning space | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 88 | a fresh installation to open the empty Free sessions space without spawning a terminal | [Spaces and folder ownership](spaces.md) |
| 89 | window geometry, pane ratios, expansion state, selection, and chrome restored | [Views, chrome and navigation](workspace-views.md), [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 90 | configuration, state, and runtime data under the `chartr` namespace | [Session lifecycle, recovery and persistence](session-lifecycle.md) |
| 91 | to drag-sort every sidebar space, including Free sessions and recovered folders | [Spaces and folder ownership](spaces.md) |
| 92 | Reduce Motion to disable space-sort settling without disabling direct manipulation | [Spaces and folder ownership](spaces.md), [Settings, themes and keybindings](settings.md) |
| 93 | Zed's wheel and trackpad behavior and terminal-owned scrollback | [Terminal interaction and search](terminals.md) |
| 94 | Chats to show searchable Inbox/Archive history beside the original terminal | [Agent launching and Inbox history](agents-inbox.md) |
| 95 | Skill sources and Saved Prompts managed in plugin settings | [Skill source registry and precedence](skill-sources.md), [Saved prompt library](saved-prompts.md) |
| 96 | Markdown Prompt to change project files only on explicit Save | [Markdown composition and explicit saves](markdown-prompt.md) |
| 97 | to release an abandoned ticket claim with confirmation | [Wayfinder maps, launch and claim recovery](wayfinder.md) |
