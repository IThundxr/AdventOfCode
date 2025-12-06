use itertools::Itertools;
use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    data: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let bytes = input.lines().map(str::as_bytes).collect_vec();

        let width = bytes[0].len() as i32;
        let height = bytes.len() as i32;
        let data = bytes.concat();

        Grid {
            width,
            height,
            data,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(width: i32, height: i32, default: T) -> Self {
        Grid {
            width,
            height,
            data: vec![default; (width * height) as usize],
        }
    }

    pub fn contains(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.data[(self.width * y + x) as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        &mut self.data[(self.width * y + x) as usize]
    }
}
