use crate::util::eq_f32;

pub struct Matrix4 {
    pub matrix: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn get(self: &Self, x: usize, y: usize) -> f32 {
        self.matrix[x][y]
    }

    pub fn new(matrix: [[f32; 4]; 4]) -> Self {
        Matrix4 { matrix }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_and_inspect_a_4_matrix() {
        let matrix = Matrix4::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        let tests = vec![
            (0, 0, 1.),
            (0, 3, 4.),
            (1, 0, 5.5),
            (1, 2, 7.5),
            (2, 2, 11.),
            (3, 0, 13.5),
            (3, 2, 15.5),
        ];

        for (x, y, result) in tests {
            assert_eq!(matrix.get(x, y), result);
        }
    }
}
