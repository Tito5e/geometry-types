use std::ops;

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

impl From<(u32, u32)> for PhysicalSize {
    #[inline]
    fn from(value: (u32, u32)) -> Self {
        PhysicalSize::new(value.0, value.1)
    }
}

impl ops::Add<PhysicalSize> for PhysicalSize {
    type Output = PhysicalSize;

    fn add(self, other: PhysicalSize) -> PhysicalSize {
        PhysicalSize::new(self.width + other.width, self.height + other.height)
    }
}

impl ops::AddAssign for PhysicalSize {
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl ops::Sub<PhysicalSize> for PhysicalSize {
    type Output = PhysicalSize;

    fn sub(self, other: PhysicalSize) -> Self::Output {
        PhysicalSize::new(self.width - other.width, self.height - other.height)
    }
}

impl ops::SubAssign for PhysicalSize {
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
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

impl From<(f32, f32)> for LogicalSize {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        LogicalSize::new(value.0, value.1)
    }
}

impl ops::Add<LogicalSize> for LogicalSize {
    type Output = LogicalSize;

    fn add(self, other: LogicalSize) -> LogicalSize {
        LogicalSize::new(self.width + other.width, self.height + other.height)
    }
}

impl ops::AddAssign for LogicalSize {
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl ops::Sub<LogicalSize> for LogicalSize {
    type Output = LogicalSize;

    fn sub(self, other: LogicalSize) -> Self::Output {
        LogicalSize::new(self.width - other.width, self.height - other.height)
    }
}

impl ops::SubAssign for LogicalSize {
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
    }
}
