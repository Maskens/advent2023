use crate::util::Pos;
use crate::util::Direction;
use crate::util::check_is_symbol;
use std::ops::Add;


pub struct Grid {
    val: Vec<Vec<char>>
}

impl Grid {
    pub fn new() -> Self {
        Self {
            val: vec![]
        }
    }

    pub fn width(&self) -> usize {
        self.val.get(0).map_or(0, |row| row.len())
    }

    pub fn height(&self) -> usize {
        self.val.len()    
    }

    pub fn find_char(&self, pos: Pos) -> Option<char> {
        match self.val.get(pos.y as usize) {
            Some(row) => {
                match row.get(pos.x as usize) {
                    Some(c) => Some(c.clone()),
                    None => None
                }
            },
            None => None
        }
    }

    pub fn find_char_offset(&self, pos: &Pos, dir: Direction) -> Option<char> {
        let offset = Pos { x: pos.x.add(dir.get_dir().0), y: pos.y.add(dir.get_dir().1)};

        return self.find_char(offset);
    }

    pub fn populate(&mut self, string: &str) {
        for line in string.lines() {
            let chars = line.chars();

            let mut l = vec![];

            for c in chars {
                l.push(c);
            }

            self.val.push(l);
        }
    }

    pub fn has_adjacent_symbol(&self, pos: Pos) -> bool {

        if pos.y > 0 {
            if let Some(c) = self.find_char_offset(&pos, Direction::Up) {
                if check_is_symbol(c) {
                    return true;
                }
            }
            if let Some(c) = self.find_char_offset(&pos, Direction::UpLeft) {
                if check_is_symbol(c) {
                    return true;
                }
            }

            if let Some(c) = self.find_char_offset(&pos, Direction::UpRight) {
                if check_is_symbol(c) {
                    return true;
                }
            }
        }

        // Check right
        if let Some(c) = self.find_char_offset(&pos, Direction::Right) {
            if check_is_symbol(c) {
                return true;
            }
        }


        if let Some(c) = self.find_char_offset(&pos, Direction::Down) {
            if check_is_symbol(c) {
                return true;
            }
        }

        if let Some(c) = self.find_char_offset(&pos, Direction::DownRight) {
            if check_is_symbol(c) {
                return true;
            }
        }

        if let Some(c) = self.find_char_offset(&pos, Direction::DownLeft) {
            if check_is_symbol(c) {
                return true;
            }
        }
        if let Some(c) = self.find_char_offset(&pos, Direction::Left) {
            if check_is_symbol(c) {
                return true;
            }
        }

        false
    }

    pub fn print(&self) {
        for line in &self.val {
            for c in line {
                print!("{}", c);
            }
            print!("\n");
        }
    }
}
