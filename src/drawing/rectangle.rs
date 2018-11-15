
pub struct Rectangle {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Rectangle {
    pub fn new(
        left: u32, 
        right: u32, 
        top: u32, 
        bottom: u32) -> Rectangle
    {
        Rectangle {
            left: left,
            right: right,
            top: top,
            bottom: bottom,
        }
    }
}