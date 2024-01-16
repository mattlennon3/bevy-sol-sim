
pub struct ZLevels {
    pub background: f32,
    pub foreground: f32,
    pub ui: f32,
    pub cursor: f32,
}

pub const Z_LEVELS: ZLevels = ZLevels {
    background: 0.0,
    foreground: 1.0,
    ui: 2.0,
    cursor: 3.0,
};