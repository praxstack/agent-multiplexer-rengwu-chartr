# Agent attention

An accent dot means an agent may need your attention. Chartr uses Herdr's existing
agent detection: a transition from working to inactive, or into a blocked state,
can mark an unfocused terminal unread. This is best effort, not confirmation of
success or failure. Cancellation or unfamiliar screens may also cause an alert.

Dots appear on the terminal and its enclosing group. Focusing that terminal in
Chartr's active window clears its dot. Showing a split or opening a group does
not acknowledge its other terminals.

Chartr requests at most one desktop notification per terminal until you visit it,
including when you are working in another Chartr tab. A later state transition
can notify again after that visit. Repeated updates while unread do not notify.
Clicking a current notification opens its originating terminal.

Chartr must remain running to monitor terminals, including when hidden, minimized,
or without an open window. Unread dots survive resumed work, app relaunch against
a surviving backend, and live backend handoff. Relaunch and reconnect establish a
silent baseline; they do not replay notifications. Closed terminals and fresh
backend terminals do not inherit old dots.

## Agent support and limits

Attention uses the same upstream detection as Chartr's activity indicators,
including Codex. It does not require a new Chartr-specific agent integration.
Coverage varies by agent and installed version; this does not promise support
for every CLI. Ordinary shells and build/test commands do not trigger attention.

Chartr reads activity about once a second. Short transitions between reads, work
during disconnection, and repeated requests that stay blocked can be missed.
Unknown states do not imply completion. Detection can mistake cancellation or an
unfamiliar screen for inactivity, so notifications use neutral wording.

## Desktop delivery

Allow notifications for Chartr in your operating system. macOS requires an app
bundle; a bare `cargo run` binary cannot send desktop notifications. Focus/Do Not
Disturb and OS notification settings can suppress banners. The unread dots remain
available independently of desktop delivery.
