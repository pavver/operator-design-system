//! Zoom, pan, and gesture state management for GalleryViewer.

use leptos::prelude::*;

/// Encapsulates zoom scale, panning translation, and interactive gesture handling.
#[derive(Clone, Copy)]
pub struct ViewerZoomState {
    pub scale: RwSignal<f32>,
    pub offset_x: RwSignal<f32>,
    pub offset_y: RwSignal<f32>,
}

impl ViewerZoomState {
    pub fn new() -> Self {
        Self {
            scale: RwSignal::new(1.0),
            offset_x: RwSignal::new(0.0),
            offset_y: RwSignal::new(0.0),
        }
    }

    /// Resets scale and translation to identity.
    pub fn reset(&self) {
        self.scale.set(1.0);
        self.offset_x.set(0.0);
        self.offset_y.set(0.0);
    }

    /// True if the current media is zoomed beyond identity.
    pub fn is_zoomed(&self) -> bool {
        self.scale.get_untracked() > 1.02
    }

    /// Adjusts scale by a zoom factor clamped between 1.0x and 5.0x.
    pub fn zoom_by(&self, factor: f32) {
        let current = self.scale.get_untracked();
        let new_scale = (current * factor).clamp(1.0, 5.0);
        self.scale.set(new_scale);
        if new_scale <= 1.02 {
            self.offset_x.set(0.0);
            self.offset_y.set(0.0);
        }
    }

    /// Pans the viewport when zoomed.
    pub fn pan_by(&self, dx: f32, dy: f32) {
        if self.is_zoomed() {
            self.offset_x.update(|x| *x += dx);
            self.offset_y.update(|y| *y += dy);
        }
    }

    /// Double-tap toggle: zooms to 3.0x on tap or resets to 1.0x if already zoomed.
    pub fn toggle_double_tap(&self) {
        if self.is_zoomed() {
            self.reset();
        } else {
            self.scale.set(3.0);
        }
    }

    /// Computes the CSS transform string for the media viewport.
    pub fn transform_style(&self) -> String {
        let s = self.scale.get();
        let x = self.offset_x.get();
        let y = self.offset_y.get();
        format!(
            "transform: translate({x:.1}px, {y:.1}px) scale({s:.2}); transform-origin: center center;"
        )
    }
}

impl Default for ViewerZoomState {
    fn default() -> Self {
        Self::new()
    }
}
