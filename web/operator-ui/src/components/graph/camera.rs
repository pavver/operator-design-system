//! Graph camera: pan and zoom between world and screen coordinates (pure logic).

/// A rectangle in world coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WorldRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl WorldRect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    pub fn right(self) -> f64 {
        self.x + self.w
    }

    pub fn bottom(self) -> f64 {
        self.y + self.h
    }

    /// The smallest rectangle containing both.
    pub fn union(self, other: Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        Self::new(
            x,
            y,
            self.right().max(other.right()) - x,
            self.bottom().max(other.bottom()) - y,
        )
    }

    /// The smallest rectangle containing all, or `None` for none.
    pub fn bounds(rects: impl IntoIterator<Item = Self>) -> Option<Self> {
        rects.into_iter().reduce(Self::union)
    }
}

/// Screen offset and scale of the world: `screen = world * zoom + (x, y)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn to_world(self, sx: f64, sy: f64) -> (f64, f64) {
        ((sx - self.x) / self.zoom, (sy - self.y) / self.zoom)
    }

    pub fn to_screen(self, wx: f64, wy: f64) -> (f64, f64) {
        (wx * self.zoom + self.x, wy * self.zoom + self.y)
    }

    /// Zooms by `factor` within the limits, keeping the screen point `(sx, sy)` fixed.
    pub fn zoom_at(self, sx: f64, sy: f64, factor: f64, min: f64, max: f64) -> Self {
        let zoom = (self.zoom * factor).clamp(min, max);
        let (wx, wy) = self.to_world(sx, sy);
        Self {
            x: sx - wx * zoom,
            y: sy - wy * zoom,
            zoom,
        }
    }

    /// Moved by a screen delta.
    pub fn pan(self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            ..self
        }
    }

    /// The world rectangle visible in a viewport of `w` × `h` screen pixels.
    pub fn visible(self, w: f64, h: f64) -> WorldRect {
        let (x, y) = self.to_world(0.0, 0.0);
        WorldRect::new(x, y, w / self.zoom, h / self.zoom)
    }

    /// Centers the world point in a viewport of `w` × `h`, keeping the zoom.
    pub fn centered_on(self, wx: f64, wy: f64, w: f64, h: f64) -> Self {
        Self {
            x: w / 2.0 - wx * self.zoom,
            y: h / 2.0 - wy * self.zoom,
            zoom: self.zoom,
        }
    }

    /// Fits `bounds` with `pad` world units around it into a `w` × `h` viewport.
    ///
    /// The zoom stays within `min` and `max`; content smaller than the viewport is centered.
    pub fn fit(bounds: WorldRect, w: f64, h: f64, pad: f64, min: f64, max: f64) -> Self {
        let bw = (bounds.w + 2.0 * pad).max(1.0);
        let bh = (bounds.h + 2.0 * pad).max(1.0);
        let zoom = (w / bw).min(h / bh).clamp(min, max);
        Self {
            x: (w - bounds.w * zoom) / 2.0 - bounds.x * zoom,
            y: (h - bounds.h * zoom) / 2.0 - bounds.y * zoom,
            zoom,
        }
    }
}
