# Zed platform patches

These crates are vendored from Zed revision
`1ea16c1ab9dd6d36649e002dc60995634da04daf`.

chartr patches the native platform and presentation seams:

- `gpui_macos` implements `WindowKind::AnchoredPopup` as a borderless AppKit
  child panel, including parent-relative placement and focus-loss dismissal.
  Active-window lookup uses AppKit's key window (including panels), so focus
  checks reflect the window receiving input rather than the main workspace.
  Unfocused anchored popups without a grab are passive panels: they cannot
  become key or main and ignore mouse events, including in transparent margins.
  On macOS 14+, normal frames use `CAMetalDisplayLink`'s supplied drawable;
  the main thread does not acquire another drawable in that callback. Temporary
  visibility stops pause the link and destruction invalidates it exactly once. Effective
  Low Power Mode requests at most 60 Hz, bounded by the display's maximum.
  Older macOS versions retain the CoreVideo source. AppKit transaction draws
  retain their existing synchronous behavior by invalidating the layer's link
  before direct acquisition and recreating it afterward. Merely pausing the link
  does not permit `nextDrawable` and raises an Objective-C exception.
- `gpui_apple` exposes `draw_with_drawable` by extracting the pinned renderer's
  encoding/presentation body. The legacy `draw` entry still acquires a drawable
  and delegates to it. Shaders, scene encoding and presentation branches remain
  those of the pinned revision; this is not a renderer upgrade.
- `gpui_linux` implements the same contract for the X11 backend. Zed's
  Wayland backend already supports anchored popups. Its XIM callback also masks
  X11's synthetic-event flag so keys forwarded by embedded plugins are not
  dropped when the input method returns them to the application.
- `ui` lets a content-sized popup override `ContextMenu`'s normal 75%-of-window
  maximum height and hosts button tooltips in passive native child windows so
  they can appear above webviews. Tooltip creation is cancelled on dismissal
  and checked against the active parent window; existing tooltips close when
  their parent loses focus, preventing them from raising a background workspace.
  Scrollbars also support container-hover visibility that retains input handling
  during thumb drags, plus surface-specific thumb colors for sidebar contrast.
  Fixed-width context menus size the bordered surface, keeping their item
  highlights equally inset on both sides of a native popup.

The root Cargo patch table selects these crates, so builds do not depend on a
modified Cargo checkout.

On a macOS desktop, `cargo run --manifest-path
vendor/zed-platform/gpui_macos/Cargo.toml --example passive_popup` checks native
mouse and focus eligibility for passive tooltips, interactive popups, and normal
windows. This covers AppKit behavior that GPUI's simulated window tests cannot.
With the renderer patch, standalone platform builds also need the root Cargo
patches (in particular the local `gpui_apple`), because an excluded crate's build
does not inherit them.

`sh scripts/check-metal-display-link.sh` runs the production frame-source policy
test and a small native lifecycle check without rebuilding GPUI's test graph.
It requires macOS 14+ and a desktop session, opens one temporary window, and checks
unused updates, submission after idle, pause, resize/resume, repeated synchronous
transactions followed by link recreation, no callbacks after
destruction, and destruction from inside a callback. Its temporary manifest uses
the application's locked dependency versions offline. This does not replace
performance sampling or interaction checks against Chartr itself.

The original Apache 2.0 and GPLv3 license texts are retained beside the
vendored crates.
