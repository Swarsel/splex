use std::fmt::Debug;

pub struct SymMat<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> SymMat<T> {
    fn get_index(&self, row: usize, col: usize) -> usize {
        debug_assert!(row < self.size);
        debug_assert!(col < self.size);

        let row = row as isize;
        let col = col as isize;

        if row <= col {
            (row * self.size as isize + col - row * (row - 1) / 2 - row) as usize
        } else {
            (col * self.size as isize + row - col * (col - 1) / 2 - col) as usize
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let index = self.get_index(row, col);
        &self.data[index]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let index = self.get_index(row, col);
        &mut self.data[index]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.get_index(row, col);
        self.data[index] = value;
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T: Clone> SymMat<T> {
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size,
        }
    }
}

impl<T: Clone + Copy> SymMat<T> {
    pub fn get_row(&self, row: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);
        for col in 0..self.size {
            result.push(self.get(row, col).clone());
        }
        result
    }

    pub fn get_col(&self, col: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);
        for row in 0..self.size {
            result.push(self.get(row, col).clone());
        }
        result
    }
}

impl<T: Default> SymMat<T> {
    pub fn new(size: usize) -> Self {
        let num_fields = size * (size + 1) / 2;
        let mut data = Vec::with_capacity(num_fields);
        for _ in 0..num_fields {
            data.push(T::default());
        }

        Self { data, size }
    }
}

impl<T: Debug> SymMat<T> {
    pub fn print(&self) {
        for row in 0..self.size {
            for col in 0..self.size {
                print!("{:?} ", self.get(row, col));
            }
            println!();
        }
    }
}

impl SymMat<bool> {
    // change to use formatter as in Debug trait
    pub fn print_block(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size {
            for col in 0..self.size {
                if *self.get(row, col) {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl std::ops::BitXor for &SymMat<bool> {
    type Output = SymMat<bool>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.size, rhs.size);

        let mut result = SymMat::new(self.size);

        for row in 0..self.size {
            for col in 0..self.size {
                let value = *self.get(row, col) ^ *rhs.get(row, col);
                result.set(row, col, value);
            }
        }

        result
    }
}
