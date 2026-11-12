use crate::round_u32;

/// A two-dimensional extent in physical (device pixel) space.
///
/// # Panics
///
/// Subtracting a larger size (`Sub` / `SubAssign`) underflows `u32`: it panics
/// in debug builds and wraps in release builds, like plain `u32` subtraction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

impl PhysicalSize {
    /// A size of `0 × 0`.
    pub const ZERO: Self = Self::new(0, 0);

    #[inline]
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Converts from logical space, rounding to the nearest pixel.
    /// Negative extents become 0.
    #[inline]
    #[must_use]
    pub const fn from_logical(other: &LogicalSize, scale_factor: f64) -> Self {
        Self::new(
            round_u32(other.width as f64 * scale_factor),
            round_u32(other.height as f64 * scale_factor),
        )
    }

    #[inline]
    #[must_use]
    pub const fn to_logical(&self, scale_factor: f64) -> LogicalSize {
        LogicalSize::from_physical(self, scale_factor)
    }
}

impl From<(u32, u32)> for PhysicalSize {
    #[inline]
    fn from(value: (u32, u32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl_offset_ops!(PhysicalSize, PhysicalSize, { width: width, height: height });

/// A two-dimensional extent in logical (scale-independent) space.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct LogicalSize {
    pub width: f32,
    pub height: f32,
}

impl LogicalSize {
    /// A size of `0.0 × 0.0`.
    pub const ZERO: Self = Self::new(0., 0.);

    #[inline]
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    #[inline]
    #[must_use]
    pub const fn from_physical(other: &PhysicalSize, scale_factor: f64) -> Self {
        Self::new(
            (other.width as f64 / scale_factor) as _,
            (other.height as f64 / scale_factor) as _,
        )
    }

    /// Converts to physical space, rounding to the nearest pixel.
    /// Negative extents become 0.
    #[inline]
    #[must_use]
    pub const fn to_physical(&self, scale_factor: f64) -> PhysicalSize {
        PhysicalSize::from_logical(self, scale_factor)
    }
}

impl From<(f32, f32)> for LogicalSize {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl_offset_ops!(LogicalSize, LogicalSize, { width: width, height: height });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_default() {
        assert_eq!(PhysicalSize::ZERO, PhysicalSize::default());
        assert_eq!(LogicalSize::ZERO, LogicalSize::default());
    }

    #[test]
    fn from_logical_rounds() {
        assert_eq!(
            LogicalSize::new(1.0, 0.2).to_physical(1.5),
            PhysicalSize::new(2, 0)
        );
        assert_eq!(
            LogicalSize::new(-4.0, 3.0).to_physical(1.0),
            PhysicalSize::new(0, 3)
        );
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn sub_underflow_panics_in_debug() {
        let _ = PhysicalSize::new(1, 1) - PhysicalSize::new(2, 0);
    }
}
