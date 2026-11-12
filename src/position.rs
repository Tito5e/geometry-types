use crate::round_i32;

/// A point in physical (device pixel) space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct PhysicalPosition {
    pub x: i32,
    pub y: i32,
}

impl PhysicalPosition {
    /// The origin `(0, 0)`.
    pub const ZERO: Self = Self::new(0, 0);

    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Converts from logical space, rounding to the nearest pixel.
    #[inline]
    #[must_use]
    pub const fn from_logical(other: &LogicalPosition, scale_factor: f64) -> Self {
        Self::new(
            round_i32(other.x as f64 * scale_factor),
            round_i32(other.y as f64 * scale_factor),
        )
    }

    #[inline]
    #[must_use]
    pub const fn to_logical(&self, scale_factor: f64) -> LogicalPosition {
        LogicalPosition::from_physical(self, scale_factor)
    }
}

impl From<(i32, i32)> for PhysicalPosition {
    #[inline]
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl_offset_ops!(PhysicalPosition, PhysicalPosition, { x: x, y: y });

/// A point in logical (scale-independent) space.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct LogicalPosition {
    pub x: f32,
    pub y: f32,
}

impl LogicalPosition {
    /// The origin `(0.0, 0.0)`.
    pub const ZERO: Self = Self::new(0., 0.);

    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn from_physical(other: &PhysicalPosition, scale_factor: f64) -> Self {
        Self::new(
            (other.x as f64 / scale_factor) as _,
            (other.y as f64 / scale_factor) as _,
        )
    }

    /// Converts to physical space, rounding to the nearest pixel.
    #[inline]
    #[must_use]
    pub const fn to_physical(&self, scale_factor: f64) -> PhysicalPosition {
        PhysicalPosition::from_logical(self, scale_factor)
    }
}

impl From<(f32, f32)> for LogicalPosition {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl_offset_ops!(LogicalPosition, LogicalPosition, { x: x, y: y });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_default() {
        assert_eq!(PhysicalPosition::ZERO, PhysicalPosition::default());
        assert_eq!(LogicalPosition::ZERO, LogicalPosition::default());
    }

    #[test]
    fn from_logical_rounds_symmetrically() {
        let p = LogicalPosition::new(1.0, -1.0).to_physical(1.5);
        assert_eq!(p, PhysicalPosition::new(2, -2));
        let p = LogicalPosition::new(0.9, -0.9).to_physical(1.0);
        assert_eq!(p, PhysicalPosition::new(1, -1));
    }

    #[test]
    fn offset_ops() {
        let mut p = PhysicalPosition::new(1, 2) + PhysicalPosition::new(3, 4);
        assert_eq!(p, PhysicalPosition::new(4, 6));
        p -= PhysicalPosition::new(4, 6);
        assert_eq!(p, PhysicalPosition::ZERO);
    }
}
