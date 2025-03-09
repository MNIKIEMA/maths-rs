use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
struct Matrix<T> {
    data: Vec<Vec<T>>
}

struct  Axis{
    axis: usize
}


impl<T> Matrix<T> {
    fn new(data: Vec<Vec<T>>) -> Self {
        Matrix { data }
    }

    fn shape(&self) -> (usize, usize) {
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


fn main() {
    let matrix = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let shape = matrix.shape();
    let first_element = matrix[0][0];
    println!("Matrix {:?} with shape {:?} and first element {}", matrix, shape, first_element)
}