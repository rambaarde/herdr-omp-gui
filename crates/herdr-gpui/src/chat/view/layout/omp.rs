//! Native GPUI rendering of OMP's timeline surface.
//!
//! OMP's reference GUI uses Herdr's timeline treatment: a narrow reading
//! column, speaker avatars, a continuous gutter, and a quiet composer card.
//! Keep this adapter separate so OMP can own that treatment without changing
//! other agents' configured layouts.

use super::{ChatLayout, ComposerParts, Look, Row, timeline};
use gpui::*;

pub(super) struct Omp;

impl ChatLayout for Omp {
    fn row(&self, row: Row<'_>, look: &Look<'_>) -> AnyElement {
        timeline::Timeline.row(row, look)
    }

    fn composer(&self, parts: ComposerParts, look: &Look<'_>) -> Div {
        timeline::Timeline.composer(parts, look)
    }

    fn tool_preview(&self) -> bool {
        false
    }
}
