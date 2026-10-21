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
