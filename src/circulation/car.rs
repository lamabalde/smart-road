use macroquad::rand::gen_range;
use std::time::SystemTime;

use crate::circulation::path::{Path, Sector};
use crate::circulation::{Direction, Statistics};

use crate::config::{
    ACCELERATION_DISTANCE, CLOSE_CALL_DISTANCE, FPS, MAX_VELOCITY, SCAN_DISTANCE, SECTOR_WIDTH,
    SPEED_LIMIT, WINDOW_SIZE,
};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Turning {
    Left,
    Straight,
    Right,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Moving {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Borders {
    pub(crate) top: f32,
    pub(crate) right: f32,
    pub(crate) left: f32,
    pub(crate) bottom: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    Standard,
    Sport,
    TaxiVert,
    TaxiOrange,
     TaxiNoire,
     TaxiBleu,
     TaxiRouge,
}
// Ajuster les chances d'obtenir certaines voitures ici. Pourrait revoir et utiliser gen_range à la place.

#[derive(Clone, Debug)]
pub struct Car {
    pub x: f32,
    pub y: f32,
    pub index: usize,
    pub moving: Moving,
    pub vel: f32,
    pub turning: Turning,
    pub path: Path,
    pub direction: Direction,
    pub id: usize,
    time: SystemTime,
    pub model: Model,
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Car {
    pub fn new(direction: Direction, turning: Turning, id: usize) -> Car {
        let path = Path::new(&direction, &turning);
        let (x, y) = get_entry_coords(&path.sectors[0], &direction);
        Car {
            x,
            y,
            index: 0,
            moving: match &direction {
                Direction::North => Moving::Down,
                Direction::East => Moving::Left,
                Direction::South => Moving::Up,
                Direction::West => Moving::Right,
            },
            vel: 1.0,
            id,
            turning,
            path,
            direction,
            time: SystemTime::now(),
            model: match gen_range(0, 9) {
                0 => Model::TaxiVert,
                1 => Model::Sport,
                 2 => Model::TaxiNoire,
                 3 => Model::TaxiRouge,
                 4 => Model::TaxiBleu,
                 5 => Model::TaxiOrange,
                _ => Model::Standard,
            },
        }
    }

    /// ### move_car
    /// Déplacer la voiture dans Path ainsi que dans Car.x et Car.y.
    pub fn move_car(&mut self, all_cars: &[Car]) {
        self.move_in_path(all_cars);
        self.moving = self.sector(0).moving;
        self.change_pos(all_cars);

        // La voiture tourne à droite, aucune logique supplémentaire nécessaire.
        if self.turning == Turning::Right {
            self.accelerate(SCAN_DISTANCE);
            return;
        }

        // La voiture est encore en train d'entrer dans l'intersection.
        if self.index < 2 {
            return;
        }

        if self.turning == Turning::Straight && (3..=7).contains(&self.index) {
            self.sector_in_front(all_cars);
        }

        if self.index == 3 && self.sector_pos() > CLOSE_CALL_DISTANCE {
            self.check_passing(all_cars);
        }

        if self.turning == Turning::Left && (5..=7).contains(&self.index) {
            self.center_scan(all_cars);
        }

        // La voiture qui va tout droit a atteint l'autre côté de l'intersection.
        if self.index >= 8 {
            self.forward_scan(all_cars);
            return;
        }

        // Adjuster la position sur les axes x et y.
        self.adjust_position();

        // Envoyer des rayons sur une certaine distance et vérifier la présence de voitures.
        self.ray_casting(all_cars);

        // Scanner devant la voiture pour déterminer s'il est sûr d'accélérer ou s'il faut s'arrêter.
        self.forward_scan(all_cars);
    }

    pub fn accelerate(&mut self, distance: f32) {
        let x = if distance >= SCAN_DISTANCE {
            1.0
        } else {
            distance / SCAN_DISTANCE
        };
        let new_vel = (SPEED_LIMIT - self.vel) / FPS as f32 * x;
        if self.vel < SPEED_LIMIT {
            self.vel += new_vel;
        }
    }

    pub fn brake(&mut self, distance: f32) {
        let new_vel = self.vel - distance / SCAN_DISTANCE;
        if new_vel < 0.0 {
            return;
        }
        self.vel -= new_vel;
        if self.vel < 0.3 {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        self.vel = 0.0;
    }

   
    /// Modifier la position de la voiture. Elle ira plus vite s'il n'y a pas de voitures autour et plus lentement s'il y a trop de voitures autour.
    fn change_pos(&mut self, cars: &[Car]) {
        let x = match cars
            .iter()
            .filter(|c| self.id != c.id && self.calc_dist(c) < ACCELERATION_DISTANCE)
            .count()
        {
            0 => 1.05,
            1 => 1.00,
            _ => 0.90,
        };
        match self.moving {
            Moving::Up => self.y -= self.vel * MAX_VELOCITY * x,
            Moving::Right => self.x += self.vel * MAX_VELOCITY * x,
            Moving::Down => self.y += self.vel * MAX_VELOCITY * x,
            Moving::Left => self.x -= self.vel * MAX_VELOCITY * x,
        }
    }

    
    /// Obtenir la distance parcourue dans un Sector. Cela est utilisé pour résoudre les impasses.
    pub fn sector_pos(&self) -> f32 {
        match self.moving {
            Moving::Up => SECTOR_WIDTH - (self.y - self.sector(0).get_y() as f32 * SECTOR_WIDTH),
            Moving::Right => SECTOR_WIDTH - (self.sector(0).get_x() as f32 * SECTOR_WIDTH - self.x),
            Moving::Down => SECTOR_WIDTH - (self.sector(0).get_y() as f32 * SECTOR_WIDTH - self.y),
            Moving::Left => SECTOR_WIDTH - (self.x - self.sector(0).get_x() as f32 * SECTOR_WIDTH),
        }
    }


    /// Déplace la voiture le long de son propre `Path` en incrémentant `path.currentss.
/// S'arrête s'il y a une voiture dans le secteur devant.
    fn move_in_path(&mut self, cars: &[Car]) {
        if self.index + 2 > self.path.sectors.len() {
            return;
        }
        let car_ahead = cars.iter().any(|c| c.sector(0) == self.sector(1));

        let next = &self.sector(0);
        match self.moving {
            Moving::Up => {
                if self.update_up(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
            Moving::Right => {
                if self.update_right(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
            Moving::Down => {
                if self.update_down(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
            Moving::Left => {
                if self.update_left(next) {
                    if !car_ahead {
                        self.index += 1;
                    } else {
                        self.stop();
                    }
                }
            }
        }
    }

    // Fonctions auxiliaires `pour move_in_path`
    fn update_up(&self, next: &Sector) -> bool {
        self.y - self.vel * MAX_VELOCITY <= next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_right(&self, next: &Sector) -> bool {
        self.x + self.vel * MAX_VELOCITY >= next.get_x() as f32 * SECTOR_WIDTH
    }

    fn update_down(&self, next: &Sector) -> bool {
        self.y + self.vel * MAX_VELOCITY >= next.get_y() as f32 * SECTOR_WIDTH
    }

    fn update_left(&self, next: &Sector) -> bool {
        self.x - self.vel * MAX_VELOCITY <= next.get_x() as f32 * SECTOR_WIDTH
    }

    /// Mets à jour la direction de la voiture en fonction du secteur actuel dans `Path`.
    pub fn adjust_position(&mut self) {
        let previous = self.index - 1;
        let previous_sector = &self.path.sectors[previous];

        let sector = self.sector(0);

        if sector.get_x() != previous_sector.get_x() {
            self.y = SECTOR_WIDTH * sector.get_y() as f32;
        }

        if sector.get_y() != previous_sector.get_y() {
            self.x = SECTOR_WIDTH * sector.get_x() as f32;
        }
    }

   
    /// Get the sector of a `Car` specified by `n`.
    pub fn sector(&self, n: usize) -> Sector {
        self.path.sectors[self.index + n].clone()
    }

    
    /// Get the borders of a `Car`.
    pub fn borders(&self) -> Borders {
        Borders {
            top: self.y,
            right: self.x + SECTOR_WIDTH,
            bottom: self.y + SECTOR_WIDTH,
            left: self.x,
        }
    }

    pub fn add_time(&self, stats: &mut Statistics) {
        let duration = SystemTime::now().duration_since(self.time).unwrap();
        stats.set_time(duration.as_secs_f32());
    }

    /// ### is_done
    /// Checks if car has reached the end of their `Path`
    pub fn is_done(&self) -> bool {
        match self.moving {
            Moving::Up => self.borders().bottom <= 0.0,
            Moving::Right => self.borders().left >= WINDOW_SIZE as f32,
            Moving::Down => self.borders().top >= WINDOW_SIZE as f32,
            Moving::Left => self.borders().right <= 0.0,
        }
    }
}

fn get_entry_coords(p: &Sector, direction: &Direction) -> (f32, f32) {
    match direction {
        Direction::West => (
            SECTOR_WIDTH * p.get_x() as f32 - SECTOR_WIDTH,
            SECTOR_WIDTH * p.get_y() as f32,
        ),
        Direction::East => (
            SECTOR_WIDTH * p.get_x() as f32 + SECTOR_WIDTH,
            SECTOR_WIDTH * p.get_y() as f32,
        ),
        Direction::North => (
            SECTOR_WIDTH * p.get_x() as f32,
            SECTOR_WIDTH * p.get_y() as f32 - SECTOR_WIDTH,
        ),
        Direction::South => (
            SECTOR_WIDTH * p.get_x() as f32,
            SECTOR_WIDTH * p.get_y() as f32 + SECTOR_WIDTH,
        ),
    }
}
