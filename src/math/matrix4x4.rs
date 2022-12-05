use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4 {
    pub matrix: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(row1: [f32; 4], row2: [f32; 4], row3: [f32; 4], row4: [f32; 4]) -> Matrix4x4 {
        Matrix4x4 {
            matrix: [row1, row2, row3, row4],
        }
    }

    pub fn from_array(array: [[f32; 4]; 4]) -> Matrix4x4 {
        Matrix4x4 { matrix: array }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn empty() -> Matrix4x4 {
        Matrix4x4::new(
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        )
    }

    pub fn transpose(self) -> Matrix4x4 {
        let mut result = Matrix4x4::empty();

        for i in 0..4 {
            for j in 0..4 {
                result.matrix[i][j] = self.matrix[j][i];
            }
        }

        result
    }

    pub fn dot(self, other: Matrix4x4) -> f32 {
        let mut result = 0.0;

        for i in 0..4 {
            for j in 0..4 {
                result += self.matrix[i][j] * other.matrix[i][j];
            }
        }

        result
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row <= 3);
        assert!(col <= 3);

        self.matrix[row][col] = value;
    }
}

impl ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        //https://en.wikipedia.org/wiki/Matrix_multiplication_algorithm#Iterative_algorithm
        let first_mat = self.matrix;
        let second_mat = rhs.matrix;

        let mut output = Matrix4x4::empty().matrix;

        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += first_mat[i][k] * second_mat[k][j];
                }
                output[i][j] = sum;
            }
        }

        Matrix4x4::from_array(output)
    }
}
