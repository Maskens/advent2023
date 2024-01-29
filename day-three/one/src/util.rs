#[derive(Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize
}


pub fn check_is_symbol(c: char) -> bool {
    return c != '.' && !c.is_ascii_digit();
}

pub enum Direction {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Right,
    Left,
}

impl Direction { 
    pub fn get_dir(&self) -> (isize, isize) {
        match *self {
            Direction::Up => (0, -1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0)
        }
    }
}

