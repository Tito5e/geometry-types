use crate::{
    LogicalBounds, LogicalPosition, LogicalSize, PhysicalBounds, PhysicalPosition, PhysicalSize,
};

/// An origin offset plus a size, in physical (device pixel) space.
///
/// Region operations convert to [`PhysicalBounds`] and back; edges beyond
/// `i32::MAX` saturate during that conversion.
///
/// # Panics
///
/// Subtracting a larger [`PhysicalSize`] underflows `u32`: it panics in debug
/// builds and wraps in release builds, like plain `u32` subtraction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PhysicalRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl PhysicalRect {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_position_size(position: PhysicalPosition, size: PhysicalSize) -> Self {
        Self::new(position.x, position.y, size.width, size.height)
    }

    /// Creates a rect from bounds. Inverted edges (`right < left` or
    /// `bottom < top`) produce a zero extent.
    #[inline]
    #[must_use]
    pub const fn from_bounds(bounds: &PhysicalBounds) -> Self {
        const fn extent(start: i32, end: i32) -> u32 {
            if end > start { end.abs_diff(start) } else { 0 }
        }
        Self::new(
            bounds.left,
            bounds.top,
            extent(bounds.left, bounds.right),
            extent(bounds.top, bounds.bottom),
        )
    }

    /// Converts to bounds. See [`PhysicalBounds::from_rect`].
    #[inline]
    #[must_use]
    pub const fn to_bounds(&self) -> PhysicalBounds {
        PhysicalBounds::from_rect(self)
    }

    /// Converts from logical space. The edges are rounded to the nearest pixel
    /// (rather than the origin and size separately), so rects that share an
    /// edge in logical space also share it in physical space.
    #[inline]
    #[must_use]
    pub const fn from_logical(other: &LogicalRect, scale_factor: f64) -> Self {
        Self::from_bounds(&PhysicalBounds::from_logical(
            &other.to_bounds(),
            scale_factor,
        ))
    }

    #[inline]
    #[must_use]
    pub const fn to_logical(&self, scale_factor: f64) -> LogicalRect {
        LogicalRect::from_physical(self, scale_factor)
    }

    #[inline]
    #[must_use]
    pub const fn position(&self) -> PhysicalPosition {
        PhysicalPosition::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub const fn size(&self) -> PhysicalSize {
        PhysicalSize::new(self.width, self.height)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        !(self.width > 0 && self.height > 0)
    }

    /// See [`PhysicalBounds::intersects`].
    #[inline]
    #[must_use]
    pub const fn intersects(&self, other: &PhysicalRect) -> bool {
        self.to_bounds().intersects(&other.to_bounds())
    }

    /// See [`PhysicalBounds::intersect`].
    #[inline]
    #[must_use]
    pub fn intersect(&self, other: &PhysicalRect) -> Option<PhysicalRect> {
        self.to_bounds()
            .intersect(&other.to_bounds())
            .map(|b| Self::from_bounds(&b))
    }

    /// See [`PhysicalBounds::contains`].
    #[inline]
    #[must_use]
    pub const fn contains(&self, point: PhysicalPosition) -> bool {
        self.to_bounds().contains(point)
    }

    /// See [`PhysicalBounds::contains_bounds`].
    #[inline]
    #[must_use]
    pub const fn contains_rect(&self, other: &PhysicalRect) -> bool {
        self.to_bounds().contains_bounds(&other.to_bounds())
    }

    /// See [`PhysicalBounds::union`].
    #[inline]
    #[must_use]
    pub fn union(&self, other: &PhysicalRect) -> PhysicalRect {
        Self::from_bounds(&self.to_bounds().union(&other.to_bounds()))
    }

    /// See [`PhysicalBounds::inset`]. If the rect is shrunk past zero, its
    /// width / height stays at 0.
    #[inline]
    #[must_use]
    pub const fn inset(&self, dx: i32, dy: i32) -> PhysicalRect {
        Self::from_bounds(&self.to_bounds().inset(dx, dy))
    }
}

impl From<(i32, i32, u32, u32)> for PhysicalRect {
    /// Creates a rect from `(x, y, width, height)`.
    #[inline]
    fn from(value: (i32, i32, u32, u32)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl From<PhysicalBounds> for PhysicalRect {
    #[inline]
    fn from(value: PhysicalBounds) -> Self {
        Self::from_bounds(&value)
    }
}

impl_offset_ops!(PhysicalRect, PhysicalPosition, { x: x, y: y });
impl_offset_ops!(PhysicalRect, PhysicalSize, { width: width, height: height });

/// An origin offset plus a size, in logical (scale-independent) space.
///
/// Region operations convert to [`LogicalBounds`] and back.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct LogicalRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl LogicalRect {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_position_size(position: LogicalPosition, size: LogicalSize) -> Self {
        Self::new(position.x, position.y, size.width, size.height)
    }

    /// Creates a rect from bounds. Inverted edges (`right < left` or
    /// `bottom < top`) produce a zero extent.
    #[inline]
    #[must_use]
    pub const fn from_bounds(bounds: &LogicalBounds) -> Self {
        Self::new(
            bounds.left,
            bounds.top,
            (bounds.right - bounds.left).max(0.),
            (bounds.bottom - bounds.top).max(0.),
        )
    }

    /// Converts to bounds. See [`LogicalBounds::from_rect`].
    #[inline]
    #[must_use]
    pub const fn to_bounds(&self) -> LogicalBounds {
        LogicalBounds::from_rect(self)
    }

    /// Converts from physical space edge by edge, so rects that share an edge
    /// in physical space also share it in logical space.
    #[inline]
    #[must_use]
    pub const fn from_physical(other: &PhysicalRect, scale_factor: f64) -> Self {
        Self::from_bounds(&LogicalBounds::from_physical(
            &other.to_bounds(),
            scale_factor,
        ))
    }

    /// See [`PhysicalRect::from_logical`].
    #[inline]
    #[must_use]
    pub const fn to_physical(&self, scale_factor: f64) -> PhysicalRect {
        PhysicalRect::from_logical(self, scale_factor)
    }

    #[inline]
    #[must_use]
    pub const fn position(&self) -> LogicalPosition {
        LogicalPosition::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub const fn size(&self) -> LogicalSize {
        LogicalSize::new(self.width, self.height)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        !(self.width > 0. && self.height > 0.)
    }

    /// See [`LogicalBounds::intersects`].
    #[inline]
    #[must_use]
    pub const fn intersects(&self, other: &LogicalRect) -> bool {
        self.to_bounds().intersects(&other.to_bounds())
    }

    /// See [`LogicalBounds::intersect`].
    #[inline]
    #[must_use]
    pub fn intersect(&self, other: &LogicalRect) -> Option<LogicalRect> {
        self.to_bounds()
            .intersect(&other.to_bounds())
            .map(|b| Self::from_bounds(&b))
    }

    /// See [`LogicalBounds::contains`].
    #[inline]
    #[must_use]
    pub const fn contains(&self, point: LogicalPosition) -> bool {
        self.to_bounds().contains(point)
    }

    /// See [`LogicalBounds::contains_bounds`].
    #[inline]
    #[must_use]
    pub const fn contains_rect(&self, other: &LogicalRect) -> bool {
        self.to_bounds().contains_bounds(&other.to_bounds())
    }

    /// See [`LogicalBounds::union`].
    #[inline]
    #[must_use]
    pub const fn union(&self, other: &LogicalRect) -> LogicalRect {
        Self::from_bounds(&self.to_bounds().union(&other.to_bounds()))
    }

    /// See [`LogicalBounds::inset`]. If the rect is shrunk past zero, its
    /// width / height stays at 0.
    #[inline]
    #[must_use]
    pub const fn inset(&self, dx: f32, dy: f32) -> LogicalRect {
        Self::from_bounds(&self.to_bounds().inset(dx, dy))
    }
}

impl From<(f32, f32, f32, f32)> for LogicalRect {
    /// Creates a rect from `(x, y, width, height)`.
    #[inline]
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl From<LogicalBounds> for LogicalRect {
    #[inline]
    fn from(value: LogicalBounds) -> Self {
        Self::from_bounds(&value)
    }
}

impl_offset_ops!(LogicalRect, LogicalPosition, { x: x, y: y });
impl_offset_ops!(LogicalRect, LogicalSize, { width: width, height: height });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects_near_i32_max_does_not_panic() {
        let a = PhysicalRect::new(i32::MAX - 1, 0, 10, 10);
        let b = PhysicalRect::new(i32::MAX - 5, 0, 10, 10);
        assert!(a.intersects(&b));
        assert_eq!(
            a.intersect(&b),
            Some(PhysicalRect::new(i32::MAX - 1, 0, 1, 10))
        );
        let huge = PhysicalRect::new(0, 0, u32::MAX, u32::MAX);
        assert!(huge.intersects(&PhysicalRect::new(5, 5, 1, 1)));
    }

    #[test]
    fn empty_rects_intersect_nothing() {
        let a = PhysicalRect::new(0, 0, 10, 10);
        let empty = PhysicalRect::new(5, 5, 0, 10);
        assert!(!a.intersects(&empty));
        assert_eq!(a.intersect(&empty), None);
    }

    #[test]
    fn from_bounds_clamps_inverted_edges() {
        let r = PhysicalRect::from_bounds(&PhysicalBounds::lrtb(5, 3, 0, 10));
        assert_eq!(r, PhysicalRect::new(5, 0, 0, 10));
        let full = PhysicalRect::from_bounds(&PhysicalBounds::lrtb(i32::MIN, i32::MAX, 0, 1));
        assert_eq!(full.width, u32::MAX);

        let r = LogicalRect::from_bounds(&LogicalBounds::lrtb(5., 3., 0., 10.));
        assert_eq!(r, LogicalRect::new(5., 0., 0., 10.));
    }

    #[test]
    fn position_size() {
        let r =
            PhysicalRect::from_position_size(PhysicalPosition::new(1, 2), PhysicalSize::new(3, 4));
        assert_eq!(r, PhysicalRect::from((1, 2, 3, 4)));
        assert_eq!(r.position(), PhysicalPosition::new(1, 2));
        assert_eq!(r.size(), PhysicalSize::new(3, 4));
        assert_eq!(
            PhysicalBounds::from_position_size(r.position(), r.size()),
            r.to_bounds()
        );
    }

    #[test]
    fn region_ops_delegate_to_bounds() {
        let a = PhysicalRect::new(0, 0, 10, 10);
        assert!(a.contains(PhysicalPosition::new(0, 0)));
        assert!(!a.contains(PhysicalPosition::new(10, 0)));
        assert!(a.contains_rect(&PhysicalRect::new(1, 1, 9, 9)));
        assert!(!a.contains_rect(&PhysicalRect::new(1, 1, 10, 9)));
        assert_eq!(
            a.union(&PhysicalRect::new(20, -5, 5, 5)),
            PhysicalRect::new(0, -5, 25, 15)
        );
        assert_eq!(a.union(&PhysicalRect::new(50, 50, 0, 0)), a);
    }

    #[test]
    fn inset_stops_at_zero() {
        let r = PhysicalRect::new(0, 0, 10, 4);
        assert_eq!(r.inset(1, 1), PhysicalRect::new(1, 1, 8, 2));
        assert_eq!(r.inset(-2, 0), PhysicalRect::new(-2, 0, 14, 4));
        let collapsed = r.inset(1, 3);
        assert_eq!(collapsed.height, 0);

        let r = LogicalRect::new(0., 0., 1., 1.);
        assert_eq!(r.inset(1., 0.).width, 0.);
    }

    #[test]
    fn from_logical_rounds_edges() {
        // Edges: -1.5 -> -2, 1.5 -> 2, 1.5 -> 2, 4.5 -> 5 (symmetric around 0).
        let r = LogicalRect::new(-1., 1., 2., 2.).to_physical(1.5);
        assert_eq!(r, PhysicalRect::new(-2, 2, 4, 3));
    }

    #[test]
    fn adjacent_rects_stay_adjacent() {
        for scale in [1.0, 1.25, 1.5, 1.75, 2.25] {
            let mut x = 0.0f32;
            for w in [0.3f32, 1.7, 2.45, 10.1, 3.33] {
                let a = LogicalRect::new(x, 0., w, 1.).to_physical(scale);
                let b = LogicalRect::new(x + w, 0., 1.9, 1.).to_physical(scale);
                assert_eq!(a.to_bounds().right, b.x, "scale {scale}, x {x}, w {w}");
                x += w;
            }
        }
    }

    #[test]
    fn offset_ops() {
        let mut r = PhysicalRect::new(0, 0, 10, 10);
        r += PhysicalPosition::new(1, 2);
        r += PhysicalSize::new(3, 4);
        assert_eq!(r, PhysicalRect::new(1, 2, 13, 14));
        r -= PhysicalPosition::new(1, 2);
        r -= PhysicalSize::new(3, 4);
        assert_eq!(r, PhysicalRect::new(0, 0, 10, 10));

        let r = LogicalRect::new(0., 0., 1., 1.) + LogicalPosition::new(1., 1.);
        assert_eq!(r, LogicalRect::new(1., 1., 1., 1.));
    }
}
