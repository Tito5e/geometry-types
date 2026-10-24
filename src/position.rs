use std::ops;

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

impl From<(i32, i32)> for PhysicalPosition {
    #[inline]
    fn from(value: (i32, i32)) -> Self {
        PhysicalPosition::new(value.0, value.1)
    }
}

impl ops::Add<PhysicalPosition> for PhysicalPosition {
    type Output = PhysicalPosition;

    fn add(self, other: PhysicalPosition) -> PhysicalPosition {
        PhysicalPosition::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign for PhysicalPosition {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<PhysicalPosition> for PhysicalPosition {
    type Output = PhysicalPosition;

    fn sub(self, other: PhysicalPosition) -> Self::Output {
        PhysicalPosition::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::SubAssign for PhysicalPosition {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
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

impl From<(f32, f32)> for LogicalPosition {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        LogicalPosition::new(value.0, value.1)
    }
}

impl ops::Add<LogicalPosition> for LogicalPosition {
    type Output = LogicalPosition;

    fn add(self, other: LogicalPosition) -> LogicalPosition {
        LogicalPosition::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign for LogicalPosition {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<LogicalPosition> for LogicalPosition {
    type Output = LogicalPosition;

    fn sub(self, other: LogicalPosition) -> Self::Output {
        LogicalPosition::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::SubAssign for LogicalPosition {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
