use std::ops;

use crate::{LogicalPosition, PhysicalPosition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalBounds {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl PhysicalBounds {
    #[inline]
    #[must_use]
    pub fn lrtb(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    #[inline]
    #[must_use]
    pub fn from_logical(other: &LogicalBounds, scale_factor: f64) -> Self {
        Self::lrtb(
            (other.left as f64 * scale_factor) as _,
            (other.right as f64 * scale_factor) as _,
            (other.top as f64 * scale_factor) as _,
            (other.bottom as f64 * scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_logical(&self, scale_factor: f64) -> LogicalBounds {
        LogicalBounds::from_physical(self, scale_factor)
    }
}

impl ops::Add<PhysicalPosition> for PhysicalBounds {
    type Output = PhysicalBounds;

    fn add(self, other: PhysicalPosition) -> Self::Output {
        PhysicalBounds::lrtb(
            self.left + other.x,
            self.right + other.x,
            self.top + other.y,
            self.bottom + other.y,
        )
    }
}

impl ops::Sub<PhysicalPosition> for PhysicalBounds {
    type Output = PhysicalBounds;

    fn sub(self, other: PhysicalPosition) -> Self::Output {
        PhysicalBounds::lrtb(
            self.left - other.x,
            self.right - other.x,
            self.top - other.y,
            self.bottom - other.y,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogicalBounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl LogicalBounds {
    #[inline]
    #[must_use]
    pub fn lrtb(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    #[inline]
    #[must_use]
    pub fn from_physical(other: &PhysicalBounds, scale_factor: f64) -> Self {
        Self::lrtb(
            (other.left as f64 / scale_factor) as _,
            (other.right as f64 / scale_factor) as _,
            (other.top as f64 / scale_factor) as _,
            (other.bottom as f64 / scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_physical(&self, scale_factor: f64) -> PhysicalBounds {
        PhysicalBounds::from_logical(self, scale_factor)
    }
}

impl ops::Add<LogicalPosition> for LogicalBounds {
    type Output = LogicalBounds;

    fn add(self, other: LogicalPosition) -> Self::Output {
        LogicalBounds::lrtb(
            self.left + other.x,
            self.right + other.x,
            self.top + other.y,
            self.bottom + other.y,
        )
    }
}

impl ops::Sub<LogicalPosition> for LogicalBounds {
    type Output = LogicalBounds;

    fn sub(self, other: LogicalPosition) -> Self::Output {
        LogicalBounds::lrtb(
            self.left - other.x,
            self.right - other.x,
            self.top - other.y,
            self.bottom - other.y,
        )
    }
}
