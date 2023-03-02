use std::io::{self, Write};

use crate::{grid::Grid, shape::Shape};

fn process_shape(shape: &Shape, starting_position_x: usize, grid: &mut Grid) {
    let mut current_height = grid.get_height();

    #[allow(unused_comparisons)]
    #[allow(clippy::absurd_extreme_comparisons)]
    while current_height >= 0 {
        let window = get_window(grid, shape, starting_position_x, current_height);

        for point in shape.get_points() {
            if window[point.1][point.0].is_some() {
                let mut temp_highest = grid.get_height();
                for point in shape.get_points() {
                    grid.set_point(point.0 + starting_position_x, point.1 + current_height + 1);
                    if point.1 + current_height > temp_highest {
                        temp_highest = point.1 + current_height;
                    }
                }

                for i in (0..grid.get_height()).rev() {
                    if grid.get_grid_row(i).iter().all(|cell| cell.is_some()) {
                        grid.remove_line(i);
                    }
                }
                return;
            }
        }

        if current_height == 0 {
            let mut temp_highest = grid.get_height();
            for point in shape.get_points() {
                grid.set_point(point.0 + starting_position_x, point.1 + current_height);
                if point.1 + current_height > temp_highest {
                    temp_highest = point.1 + current_height;
                }
            }

            for i in (0..grid.get_height()).rev() {
                if grid.get_grid_row(i).iter().all(|cell| cell.is_some()) {
                    grid.remove_line(i);
                }
            }
            return;
        }

        current_height -= 1;
    }
}

/// Returns the squared area around the shape.
fn get_window(
    grid: &Grid,
    shape: &Shape,
    x_position: usize,
    y_position: usize,
) -> Vec<Vec<Option<()>>> {
    let subgrid = &grid.get_grid()[y_position..y_position + shape.get_height()];
    subgrid
        .iter()
        .map(|row| row[x_position..x_position + shape.get_width()].to_owned())
        .collect()
}

fn get_line_elements(line: String) -> Vec<String> {
    line.split(',').map(|element| element.to_owned()).collect()
}

/// Calculates the total height of the shape stack.
fn calculate_height(line: String) -> usize {
    let mut grid = Grid::new();
    for element in get_line_elements(line) {
        let mut element = element.chars();

        let letter = element.next().unwrap();
        let starting_position_x = char::to_digit(element.next().unwrap(), 10).unwrap() as usize;

        let shape = Shape::new(letter);

        process_shape(&shape, starting_position_x, &mut grid);
    }

    // println!("{:?}", grid);
    grid.get_height() + 1
}

pub fn output_height(line: String, stdout: &mut io::StdoutLock) {
    stdout
        .write_fmt(format_args!("{},", calculate_height(line)))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::game::calculate_height;

    #[test]
    fn example_one() {
        let line = String::from("I0,I4,Q8");

        assert_eq!(calculate_height(line), 1);
    }

    #[test]
    fn example_two() {
        let line = String::from("T1,Z3,I4");

        assert_eq!(calculate_height(line), 4);
    }

    #[test]
    fn example_three() {
        let line = String::from("Q0,I2,I6,I0,I6,I6,Q2,Q4");

        assert_eq!(calculate_height(line), 3);
    }
}
