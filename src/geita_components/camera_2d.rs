pub struct Camera2D {
    pub position: Vec<i32>,
    pub angle: Vec<i32>,
    pub speed: i32,
}

impl Camera2D {
    pub fn new() -> Camera2D {
        let mut position: Vec<i32> = vec![0, 0, 0];
        let mut angle = Vec::new();
        let speed = 10;

        Camera2D {
            position,
            angle,
            speed,
        }
    }

    pub fn change_position(&mut self, velocity: Vec<i32>) {
        for i in 0..2 {
            self.position[i] += velocity[i];
        }
    }
}
