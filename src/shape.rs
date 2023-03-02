pub struct Shape {
    letter: char,
    points: Vec<(usize, usize)>,
}

impl Shape {
    pub fn new(letter: char) -> Self {
        let points = match letter {
            'Q' => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            'Z' => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
            'S' => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
            'T' => vec![(1, 0), (0, 1), (1, 1), (2, 1)],
            'I' => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            'L' => vec![(0, 0), (1, 0), (0, 1), (0, 2)],
            'J' => vec![(0, 0), (1, 0), (1, 1), (1, 2)],
            _ => vec![],
        };

        Self { letter, points }
    }

    pub fn get_height(&self) -> usize {
        match self.letter {
            'Q' | 'Z' | 'S' | 'T' => 2,
            'I' => 1,
            'L' | 'J' => 3,
            _ => 0,
        }
    }

    pub fn get_width(&self) -> usize {
        match self.letter {
            'Q' | 'L' | 'J' => 2,
            'Z' | 'S' | 'T' => 3,
            'I' => 4,
            _ => 0,
        }
    }

    pub fn translate_x_by(&mut self, x: usize) {
        for i in 0..self.points.len() {
            let point = self.points[i];
            let point = (point.0 + x, point.1);
            self.points[i] = point;
        }
    }

    pub fn get_points(&self) -> &[(usize, usize)] {
        &self.points
    }
}

#[cfg(test)]
mod tests {
    use super::Shape;

    #[test]
    fn get_new_shape() {
        let letter = 'S';
        let shape = Shape::new(letter);

        assert_eq!(shape.get_points(), &[(0, 0), (1, 0), (1, 1), (2, 1)]);
    }
}
