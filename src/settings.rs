pub struct Vec2f {
    pub x: f64,
    pub y: f64
}

pub struct Settings {
    pub wind_size: Vec2f,
    pub cell_size: Vec2f,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            wind_size: Vec2f{ x: 640.0, y: 480.0 },
            cell_size: Vec2f{ x: 0.0, y: 0.0 },
        }
    }
}