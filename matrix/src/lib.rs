use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

#[derive(Debug)]
pub struct COOMatrix<T> {
    data: Vec<Tuple<T>>
}

#[derive(Debug)]
pub struct Tuple<T> {
    row: usize,
    col: usize,
    value: T,
}


struct Axis {
    axis: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Matrix { data }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.data.len(), self.data.get(0).map_or(0, |row| row.len()))
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}


impl<T> COOMatrix<T> {
    pub fn new() -> Self {
        COOMatrix { data: Vec::new() }
    }

    pub fn push(&mut self, row: usize, col: usize, value: T) {
        self.data.push(Tuple { row, col, value });
    }

    pub fn data(&self) -> &Vec<Tuple<T>> {
        &self.data
    }
}