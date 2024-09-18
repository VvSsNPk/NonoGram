use std::fmt::Display;

use self::clue::Line;

pub mod clue;
pub mod solver_nono;
pub mod state;

#[derive(Debug, Clone)]
pub struct NonoGram {
    map: Vec<Line>,
    size: (u32, u32),
    is_hex: bool,
    multi_color: bool,
    pub is_question_mark: bool,
}

impl NonoGram {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            map: Vec::new(),
            size: (x, y),
            is_hex: false,
            multi_color: false,
            is_question_mark: false,
        }
    }

    pub fn add(&mut self, line: Line) {
        self.map.push(line)
    }

    pub fn get_map(&self) -> Vec<Line> {
        self.map.clone()
    }

    pub fn get_size(&self) -> usize {
        let mut ans = 0;
        if self.is_hex {
            ans = (3 * (self.size.1 * self.size.1) - 3 * (self.size.1) + 1) as usize;
        } else {
            ans = (self.size.0 * self.size.1) as usize;
        }

        if self.multi_color {
            2 * ans
        } else {
            ans
        }
    }

    pub fn get_height(&self) -> usize {
        self.size.0 as usize
    }

    pub fn get_width(&self) -> usize {
        self.size.1 as usize
    }

    pub fn set_hex(&mut self) {
        self.is_hex = true;
    }

    pub fn is_hex(&self) -> bool {
        self.is_hex
    }
    pub fn get_hex_sizes(&self) -> Vec<usize> {
        let mut answer = Vec::new();
        if self.is_hex {
            let size = (2 * self.size.0 - 1) as usize;
            for i in 0..size {
                let line = self.map.get(i).unwrap().get_line_size();
                answer.push(line);
            }
        }
        answer
    }
    pub fn set_multi_color(&mut self) {
        self.multi_color = true;
    }
    pub fn is_multi_color(&self) -> bool {
        self.multi_color
    }
}

impl Display for NonoGram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.size)?;
        for i in &self.map {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}
