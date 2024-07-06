use crate::config::FPS;

#[derive(PartialEq, Debug)]
pub struct Statistics {
    max_vehicles: usize,
    max_velocity: f32,
    min_velocity: f32,
    max_time: f32,
    min_time: f32,
    close_calls: u32,
    collisions: u32,
    average_time: f32,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            max_vehicles: 0,
            max_velocity: 0.0,
            min_velocity: 0.0,
            max_time: 0.0,
            min_time: 0.0,
            close_calls: 0,
            collisions: 0,
            average_time: 0.0, // Initialisation de average_time à 0.0
        }
    }

    // Setters
    pub fn set_max_vehicles(&mut self, max_vehicles: usize) {
        if max_vehicles > self.max_vehicles {
            self.max_vehicles = max_vehicles;
        }
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        self.set_min_velocity(velocity);
        self.set_max_velocity(velocity);
    }

    pub fn set_max_velocity(&mut self, max_velocity: f32) {
        if max_velocity > self.max_velocity {
            self.max_velocity = max_velocity;
        }
    }

    pub fn set_min_velocity(&mut self, min_velocity: f32) {
        if self.min_velocity == 0.0 {
            self.min_velocity = min_velocity;
        }

        if min_velocity < self.min_velocity {
            self.min_velocity = min_velocity;
        }
    }

    pub fn set_time(&mut self, time: f32) {
        self.set_min_time(time);
        self.set_max_time(time);
    }

    pub fn set_max_time(&mut self, max_time: f32) {
        if max_time > self.max_time {
            self.max_time = max_time;
        }
    }

    pub fn set_min_time(&mut self, min_time: f32) {
        if self.min_time == 0.0 {
            self.min_time = min_time;
        }

        if min_time < self.min_time {
            self.min_time = min_time;
        }
    }

    pub fn add_time(&mut self, time: f32) {
        let current_count = self.close_calls + self.collisions;
        self.average_time = (self.average_time * current_count as f32 + time) / (current_count + 1) as f32;
    }

    pub fn set_close_calls(&mut self) {
        self.close_calls += 1;
    }

    pub fn set_collisions(&mut self) {
        self.collisions += 1;
    }

    // Getters
    pub fn max_vehicles(&self) -> usize {
        self.max_vehicles
    }

    pub fn max_velocity(&self) -> f32 {
        self.max_velocity
    }

    pub fn min_velocity(&self) -> f32 {
        self.min_velocity
    }

    pub fn max_time(&self) -> f32 {
        self.max_time
    }

    pub fn min_time(&self) -> f32 {
        self.min_time
    }

    pub fn average_time(&self) -> f32 {
        (self.min_time + self.max_time) / 2.0
    }

    pub fn close_calls(&self) -> u32 {
        (self.close_calls / 2) / FPS as u32
    }

    pub fn collisions(&self) -> u32 {
        (self.collisions / 2) / FPS as u32
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self::new()
    }
}
