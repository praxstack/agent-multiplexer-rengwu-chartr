//! App-wide macOS actions remain available even with no workspace window.

use gpui::{App, KeyBinding, Menu, MenuItem};

gpui::actions!(chartr_application, [Hide, HideOthers, ShowAll, Quit]);

pub fn init(cx: &mut App) {
    cx.on_action(|_: &Hide, cx| cx.hide());
    cx.on_action(|_: &HideOthers, cx| cx.hide_other_apps());
    cx.on_action(|_: &ShowAll, cx| cx.unhide_other_apps());
    // Use GPUI's normal quit path so persistence and session exit policy run.
    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.bind_keys([
        KeyBinding::new("cmd-h", Hide, None),
        KeyBinding::new("cmd-alt-h", HideOthers, None),
        KeyBinding::new("cmd-q", Quit, None),
    ]);
    cx.set_menus([Menu::new("chartr").items([
        MenuItem::action("Hide chartr", Hide),
        MenuItem::action("Hide Others", HideOthers),
        MenuItem::action("Show All", ShowAll),
        MenuItem::separator(),
        MenuItem::action("Quit chartr", Quit),
    ])]);
}
