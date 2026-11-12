//! Simple geometry collections for 2D graphics.
//!
//! Every type comes in two flavours:
//!
//! - **Physical** types use integer device pixels (`i32` coordinates, `u32` extents).
//! - **Logical** types use `f32` units that are independent of the display scale.
//!
//! The two are converted with an explicit `scale_factor`
//! (`physical = logical * scale_factor`). Values are meant to be kept in one
//! space and converted only once, at the point where the other space is needed.
//! Logical → physical conversions round to the nearest integer (halfway cases
//! away from zero).
//!
//! # Rect and Bounds
//!
//! - A **Rect** is an origin offset plus a size (`x`, `y`, `width`, `height`).
//! - A **Bounds** is four edges (`left`, `right`, `top`, `bottom`).
//!
//! Use `Bounds` whenever you need to reason about edges. Region operations
//! (`intersect`, `union`, `contains`, `inset`, ...) are implemented on `Bounds`;
//! the `Rect` versions convert to `Bounds` and back.
//!
//! All region tests use half-open intervals: `[left, right) × [top, bottom)`.
//! A region whose `right <= left` or `bottom <= top` has no area: it contains
//! nothing and intersects nothing.
//!
//! # Insets
//!
//! An **Insets** is the thickness of each edge (padding, borders, window
//! frames, safe areas). Subtracting insets from a `Bounds` or `Rect` shrinks
//! the region; adding them grows it.
//!
//! ```
//! use geometry_types::{PhysicalInsets, PhysicalRect};
//!
//! let window = PhysicalRect::new(0, 0, 800, 600);
//! let content = window - PhysicalInsets::lrtb(8, 8, 32, 8);
//! assert_eq!(content, PhysicalRect::new(8, 32, 784, 560));
//! ```

/// Implements `Add`, `AddAssign`, `Sub` and `SubAssign` of `$rhs` for `$lhs`,
/// where each listed field of `$lhs` is offset by the paired field of `$rhs`.
macro_rules! impl_offset_ops {
    ($lhs:ty, $rhs:ty, { $($lf:ident : $rf:ident),* $(,)? }) => {
        impl ::std::ops::Add<$rhs> for $lhs {
            type Output = $lhs;

            #[inline]
            fn add(mut self, rhs: $rhs) -> $lhs {
                self += rhs;
                self
            }
        }

        impl ::std::ops::AddAssign<$rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                $( self.$lf += rhs.$rf; )*
            }
        }

        impl ::std::ops::Sub<$rhs> for $lhs {
            type Output = $lhs;

            #[inline]
            fn sub(mut self, rhs: $rhs) -> $lhs {
                self -= rhs;
                self
            }
        }

        impl ::std::ops::SubAssign<$rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs) {
                $( self.$lf -= rhs.$rf; )*
            }
        }
    };
}

mod bounds;
mod insets;
mod position;
mod rect;
mod size;

pub use bounds::LogicalBounds;
pub use bounds::PhysicalBounds;
pub use insets::LogicalInsets;
pub use insets::PhysicalInsets;
pub use position::LogicalPosition;
pub use position::PhysicalPosition;
pub use rect::LogicalRect;
pub use rect::PhysicalRect;
pub use size::LogicalSize;
pub use size::PhysicalSize;

/// Rounds to the nearest integer, halfway cases away from zero, saturating at
/// the bounds of `i32` (NaN becomes 0).
///
/// Unlike `f64::round`, this compiles to inline arithmetic on every target
/// instead of a libm call. It may differ from `f64::round` by one for inputs
/// just below a halfway point (e.g. `0.49999999999999994`).
#[inline(always)]
pub(crate) const fn round_i32(v: f64) -> i32 {
    (v + 0.5f64.copysign(v)) as i32
}

/// Same as [`round_i32`], saturating at the bounds of `u32` (negative values
/// become 0).
#[inline(always)]
pub(crate) const fn round_u32(v: f64) -> u32 {
    (v + 0.5f64.copysign(v)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding_is_symmetric() {
        assert_eq!(round_i32(1.5), 2);
        assert_eq!(round_i32(-1.5), -2);
        assert_eq!(round_i32(1.4), 1);
        assert_eq!(round_i32(-1.4), -1);
        assert_eq!(round_i32(0.0), 0);
    }

    #[test]
    fn rounding_saturates() {
        assert_eq!(round_i32(f64::MAX), i32::MAX);
        assert_eq!(round_i32(f64::MIN), i32::MIN);
        assert_eq!(round_i32(f64::NAN), 0);
        assert_eq!(round_u32(-3.0), 0);
        assert_eq!(round_u32(f64::MAX), u32::MAX);
    }
}
