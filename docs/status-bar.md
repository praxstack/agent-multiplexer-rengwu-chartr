# Workspace status bar

The bottom bar is hidden by default in shipped builds. Enable it from
Settings → General → Show status bar or the “Workspace: Toggle status bar”
command to see persistent plugin services. Mobile Companion is excluded
from the current app build, so its sharing status and mobile session count are
absent.

Click a service to open its settings. Right-click the bar and choose Hide Status Bar; restore it from Settings → General → Show status bar or the “Workspace: Toggle status bar” command. Visibility is saved globally as `[general] show_status_bar` and hiding it does not stop services.

Bundled native plugins can contribute a status with `Plugin::background_status`. The host refreshes these once per second and redraws only when their reported values change. See [plugin documentation](plugins.md#background-status).

## Development verification

See the [status-bar feature record](wiki/features/status-bar.md) for the recipe
and coverage limits, and the [release matrix](wiki/verification/acceptance.md)
for broader acceptance. Historical Companion counts do not establish current
status-bar behavior.
