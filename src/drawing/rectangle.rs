pub struct Rectangle {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

pub struct Region {
    pub width: u32,
    pub height: u32,
}

impl Region {
    pub fn new(width: u32, height: u32) -> Region {
        Region { width, height}
    }
}

impl Rectangle {
    pub fn new(left: u32, right: u32, top: u32, bottom: u32) -> Rectangle {
        Rectangle {
            left,
            right,
            top,
            bottom,
        }
    }
}
