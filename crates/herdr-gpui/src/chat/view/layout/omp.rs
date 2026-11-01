//! Native GPUI rendering of OMP's editorial transcript surface.
//!
//! This ports the visual contract of OMP's GUI without embedding its Electron
//! renderer: soft right-aligned prompts, quiet agent prose, and one rounded
//! composer surface with controls below it.

use super::{Bubble, ChatLayout, ComposerParts, Look, Row, bubble_row, summary_row};
use crate::config::mix;
use gpui::{prelude::*, *};

const SHAPE: Bubble = Bubble {
    radius: 20.,
    padding: 11.,
    gap: 16.,
};

pub(super) struct Omp;

impl ChatLayout for Omp {
    fn row(&self, row: Row<'_>, look: &Look<'_>) -> AnyElement {
        if row.side().is_none() {
            return summary_row(row, look, SHAPE.gap);
        }
        bubble_row(row, look, SHAPE)
    }

    fn composer(&self, parts: ComposerParts, look: &Look<'_>) -> Div {
        let theme = look.theme;
        let field = div()
            .flex()
            .flex_col()
            .rounded(px(24.))
            .border_1()
            .border_color(rgb(mix(theme.background, theme.foreground, 18)))
            .bg(rgb(mix(theme.background, theme.foreground, 2)))
            .children(parts.chips)
            .child(
                div()
                    .flex()
                    .items_end()
                    .gap(px(8.))
                    .pl(px(16.))
                    .pr(px(8.))
                    .py(px(10.))
                    .child(parts.draft.flex_1().min_w_0().py(px(2.)))
                    .child(parts.send),
            );
        div().flex().flex_col().gap(px(6.)).child(field).child(
            div()
                .debug_selector(|| "chat-settings".into())
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(6.))
                .px(px(4.))
                .child(parts.attach)
                .children(parts.settings)
                .child(div().flex_1())
                .children(parts.usage),
        )
    }

    fn tool_preview(&self) -> bool {
        false
    }

    fn prompt_text(&self, look: &Look<'_>) -> f32 {
        (look.text - 0.5).max(10.)
    }

    fn prose(&self, look: &Look<'_>) -> Option<u32> {
        Some(look.theme.foreground)
    }

    fn backdrop(&self, _look: &Look<'_>) -> Option<u32> {
        None
    }

    fn turn(&self, text: AnyElement, look: &Look<'_>) -> Div {
        div()
            .debug_selector(|| "chat-turn".into())
            .flex()
            .items_center()
            .gap(px(8.))
            .text_size(px((look.text - 2.).max(10.)))
            .text_color(rgb(look.theme.muted))
            .child(div().flex_1().h(px(1.)).bg(rgb(look.theme.active)))
            .child(div().flex_none().child(text))
            .child(div().flex_1().h(px(1.)).bg(rgb(look.theme.active)))
    }

    fn copy(
        &self,
        button: Stateful<Div>,
        _placement: super::super::copy::CopyPlacement,
        _look: &Look<'_>,
    ) -> Div {
        div().h(px(18.)).flex().justify_end().child(button)
    }
}
