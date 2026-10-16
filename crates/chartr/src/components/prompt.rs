//! Linux has no native GPUI prompt. Keep confirmation details readable in the fallback.

use chartr_plugin::ui as plugin_ui;
use gpui::{EventEmitter, FocusHandle, Focusable, PromptButton, PromptResponse};
use ui::prelude::*;

pub fn init(cx: &mut App) {
    cx.set_prompt_builder(|_, message, detail, actions, handle, window, cx| {
        let view = cx.new(|cx| ConfirmationPrompt {
            message: message.to_owned(),
            detail: detail.map(str::to_owned),
            actions: actions.to_vec(),
            focus: cx.focus_handle(),
        });
        handle.with_view(view, window, cx)
    });
}

struct ConfirmationPrompt {
    message: String,
    detail: Option<String>,
    actions: Vec<PromptButton>,
    focus: FocusHandle,
}

impl EventEmitter<PromptResponse> for ConfirmationPrompt {}

impl Focusable for ConfirmationPrompt {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for ConfirmationPrompt {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let font = crate::fonts::Fonts::setup_ui(window, cx);
        let dialog = plugin_ui::DialogSurface::new("confirmation-prompt")
            .compact()
            .height_limit((window.viewport_size().height - px(32.)).max(px(0.)))
            .debug_selector("CONFIRMATION_PROMPT")
            .child(
                plugin_ui::dialog_body()
                    .id("confirmation-body")
                    .debug_selector(|| "CONFIRMATION_BODY".into())
                    .overflow_y_scroll()
                    .child(
                        div()
                            .w_full()
                            .flex_none()
                            .whitespace_normal()
                            .debug_selector(|| "CONFIRMATION_MESSAGE".into())
                            .child(self.message.clone()),
                    )
                    .children(self.detail.clone().map(|detail| {
                        div()
                            .w_full()
                            .flex_none()
                            .whitespace_normal()
                            .debug_selector(|| "CONFIRMATION_DETAIL".into())
                            .child(detail)
                    })),
            )
            .child(
                plugin_ui::dialog_actions(cx)
                    .flex_wrap()
                    .debug_selector(|| "CONFIRMATION_ACTIONS".into())
                    .children(self.actions.iter().enumerate().map(|(index, action)| {
                        div().debug_selector(move || format!("CONFIRMATION_ACTION_{index}")).child(
                            plugin_ui::form_button(
                                ("confirmation-action", index),
                                action.label().clone(),
                            )
                            .on_click(cx.listener(
                                move |_, _, _, cx| {
                                    cx.emit(PromptResponse(index));
                                    cx.stop_propagation();
                                },
                            )),
                        )
                    })),
            );
        div()
            .id("confirmation-overlay")
            .role(gpui::Role::Dialog)
            .aria_label(self.message.clone())
            .track_focus(&self.focus)
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .font(font)
            .text_size(crate::fonts::UI_TEXT_DEFAULT)
            .text_color(cx.theme().colors().text)
            .bg(gpui::black().opacity(0.5))
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, _, cx| {
                // Escape must never choose a destructive/default action when no cancel exists.
                if event.keystroke.key == "escape" {
                    if let Some(index) = this.actions.iter().position(PromptButton::is_cancel) {
                        cx.emit(PromptResponse(index));
                        cx.stop_propagation();
                    }
                }
            }))
            .child(dialog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{PromptLevel, TestAppContext, size};

    struct Harness;
    impl Render for Harness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full()
        }
    }

    #[gpui::test]
    async fn confirmation_wraps_details_and_keeps_actions_reachable(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ::settings::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            crate::fonts::install(&crate::settings::ResolvedSettings::default(), cx);
            init(cx);
        });
        let (_, cx) = cx.add_window_view(|_, _| Harness);
        cx.simulate_resize(size(px(360.), px(280.)));
        let response = cx.update(|window, cx| {
            window.prompt(
                PromptLevel::Warning,
                "Install Clock?",
                Some(&format!(
                    "Source: /home/verify/{}. Declared host access: project_files=none. {}",
                    "longdirectory".repeat(20),
                    "Review these permissions before installing. ".repeat(20),
                )),
                &["Install", "Cancel"],
                cx,
            )
        });
        cx.run_until_parked();
        let dialog = cx.debug_bounds("CONFIRMATION_PROMPT").unwrap();
        let body = cx.debug_bounds("CONFIRMATION_BODY").unwrap();
        let detail = cx.debug_bounds("CONFIRMATION_DETAIL").unwrap();
        let actions = cx.debug_bounds("CONFIRMATION_ACTIONS").unwrap();
        assert!(dialog.left() >= px(0.) && dialog.right() <= px(360.));
        assert!(dialog.top() >= px(0.) && dialog.bottom() <= px(280.));
        assert!(detail.size.height > body.size.height, "long details wrap into scrollable content");
        assert!(detail.left() >= body.left() && detail.right() <= body.right());
        assert_eq!(body.bottom(), actions.top());
        let cancel = cx.debug_bounds("CONFIRMATION_ACTION_1").unwrap();
        assert!(cancel.top() >= actions.top() && cancel.bottom() <= dialog.bottom());
        cx.simulate_click(cancel.center(), gpui::Modifiers::none());
        assert_eq!(response.await.unwrap(), 1, "button indices retain the caller's meaning");
        cx.run_until_parked();
        assert!(cx.debug_bounds("CONFIRMATION_PROMPT").is_none());

        let response = cx.update(|window, cx| {
            window.prompt(PromptLevel::Warning, "Remove Clock?", None, &["Remove", "Cancel"], cx)
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("escape");
        assert_eq!(response.await.unwrap(), 1, "Escape cancels instead of confirming removal");
    }
}
