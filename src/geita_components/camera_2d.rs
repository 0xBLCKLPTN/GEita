use super::GEitaComponentsEnum;

pub struct Camera2D {
    pub position: Vec<i32>,
    pub angle: Vec<i32>,
    pub speed: i32,
    pub child_components: Vec<GEitaComponentsEnum>,
}

impl Camera2D {
    pub fn new() -> Camera2D {
        let mut position: Vec<i32> = vec![0, 0, 0];
        let mut angle = Vec::new();
        let speed = 10;

        let mut child_components: Vec<GEitaComponentsEnum> = Vec::new();

        Camera2D {
            position,
            angle,
            speed,
            child_components,
        }
    }

    pub fn change_position(&mut self, velocity: Vec<i32>) {
        for i in 0..2 {
            self.position[i] += velocity[i];
        }
    }

    pub fn add_component(&mut self, component: GEitaComponentsEnum) {
        self.child_components.push(component)
    }
}
