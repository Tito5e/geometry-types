use std::ops;

use crate::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize};

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
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !(self.width > 0 && self.height > 0)
    }

    #[inline]
    #[must_use]
    pub fn intersects(&self, other: &PhysicalRect) -> bool {
        !self.is_empty()
            && !other.is_empty()
            && self.x < other.x + other.width as i32
            && other.x < self.x + self.width as i32
            && self.y < other.y + other.height as i32
            && other.y < self.y + self.height as i32
    }

    #[inline]
    #[must_use]
    pub fn intersect(&self, other: &PhysicalRect) -> Option<PhysicalRect> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = (self.x + self.width as i32).min(other.x + other.width as i32);
        let bottom = (self.y + self.height as i32).min(other.y + other.height as i32);

        if right > x && bottom > y {
            Some(PhysicalRect::new(
                x,
                y,
                (right - x) as u32,
                (bottom - y) as u32,
            ))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub fn from_logical(other: &LogicalRect, scale_factor: f64) -> Self {
        Self::new(
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

impl ops::Add<PhysicalPosition> for PhysicalRect {
    type Output = PhysicalRect;

    fn add(self, other: PhysicalPosition) -> Self::Output {
        PhysicalRect::new(self.x + other.x, self.y + other.y, self.width, self.height)
    }
}

impl ops::Sub<PhysicalPosition> for PhysicalRect {
    type Output = PhysicalRect;

    fn sub(self, other: PhysicalPosition) -> Self::Output {
        PhysicalRect::new(self.x - other.x, self.y - other.y, self.width, self.height)
    }
}

impl ops::Add<PhysicalSize> for PhysicalRect {
    type Output = PhysicalRect;

    fn add(self, other: PhysicalSize) -> Self::Output {
        PhysicalRect::new(
            self.x,
            self.y,
            self.width + other.width,
            self.height + other.height,
        )
    }
}

impl ops::Sub<PhysicalSize> for PhysicalRect {
    type Output = PhysicalRect;

    fn sub(self, other: PhysicalSize) -> Self::Output {
        PhysicalRect::new(
            self.x,
            self.y,
            self.width - other.width,
            self.height - other.height,
        )
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
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !(self.width > 0. && self.height > 0.)
    }

    #[inline]
    #[must_use]
    pub fn intersects(&self, other: &LogicalRect) -> bool {
        !self.is_empty()
            && !other.is_empty()
            && self.x < other.x + other.width
            && other.x < self.x + self.width
            && self.y < other.y + other.height
            && other.y < self.y + self.height
    }

    #[inline]
    #[must_use]
    pub fn intersect(&self, other: &LogicalRect) -> Option<LogicalRect> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);

        if right > x && bottom > y {
            Some(LogicalRect::new(x, y, right - x, bottom - y))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub fn from_physical(other: &PhysicalRect, scale_factor: f64) -> Self {
        Self::new(
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

impl ops::Add<LogicalPosition> for LogicalRect {
    type Output = LogicalRect;

    fn add(self, other: LogicalPosition) -> Self::Output {
        LogicalRect::new(self.x + other.x, self.y + other.y, self.width, self.height)
    }
}

impl ops::Sub<LogicalPosition> for LogicalRect {
    type Output = LogicalRect;

    fn sub(self, other: LogicalPosition) -> Self::Output {
        LogicalRect::new(self.x - other.x, self.y - other.y, self.width, self.height)
    }
}

impl ops::Add<LogicalSize> for LogicalRect {
    type Output = LogicalRect;

    fn add(self, other: LogicalSize) -> Self::Output {
        LogicalRect::new(
            self.x,
            self.y,
            self.width + other.width,
            self.height + other.height,
        )
    }
}

impl ops::Sub<LogicalSize> for LogicalRect {
    type Output = LogicalRect;

    fn sub(self, other: LogicalSize) -> Self::Output {
        LogicalRect::new(
            self.x,
            self.y,
            self.width - other.width,
            self.height - other.height,
        )
    }
}
