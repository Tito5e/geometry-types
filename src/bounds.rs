use crate::{
    LogicalPosition, LogicalRect, LogicalSize, PhysicalPosition, PhysicalRect, PhysicalSize,
    round_i32,
};

/// A region described by its four edges, in physical (device pixel) space.
///
/// The region covers `[left, right) × [top, bottom)`. Bounds whose
/// `right <= left` or `bottom <= top` have no area: they contain nothing and
/// intersect nothing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PhysicalBounds {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl PhysicalBounds {
    #[inline]
    #[must_use]
    pub const fn lrtb(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Creates bounds from a rect. Edges that would exceed `i32::MAX` saturate.
    #[inline]
    #[must_use]
    pub const fn from_rect(rect: &PhysicalRect) -> Self {
        Self::lrtb(
            rect.x,
            rect.x.saturating_add_unsigned(rect.width),
            rect.y,
            rect.y.saturating_add_unsigned(rect.height),
        )
    }

    /// Converts to a rect. See [`PhysicalRect::from_bounds`].
    #[inline]
    #[must_use]
    pub const fn to_rect(&self) -> PhysicalRect {
        PhysicalRect::from_bounds(self)
    }

    #[inline]
    #[must_use]
    pub const fn from_position_size(position: PhysicalPosition, size: PhysicalSize) -> Self {
        Self::from_rect(&PhysicalRect::from_position_size(position, size))
    }

    /// Converts from logical space, rounding each edge to the nearest pixel.
    #[inline]
    #[must_use]
    pub const fn from_logical(other: &LogicalBounds, scale_factor: f64) -> Self {
        Self::lrtb(
            round_i32(other.left as f64 * scale_factor),
            round_i32(other.right as f64 * scale_factor),
            round_i32(other.top as f64 * scale_factor),
            round_i32(other.bottom as f64 * scale_factor),
        )
    }

    #[inline]
    #[must_use]
    pub const fn to_logical(&self, scale_factor: f64) -> LogicalBounds {
        LogicalBounds::from_physical(self, scale_factor)
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> i32 {
        self.right - self.left
    }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> i32 {
        self.bottom - self.top
    }

    #[inline]
    const fn has_area(&self) -> bool {
        (self.left < self.right) & (self.top < self.bottom)
    }

    /// Returns `true` if the two regions share any area.
    #[inline]
    #[must_use]
    pub const fn intersects(&self, other: &PhysicalBounds) -> bool {
        self.has_area()
            & other.has_area()
            & (self.left < other.right)
            & (other.left < self.right)
            & (self.top < other.bottom)
            & (other.top < self.bottom)
    }

    /// Returns the shared area, or `None` if the regions do not intersect.
    #[inline]
    #[must_use]
    pub fn intersect(&self, other: &PhysicalBounds) -> Option<PhysicalBounds> {
        let b = Self::lrtb(
            self.left.max(other.left),
            self.right.min(other.right),
            self.top.max(other.top),
            self.bottom.min(other.bottom),
        );
        b.has_area().then_some(b)
    }

    /// Returns `true` if `point` lies in `[left, right) × [top, bottom)`.
    #[inline]
    #[must_use]
    pub const fn contains(&self, point: PhysicalPosition) -> bool {
        (self.left <= point.x)
            & (point.x < self.right)
            & (self.top <= point.y)
            & (point.y < self.bottom)
    }

    /// Returns `true` if `other` lies entirely inside `self`.
    /// Always `false` when `other` has no area.
    #[inline]
    #[must_use]
    pub const fn contains_bounds(&self, other: &PhysicalBounds) -> bool {
        other.has_area()
            & (self.left <= other.left)
            & (other.right <= self.right)
            & (self.top <= other.top)
            & (other.bottom <= self.bottom)
    }

    /// Returns the smallest region containing both. If one of them has no
    /// area, the other is returned unchanged.
    #[inline]
    #[must_use]
    pub fn union(&self, other: &PhysicalBounds) -> PhysicalBounds {
        if !self.has_area() {
            return *other;
        }
        if !other.has_area() {
            return *self;
        }
        Self::lrtb(
            self.left.min(other.left),
            self.right.max(other.right),
            self.top.min(other.top),
            self.bottom.max(other.bottom),
        )
    }

    /// Moves the left and right edges inwards by `dx` and the top and bottom
    /// edges inwards by `dy`. Negative values move them outwards.
    /// Edges saturate at the bounds of `i32`.
    #[inline]
    #[must_use]
    pub const fn inset(&self, dx: i32, dy: i32) -> PhysicalBounds {
        Self::lrtb(
            self.left.saturating_add(dx),
            self.right.saturating_sub(dx),
            self.top.saturating_add(dy),
            self.bottom.saturating_sub(dy),
        )
    }
}

impl From<PhysicalRect> for PhysicalBounds {
    #[inline]
    fn from(value: PhysicalRect) -> Self {
        Self::from_rect(&value)
    }
}

impl_offset_ops!(PhysicalBounds, PhysicalPosition, { left: x, right: x, top: y, bottom: y });

/// A region described by its four edges, in logical (scale-independent) space.
///
/// The region covers `[left, right) × [top, bottom)`. Bounds whose
/// `right <= left` or `bottom <= top` (or any NaN edge) have no area: they
/// contain nothing and intersect nothing.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct LogicalBounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl LogicalBounds {
    #[inline]
    #[must_use]
    pub const fn lrtb(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_rect(rect: &LogicalRect) -> Self {
        Self::lrtb(rect.x, rect.x + rect.width, rect.y, rect.y + rect.height)
    }

    /// Converts to a rect. See [`LogicalRect::from_bounds`].
    #[inline]
    #[must_use]
    pub const fn to_rect(&self) -> LogicalRect {
        LogicalRect::from_bounds(self)
    }

    #[inline]
    #[must_use]
    pub const fn from_position_size(position: LogicalPosition, size: LogicalSize) -> Self {
        Self::from_rect(&LogicalRect::from_position_size(position, size))
    }

    #[inline]
    #[must_use]
    pub const fn from_physical(other: &PhysicalBounds, scale_factor: f64) -> Self {
        Self::lrtb(
            (other.left as f64 / scale_factor) as _,
            (other.right as f64 / scale_factor) as _,
            (other.top as f64 / scale_factor) as _,
            (other.bottom as f64 / scale_factor) as _,
        )
    }

    /// Converts to physical space, rounding each edge to the nearest pixel.
    #[inline]
    #[must_use]
    pub const fn to_physical(&self, scale_factor: f64) -> PhysicalBounds {
        PhysicalBounds::from_logical(self, scale_factor)
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> f32 {
        self.right - self.left
    }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> f32 {
        self.bottom - self.top
    }

    #[inline]
    const fn has_area(&self) -> bool {
        (self.left < self.right) & (self.top < self.bottom)
    }

    /// Returns `true` if the two regions share any area.
    #[inline]
    #[must_use]
    pub const fn intersects(&self, other: &LogicalBounds) -> bool {
        self.has_area()
            & other.has_area()
            & (self.left < other.right)
            & (other.left < self.right)
            & (self.top < other.bottom)
            & (other.top < self.bottom)
    }

    /// Returns the shared area, or `None` if the regions do not intersect.
    #[inline]
    #[must_use]
    pub const fn intersect(&self, other: &LogicalBounds) -> Option<LogicalBounds> {
        let b = Self::lrtb(
            self.left.max(other.left),
            self.right.min(other.right),
            self.top.max(other.top),
            self.bottom.min(other.bottom),
        );
        if b.has_area() { Some(b) } else { None }
    }

    /// Returns `true` if `point` lies in `[left, right) × [top, bottom)`.
    #[inline]
    #[must_use]
    pub const fn contains(&self, point: LogicalPosition) -> bool {
        (self.left <= point.x)
            & (point.x < self.right)
            & (self.top <= point.y)
            & (point.y < self.bottom)
    }

    /// Returns `true` if `other` lies entirely inside `self`.
    /// Always `false` when `other` has no area.
    #[inline]
    #[must_use]
    pub const fn contains_bounds(&self, other: &LogicalBounds) -> bool {
        other.has_area()
            & (self.left <= other.left)
            & (other.right <= self.right)
            & (self.top <= other.top)
            & (other.bottom <= self.bottom)
    }

    /// Returns the smallest region containing both. If one of them has no
    /// area, the other is returned unchanged.
    #[inline]
    #[must_use]
    pub const fn union(&self, other: &LogicalBounds) -> LogicalBounds {
        if !self.has_area() {
            return *other;
        }
        if !other.has_area() {
            return *self;
        }
        Self::lrtb(
            self.left.min(other.left),
            self.right.max(other.right),
            self.top.min(other.top),
            self.bottom.max(other.bottom),
        )
    }

    /// Moves the left and right edges inwards by `dx` and the top and bottom
    /// edges inwards by `dy`. Negative values move them outwards.
    #[inline]
    #[must_use]
    pub const fn inset(&self, dx: f32, dy: f32) -> LogicalBounds {
        Self::lrtb(
            self.left + dx,
            self.right - dx,
            self.top + dy,
            self.bottom - dy,
        )
    }
}

impl From<LogicalRect> for LogicalBounds {
    #[inline]
    fn from(value: LogicalRect) -> Self {
        Self::from_rect(&value)
    }
}

impl_offset_ops!(LogicalBounds, LogicalPosition, { left: x, right: x, top: y, bottom: y });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverted_bounds_intersect_nothing() {
        let inverted = PhysicalBounds::lrtb(5, 3, 0, 10);
        let normal = PhysicalBounds::lrtb(0, 10, 0, 10);
        assert!(!inverted.intersects(&normal));
        assert!(!normal.intersects(&inverted));
        assert_eq!(inverted.intersect(&normal), None);

        let inverted = LogicalBounds::lrtb(5., 3., 0., 10.);
        let normal = LogicalBounds::lrtb(0., 10., 0., 10.);
        assert!(!inverted.intersects(&normal));
        assert_eq!(inverted.intersect(&normal), None);
    }

    #[test]
    fn intersect() {
        let a = PhysicalBounds::lrtb(0, 10, 0, 10);
        let b = PhysicalBounds::lrtb(5, 15, -5, 5);
        assert_eq!(a.intersect(&b), Some(PhysicalBounds::lrtb(5, 10, 0, 5)));
        // Touching edges share no area.
        let c = PhysicalBounds::lrtb(10, 20, 0, 10);
        assert!(!a.intersects(&c));
        assert_eq!(a.intersect(&c), None);
    }

    #[test]
    fn contains_is_half_open() {
        let b = PhysicalBounds::lrtb(0, 10, 0, 10);
        assert!(b.contains(PhysicalPosition::new(0, 0)));
        assert!(b.contains(PhysicalPosition::new(9, 9)));
        assert!(!b.contains(PhysicalPosition::new(10, 5)));
        assert!(!b.contains(PhysicalPosition::new(5, 10)));
        assert!(!PhysicalBounds::lrtb(5, 3, 0, 10).contains(PhysicalPosition::new(4, 4)));

        let b = LogicalBounds::lrtb(0., 1., 0., 1.);
        assert!(b.contains(LogicalPosition::new(0., 0.)));
        assert!(!b.contains(LogicalPosition::new(1., 0.5)));
    }

    #[test]
    fn contains_bounds() {
        let outer = PhysicalBounds::lrtb(0, 10, 0, 10);
        assert!(outer.contains_bounds(&outer));
        assert!(outer.contains_bounds(&PhysicalBounds::lrtb(2, 8, 2, 8)));
        assert!(!outer.contains_bounds(&PhysicalBounds::lrtb(2, 11, 2, 8)));
        assert!(!outer.contains_bounds(&PhysicalBounds::lrtb(3, 3, 3, 3)));
    }

    #[test]
    fn union() {
        let a = PhysicalBounds::lrtb(0, 5, 0, 5);
        let b = PhysicalBounds::lrtb(10, 15, -5, 2);
        assert_eq!(a.union(&b), PhysicalBounds::lrtb(0, 15, -5, 5));
        let empty = PhysicalBounds::lrtb(100, 100, 100, 100);
        assert_eq!(a.union(&empty), a);
        assert_eq!(empty.union(&a), a);
    }

    #[test]
    fn inset() {
        let b = PhysicalBounds::lrtb(0, 10, 0, 10);
        assert_eq!(b.inset(2, 3), PhysicalBounds::lrtb(2, 8, 3, 7));
        assert_eq!(b.inset(-1, -1), PhysicalBounds::lrtb(-1, 11, -1, 11));
        let max = PhysicalBounds::lrtb(i32::MIN, i32::MAX, 0, 0);
        assert_eq!(max.inset(-1, 0), max);
    }

    #[test]
    fn rect_round_trip() {
        let r = PhysicalRect::new(-3, 4, 10, 20);
        assert_eq!(PhysicalBounds::from(r), PhysicalBounds::lrtb(-3, 7, 4, 24));
        assert_eq!(PhysicalBounds::from(r).to_rect(), r);

        let r = LogicalRect::new(-3., 4., 10., 20.);
        assert_eq!(LogicalBounds::from(r).to_rect(), r);
    }

    #[test]
    fn from_rect_saturates() {
        let r = PhysicalRect::new(i32::MAX - 1, 0, 10, u32::MAX);
        assert_eq!(
            PhysicalBounds::from_rect(&r),
            PhysicalBounds::lrtb(i32::MAX - 1, i32::MAX, 0, i32::MAX)
        );
    }

    #[test]
    fn offset_ops() {
        let mut b = PhysicalBounds::lrtb(0, 10, 0, 10) + PhysicalPosition::new(1, 2);
        assert_eq!(b, PhysicalBounds::lrtb(1, 11, 2, 12));
        b -= PhysicalPosition::new(1, 2);
        assert_eq!(b, PhysicalBounds::lrtb(0, 10, 0, 10));
    }
}
