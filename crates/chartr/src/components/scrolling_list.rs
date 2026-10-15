//! Shared scrolling treatment for Spaces and chat history.

use gpui::{
    Bounds, BoxShadow, ContentMask, ScrollHandle, canvas, fill, linear_color_stop, linear_gradient,
    point, relative, size,
};
use ui::{ScrollAxes, Scrollbars, WithScrollbar, prelude::*};

pub(crate) fn scrollbar_thumb_colors(colors: &theme::ThemeColors) -> [gpui::Hsla; 3] {
    // Light palettes need a little more opacity to keep the overlay readable.
    let opacity =
        if colors.panel_background.l > 0.5 { [0.68, 0.78, 0.85] } else { [0.55, 0.7, 0.8] };
    opacity.map(|alpha| colors.text.alpha(alpha))
}

/// A tracked list with the shared top fade and hover-only vertical scrollbar.
pub(crate) fn scrolling_list(
    scrollbar_id: impl Into<gpui::ElementId>,
    content: gpui::Stateful<gpui::Div>,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let colors = cx.theme().colors();
    let [thumb, hovered_thumb, active_thumb] = scrollbar_thumb_colors(colors);
    let scroll_background = colors.panel_background;
    let fade_handle = scroll_handle.clone();
    v_flex()
        .relative()
        .flex_1()
        .min_h_0()
        .w_full()
        .child(content.flex_1().min_h_0().overflow_y_scroll().track_scroll(scroll_handle))
        .child(
            // Read the clamped offset at paint time. This passive overlay
            // adds no hitbox, layout shift, timer, or scroll subscription.
            canvas(
                |_, _, _| {},
                move |bounds, _, window, _| {
                    let strength = (-fade_handle.offset().y / px(12.)).clamp(0., 1.);
                    if strength == 0. {
                        return;
                    }
                    window.with_content_mask(Some(ContentMask { bounds }), |window| {
                        // A small, theme-colored blurred veil gives the
                        // edge a frosted appearance. GPUI has no element
                        // backdrop blur; this needs no offscreen capture.
                        let veil = Bounds::new(
                            point(bounds.left(), bounds.top() - px(8.)),
                            size(bounds.size.width, px(8.)),
                        );
                        window.paint_drop_shadows(
                            veil,
                            Default::default(),
                            &[BoxShadow::new(
                                px(0.),
                                px(4.),
                                scroll_background.opacity(0.35 * strength),
                            )
                            .blur_radius(px(12.))],
                        );
                        window.paint_quad(fill(
                            bounds,
                            linear_gradient(
                                180.,
                                linear_color_stop(scroll_background.opacity(strength), 0.),
                                linear_color_stop(scroll_background.opacity(0.), 1.),
                            ),
                        ));
                    });
                },
            )
            .absolute()
            .top_0()
            .left_0()
            .w_full()
            .h(px(28.))
            .max_h(relative(1.)),
        )
        .custom_scrollbars(
            Scrollbars::on_hover(ScrollAxes::Vertical)
                .id(scrollbar_id)
                .thumb_colors(thumb, hovered_thumb, active_thumb)
                .tracked_scroll_handle(scroll_handle)
                .notify_content(),
            window,
            cx,
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Context, Modifiers, MouseButton, Render, TestAppContext};
    use std::time::Duration;

    struct Harness {
        handle: ScrollHandle,
    }

    struct HistoryHarness {
        handle: ScrollHandle,
        rows: usize,
    }

    impl Render for HistoryHarness {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            v_flex().size(px(200.)).child(scrolling_list(
                "history-test-scrollbar",
                crate::components::selection_list()
                    .id("history-test-list")
                    .px_1p5()
                    .when(self.rows > 0, |list| list.pb_2())
                    .children((0..self.rows).map(|_| div().h(px(30.)).flex_none())),
                &self.handle,
                window,
                cx,
            ))
        }
    }

    #[gpui::test]
    fn empty_and_fitting_history_has_no_scrollbar_even_on_hover(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ::settings::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
        let handle = ScrollHandle::new();
        let (view, cx) =
            cx.add_window_view(|_, _| HistoryHarness { handle: handle.clone(), rows: 0 });
        cx.simulate_mouse_move(point(px(100.), px(100.)), None, Modifiers::none());
        for rows in [0, 1, 20, 0] {
            view.update(cx, |view, cx| {
                view.rows = rows;
                cx.notify();
            });
            cx.run_until_parked();
            let overflow = rows == 20;
            assert_eq!(
                handle.max_offset().y > px(0.),
                overflow,
                "rows={rows}, offset={:?}",
                handle.max_offset()
            );
            let has_thumb = cx.update(|window, cx| {
                let [normal, hover, active] = scrollbar_thumb_colors(cx.theme().colors());
                window.painted_quads().iter().any(|quad| {
                    [normal, hover, active].iter().any(|color| quad.background == (*color).into())
                })
            });
            assert_eq!(has_thumb, overflow, "scrollbar visibility for {rows} rows");
            if overflow {
                handle.set_offset(point(px(0.), px(-50.)));
            } else {
                assert_eq!(handle.offset().y, px(0.), "emptying the list clears its scroll offset");
            }
        }
    }

    impl Render for Harness {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            v_flex().size(px(200.)).child(scrolling_list(
                "test-scrollbar",
                div().id("test-list").child(div().h(px(1000.)).w_full()),
                &self.handle,
                window,
                cx,
            ))
        }
    }

    #[gpui::test]
    fn scrollbar_fades_after_exit_and_reappears_on_reentry(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ::settings::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
        let handle = ScrollHandle::new();
        let (_, cx) = cx.add_window_view(|_, _| Harness { handle: handle.clone() });
        let thumb_alpha = |cx: &mut gpui::VisualTestContext| {
            cx.update(|window, _| {
                window.painted_quads().iter().find_map(|quad| {
                    (quad.bounds.size.width.as_f32() == 6. * window.scale_factor())
                        .then(|| quad.background.as_solid().unwrap().a)
                })
            })
        };
        let advance = |cx: &mut gpui::VisualTestContext, millis| {
            cx.executor().advance_clock(Duration::from_millis(millis));
            cx.update(|window, _| window.refresh());
            cx.run_until_parked();
        };
        let inside = point(px(100.), px(100.));
        let outside = point(px(250.), px(250.));
        cx.simulate_mouse_move(outside, None, Modifiers::none());
        advance(cx, 900);
        assert_eq!(thumb_alpha(cx), None);
        cx.simulate_mouse_move(inside, None, Modifiers::none());
        let visible_alpha = thumb_alpha(cx).unwrap();
        let configured_alpha = cx.update(|_, cx| scrollbar_thumb_colors(cx.theme().colors())[0].a);
        assert_eq!(visible_alpha, configured_alpha, "paint must preserve the custom thumb's alpha");
        assert!(visible_alpha > 0. && visible_alpha < 1.);
        advance(cx, 2000);
        assert_eq!(thumb_alpha(cx), Some(visible_alpha), "hover keeps the thumb visible");
        cx.simulate_mouse_move(outside, None, Modifiers::none());
        assert_eq!(thumb_alpha(cx), Some(visible_alpha), "exit starts at the visible alpha");
        advance(cx, 400);
        let halfway_alpha = thumb_alpha(cx).unwrap();
        assert!(halfway_alpha > 0. && halfway_alpha < visible_alpha);

        // Scroll/layout changes must not restart a fade while the pointer is away.
        handle.set_offset(point(px(0.), px(-50.)));
        advance(cx, 100);
        assert!(thumb_alpha(cx).unwrap() < halfway_alpha);
        cx.simulate_mouse_move(inside, None, Modifiers::none());
        assert_eq!(thumb_alpha(cx), Some(visible_alpha), "re-entry cancels the fade");
        advance(cx, 2000);
        assert_eq!(thumb_alpha(cx), Some(visible_alpha));
        cx.simulate_mouse_move(outside, None, Modifiers::none());
        advance(cx, 900);
        assert_eq!(thumb_alpha(cx), None);
        handle.set_offset(point(px(0.), px(-100.)));
        advance(cx, 100);
        assert_eq!(thumb_alpha(cx), None, "programmatic scrolling outside stays hidden");

        cx.simulate_mouse_move(inside, None, Modifiers::none());
        assert_eq!(thumb_alpha(cx), Some(visible_alpha));
        cx.update(|_, cx| cx.set_reduce_motion(true));
        cx.simulate_mouse_move(outside, None, Modifiers::none());
        assert_eq!(thumb_alpha(cx), None, "reduced motion skips the exit animation");
    }

    #[gpui::test]
    fn fade_tracks_scroll_and_hover_thumb_remains_draggable(cx: &mut TestAppContext) {
        cx.update(|cx| {
            ::settings::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
        let handle = ScrollHandle::new();
        let (_, cx) = cx.add_window_view(|_, _| Harness { handle: handle.clone() });
        let outside = point(px(250.), px(250.));
        cx.simulate_mouse_move(outside, None, Modifiers::none());
        cx.executor().advance_clock(Duration::from_millis(900));
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
        let resting = cx.update(|window, _| window.painted_quads().len());
        handle.set_offset(point(px(0.), px(-50.)));
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
        let faded = cx.update(|window, _| window.painted_quads().len());
        assert_eq!(faded, resting + 1, "scrolling paints the top fade");

        let thumb = point(px(193.), px(25.));
        cx.simulate_mouse_move(thumb, None, Modifiers::none());
        assert_eq!(cx.update(|window, _| window.painted_quads().len()), faded + 1);
        let before = handle.offset().y;
        cx.simulate_mouse_down(thumb, MouseButton::Left, Modifiers::none());
        cx.simulate_mouse_move(outside, MouseButton::Left, Modifiers::none());
        assert!(handle.offset().y < before, "the fade must not intercept thumb dragging");
        cx.simulate_mouse_up(outside, MouseButton::Left, Modifiers::none());
        assert_eq!(cx.update(|window, _| window.painted_quads().len()), faded + 1);
        cx.executor().advance_clock(Duration::from_millis(900));
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
        assert_eq!(cx.update(|window, _| window.painted_quads().len()), faded);

        handle.set_offset(point(px(0.), px(0.)));
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
        assert_eq!(cx.update(|window, _| window.painted_quads().len()), resting);
    }
}
