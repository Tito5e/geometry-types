use std::ops;

use crate::{LogicalBounds, LogicalRect, PhysicalBounds, PhysicalRect, round_u32};

/// The thickness of each edge, in physical (device pixel) space.
///
/// Unlike [`PhysicalBounds`], the fields are thicknesses, not edge positions.
/// Typical uses are padding, margins, border widths, window frames and safe
/// areas.
///
/// - `Bounds - Insets` / `Rect - Insets` shrinks the region by the insets.
/// - `Bounds + Insets` / `Rect + Insets` grows the region by the insets.
///
/// Edges saturate at the bounds of `i32`; a rect shrunk past zero keeps a
/// width / height of 0.
///
/// # Panics
///
/// Subtracting larger insets from insets (`Sub` / `SubAssign`) underflows
/// `u32`: it panics in debug builds and wraps in release builds, like plain
/// `u32` subtraction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PhysicalInsets {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl PhysicalInsets {
    /// No thickness on any edge.
    pub const ZERO: Self = Self::uniform(0);

    #[inline]
    #[must_use]
    pub const fn lrtb(left: u32, right: u32, top: u32, bottom: u32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// The same thickness on every edge.
    #[inline]
    #[must_use]
    pub const fn uniform(thickness: u32) -> Self {
        Self::lrtb(thickness, thickness, thickness, thickness)
    }

    /// Converts from logical space, rounding to the nearest pixel.
    /// Negative thicknesses become 0.
    #[inline]
    #[must_use]
    pub const fn from_logical(other: &LogicalInsets, scale_factor: f64) -> Self {
        Self::lrtb(
            round_u32(other.left as f64 * scale_factor),
            round_u32(other.right as f64 * scale_factor),
            round_u32(other.top as f64 * scale_factor),
            round_u32(other.bottom as f64 * scale_factor),
        )
    }

    #[inline]
    #[must_use]
    pub const fn to_logical(&self, scale_factor: f64) -> LogicalInsets {
        LogicalInsets::from_physical(self, scale_factor)
    }

    /// `left + right`, saturating at `u32::MAX`.
    #[inline]
    #[must_use]
    pub const fn horizontal(&self) -> u32 {
        self.left.saturating_add(self.right)
    }

    /// `top + bottom`, saturating at `u32::MAX`.
    #[inline]
    #[must_use]
    pub const fn vertical(&self) -> u32 {
        self.top.saturating_add(self.bottom)
    }
}

impl_offset_ops!(PhysicalInsets, PhysicalInsets, {
    left: left,
    right: right,
    top: top,
    bottom: bottom,
});

/// The thickness of each edge, in logical (scale-independent) space.
///
/// See [`PhysicalInsets`] for the semantics. Negative thicknesses are allowed
/// and act in the opposite direction.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct LogicalInsets {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl LogicalInsets {
    /// No thickness on any edge.
    pub const ZERO: Self = Self::uniform(0.);

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

    /// The same thickness on every edge.
    #[inline]
    #[must_use]
    pub const fn uniform(thickness: f32) -> Self {
        Self::lrtb(thickness, thickness, thickness, thickness)
    }

    #[inline]
    #[must_use]
    pub const fn from_physical(other: &PhysicalInsets, scale_factor: f64) -> Self {
        Self::lrtb(
            (other.left as f64 / scale_factor) as _,
            (other.right as f64 / scale_factor) as _,
            (other.top as f64 / scale_factor) as _,
            (other.bottom as f64 / scale_factor) as _,
        )
    }

    /// Converts to physical space, rounding to the nearest pixel.
    /// Negative thicknesses become 0.
    #[inline]
    #[must_use]
    pub const fn to_physical(&self, scale_factor: f64) -> PhysicalInsets {
        PhysicalInsets::from_logical(self, scale_factor)
    }

    /// `left + right`.
    #[inline]
    #[must_use]
    pub const fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// `top + bottom`.
    #[inline]
    #[must_use]
    pub const fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl_offset_ops!(LogicalInsets, LogicalInsets, {
    left: left,
    right: right,
    top: top,
    bottom: bottom,
});

// Bounds ± Insets: the canonical implementation.

impl ops::AddAssign<PhysicalInsets> for PhysicalBounds {
    /// Grows the region by the insets.
    #[inline]
    fn add_assign(&mut self, rhs: PhysicalInsets) {
        self.left = self.left.saturating_sub_unsigned(rhs.left);
        self.right = self.right.saturating_add_unsigned(rhs.right);
        self.top = self.top.saturating_sub_unsigned(rhs.top);
        self.bottom = self.bottom.saturating_add_unsigned(rhs.bottom);
    }
}

impl ops::SubAssign<PhysicalInsets> for PhysicalBounds {
    /// Shrinks the region by the insets.
    #[inline]
    fn sub_assign(&mut self, rhs: PhysicalInsets) {
        self.left = self.left.saturating_add_unsigned(rhs.left);
        self.right = self.right.saturating_sub_unsigned(rhs.right);
        self.top = self.top.saturating_add_unsigned(rhs.top);
        self.bottom = self.bottom.saturating_sub_unsigned(rhs.bottom);
    }
}

impl ops::AddAssign<LogicalInsets> for LogicalBounds {
    /// Grows the region by the insets.
    #[inline]
    fn add_assign(&mut self, rhs: LogicalInsets) {
        self.left -= rhs.left;
        self.right += rhs.right;
        self.top -= rhs.top;
        self.bottom += rhs.bottom;
    }
}

impl ops::SubAssign<LogicalInsets> for LogicalBounds {
    /// Shrinks the region by the insets.
    #[inline]
    fn sub_assign(&mut self, rhs: LogicalInsets) {
        self.left += rhs.left;
        self.right -= rhs.right;
        self.top += rhs.top;
        self.bottom -= rhs.bottom;
    }
}

/// Derives `Add` / `Sub` for bounds from their assigning forms.
macro_rules! impl_bounds_insets_ops {
    ($bounds:ty, $insets:ty) => {
        impl ops::Add<$insets> for $bounds {
            type Output = $bounds;

            #[inline]
            fn add(mut self, rhs: $insets) -> $bounds {
                self += rhs;
                self
            }
        }

        impl ops::Sub<$insets> for $bounds {
            type Output = $bounds;

            #[inline]
            fn sub(mut self, rhs: $insets) -> $bounds {
                self -= rhs;
                self
            }
        }
    };
}

impl_bounds_insets_ops!(PhysicalBounds, PhysicalInsets);
impl_bounds_insets_ops!(LogicalBounds, LogicalInsets);

/// Rect ± Insets: delegates to the bounds implementation.
macro_rules! impl_rect_insets_ops {
    ($rect:ty, $insets:ty) => {
        impl ops::Add<$insets> for $rect {
            type Output = $rect;

            /// Grows the rect by the insets.
            #[inline]
            fn add(self, rhs: $insets) -> $rect {
                Self::from_bounds(&(self.to_bounds() + rhs))
            }
        }

        impl ops::AddAssign<$insets> for $rect {
            #[inline]
            fn add_assign(&mut self, rhs: $insets) {
                *self = *self + rhs;
            }
        }

        impl ops::Sub<$insets> for $rect {
            type Output = $rect;

            /// Shrinks the rect by the insets.
            #[inline]
            fn sub(self, rhs: $insets) -> $rect {
                Self::from_bounds(&(self.to_bounds() - rhs))
            }
        }

        impl ops::SubAssign<$insets> for $rect {
            #[inline]
            fn sub_assign(&mut self, rhs: $insets) {
                *self = *self - rhs;
            }
        }
    };
}

impl_rect_insets_ops!(PhysicalRect, PhysicalInsets);
impl_rect_insets_ops!(LogicalRect, LogicalInsets);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        assert_eq!(PhysicalInsets::ZERO, PhysicalInsets::default());
        assert_eq!(LogicalInsets::ZERO, LogicalInsets::default());
        assert_eq!(PhysicalInsets::uniform(3), PhysicalInsets::lrtb(3, 3, 3, 3));
        let i = PhysicalInsets::lrtb(1, 2, 3, 4);
        assert_eq!((i.horizontal(), i.vertical()), (3, 7));
        assert_eq!(
            PhysicalInsets::lrtb(u32::MAX, 1, 0, 0).horizontal(),
            u32::MAX
        );
    }

    #[test]
    fn bounds_grow_and_shrink() {
        let b = PhysicalBounds::lrtb(0, 10, 0, 10);
        let i = PhysicalInsets::lrtb(1, 2, 3, 4);
        assert_eq!(b - i, PhysicalBounds::lrtb(1, 8, 3, 6));
        assert_eq!(b + i, PhysicalBounds::lrtb(-1, 12, -3, 14));
        assert_eq!(b + i - i, b);

        let b = LogicalBounds::lrtb(0., 10., 0., 10.);
        let i = LogicalInsets::lrtb(1., 2., 3., 4.);
        assert_eq!(b - i, LogicalBounds::lrtb(1., 8., 3., 6.));
        assert_eq!(b + i, LogicalBounds::lrtb(-1., 12., -3., 14.));
    }

    #[test]
    fn bounds_saturate() {
        let b = PhysicalBounds::lrtb(i32::MIN, i32::MAX, 0, 0);
        assert_eq!(b + PhysicalInsets::lrtb(1, 1, 0, 0), b);
        let b = PhysicalBounds::lrtb(0, 0, 0, 0) - PhysicalInsets::uniform(u32::MAX);
        assert_eq!(
            b,
            PhysicalBounds::lrtb(i32::MAX, i32::MIN, i32::MAX, i32::MIN)
        );
    }

    #[test]
    fn rect_grow_and_shrink() {
        let r = PhysicalRect::new(10, 10, 100, 50);
        let padding = PhysicalInsets::lrtb(1, 2, 3, 4);
        assert_eq!(r - padding, PhysicalRect::new(11, 13, 97, 43));
        assert_eq!(r + padding, PhysicalRect::new(9, 7, 103, 57));

        let mut r = PhysicalRect::new(0, 0, 4, 4);
        r -= PhysicalInsets::uniform(3);
        assert_eq!((r.width, r.height), (0, 0));

        let r = LogicalRect::new(0., 0., 10., 10.) - LogicalInsets::uniform(1.);
        assert_eq!(r, LogicalRect::new(1., 1., 8., 8.));
    }

    #[test]
    fn insets_combine() {
        let border = PhysicalInsets::uniform(1);
        let padding = PhysicalInsets::lrtb(4, 4, 2, 2);
        assert_eq!(border + padding, PhysicalInsets::lrtb(5, 5, 3, 3));
    }

    #[test]
    fn from_logical_rounds() {
        let i = LogicalInsets::lrtb(1., -1., 0.2, 2.).to_physical(1.5);
        assert_eq!(i, PhysicalInsets::lrtb(2, 0, 0, 3));
    }
}
