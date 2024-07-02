use crate::circulation::car::Turning;
use crate::circulation::{Direction, Moving};

#[derive(Eq, Clone, Debug)]
pub struct Sector {
    x: usize,
    y: usize,
    pub moving: Moving, //    status: SectorStatus
}

impl PartialEq for Sector {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl Sector {
    pub fn new(x: usize, y: usize, moving: Moving) -> Sector {
        Sector {
            x,
            y,
            moving, //status: SectorStatus::Free
        }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Path {
    pub sectors: Vec<Sector>,
}

impl Path {
    pub fn new(direction: &Direction, turning: &Turning) -> Path {
        Path {
            sectors: match turning {
                Turning::Left => left_turn(direction),
                Turning::Straight => go_straight(direction),
                Turning::Right => right_turn(direction),
            },
        }
    }
}

fn left_turn(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(5, 0, Moving::Down),   // Entrée
            Sector::new(5, 6, Moving::Right),  // Turning-point
            Sector::new(11, 6, Moving::Right), // Sortir
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 5, Moving::Left), // Entrée
            Sector::new(5, 5, Moving::Down),  // Turning-point
            Sector::new(5, 11, Moving::Down), // Sortir
        ]),
        Direction::South => get_path(vec![
            Sector::new(6, 11, Moving::Up),  // Entrée
            Sector::new(6, 5, Moving::Left), // Turning-point
            Sector::new(0, 5, Moving::Left), // Sortir
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 6, Moving::Right), // Entrée
            Sector::new(6, 6, Moving::Up),    // Turning-point
            Sector::new(6, 0, Moving::Up),    // Sortir
        ]),
    }
}

fn go_straight(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(4, 0, Moving::Down),  // Entrée
            Sector::new(4, 5, Moving::Down),  // Point-median
            Sector::new(4, 11, Moving::Down), // Sortir
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 4, Moving::Left), // Entrée
            Sector::new(5, 4, Moving::Left),  // Point-median
            Sector::new(0, 4, Moving::Left),  // Sortir
        ]),
        Direction::South => get_path(vec![
            Sector::new(7, 11, Moving::Up), // Entrée
            Sector::new(7, 5, Moving::Up),  // Point-median
            Sector::new(7, 0, Moving::Up),  // Sortir
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 7, Moving::Right),  // Entrée
            Sector::new(5, 7, Moving::Right),  // Point-median
            Sector::new(11, 7, Moving::Right), // Sortir
        ]),
    }
}

fn right_turn(direction: &Direction) -> Vec<Sector> {
    match direction {
        Direction::North => get_path(vec![
            Sector::new(3, 0, Moving::Down), // Entrée
            Sector::new(3, 3, Moving::Left), // Tournant
            Sector::new(0, 3, Moving::Left), // Sortir
        ]),
        Direction::East => get_path(vec![
            Sector::new(11, 3, Moving::Left), // Entrée
            Sector::new(8, 3, Moving::Up),    // Tournant
            Sector::new(8, 0, Moving::Up),    // Sortir
        ]),
        Direction::South => get_path(vec![
            Sector::new(8, 11, Moving::Up),    // Entrée
            Sector::new(8, 8, Moving::Right),  // Tournant
            Sector::new(11, 8, Moving::Right), // Sortir
        ]),
        Direction::West => get_path(vec![
            Sector::new(0, 8, Moving::Right), // Entrée
            Sector::new(3, 8, Moving::Down),  // Tournant
            Sector::new(3, 11, Moving::Down), // Sortir
        ]),
    }
}

// Helper function to get all sectors in the path
fn get_path(sectors: Vec<Sector>) -> Vec<Sector> {
    let mut path = vec![sectors[0].clone()];
    let mut x: usize = sectors[0].x;
    let mut y: usize = sectors[0].y;
    while x != sectors[1].x || y != sectors[1].y {
        if x < sectors[1].x {
            x += 1;
        }
        if x > sectors[1].x {
            x -= 1;
        }
        if y < sectors[1].y {
            y += 1;
        }
        if y > sectors[1].y {
            y -= 1;
        }

        path.push(Sector::new(x, y, sectors[0].clone().moving));
    }
    while x != sectors[2].x || y != sectors[2].y {
        if x < sectors[2].x {
            x += 1;
        }
        if x > sectors[2].x {
            x -= 1;
        }
        if y < sectors[2].y {
            y += 1;
        }
        if y > sectors[2].y {
            y -= 1;
        }
        path.push(Sector::new(x, y, sectors[1].clone().moving));
    }
    path
}
