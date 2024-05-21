pub struct Camera {
    pub position: Vec<i32>,
    pub angle: Vec<i32>,
}

impl Camera {
    pub fn new() -> Camera {
        let mut position: Vec<i32> = vec![0, 0, 0];
        let mut angle = Vec::new();

        Camera { position, angle }
    }

    pub fn change_position(&mut self, velocity: Vec<i32>) {
        for i in 0..2 {
            self.position[i] += velocity[i];
        }
    }
}
