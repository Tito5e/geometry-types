#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

impl PhysicalSize {
    #[inline]
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    #[inline]
    #[must_use]
    pub fn from_logical(other: &LogicalSize, scale_factor: f64) -> Self {
        Self::new(
            (other.width as f64 * scale_factor) as _,
            (other.height as f64 * scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_logical(&self, scale_factor: f64) -> LogicalSize {
        LogicalSize::from_physical(self, scale_factor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogicalSize {
    pub width: f32,
    pub height: f32,
}

impl LogicalSize {
    #[inline]
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    #[inline]
    #[must_use]
    pub fn from_physical(other: &PhysicalSize, scale_factor: f64) -> Self {
        Self::new(
            (other.width as f64 / scale_factor) as _,
            (other.height as f64 / scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_physical(&self, scale_factor: f64) -> PhysicalSize {
        PhysicalSize::from_logical(self, scale_factor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalPosition {
    pub x: i32,
    pub y: i32,
}

impl PhysicalPosition {
    #[inline]
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn from_logical(other: &LogicalPosition, scale_factor: f64) -> Self {
        Self::new(
            (other.x as f64 * scale_factor) as _,
            (other.y as f64 * scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_logical(&self, scale_factor: f64) -> LogicalPosition {
        LogicalPosition::from_physical(self, scale_factor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogicalPosition {
    pub x: f32,
    pub y: f32,
}

impl LogicalPosition {
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn from_physical(other: &PhysicalPosition, scale_factor: f64) -> Self {
        Self::new(
            (other.x as f64 / scale_factor) as _,
            (other.y as f64 / scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_physical(&self, scale_factor: f64) -> PhysicalPosition {
        PhysicalPosition::from_logical(self, scale_factor)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl PhysicalRect {
    #[inline]
    #[must_use]
    pub fn lrtb(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    #[must_use]
    pub fn from_logical(other: &LogicalRect, scale_factor: f64) -> Self {
        Self::lrtb(
            (other.x as f64 * scale_factor) as _,
            (other.y as f64 * scale_factor) as _,
            (other.width as f64 * scale_factor) as _,
            (other.height as f64 * scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_logical(&self, scale_factor: f64) -> LogicalRect {
        LogicalRect::from_physical(self, scale_factor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogicalRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl LogicalRect {
    #[inline]
    #[must_use]
    pub fn lrtb(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    #[must_use]
    pub fn from_physical(other: &PhysicalRect, scale_factor: f64) -> Self {
        Self::lrtb(
            (other.x as f64 / scale_factor) as _,
            (other.y as f64 / scale_factor) as _,
            (other.width as f64 / scale_factor) as _,
            (other.height as f64 / scale_factor) as _,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_physical(&self, scale_factor: f64) -> PhysicalRect {
        PhysicalRect::from_logical(self, scale_factor)
    }
}
