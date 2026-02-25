mod number;
mod parser;

use std::collections::HashMap;

use number::Number;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct EngineSchematic {
    vec2d: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}

impl EngineSchematic {
    pub(crate) fn parse(input: &str) -> Self {
        let array2d = parser::input_to_2darray(input);
        let rows = array2d.len();
        let columns = array2d[0].len();
        Self {
            vec2d: array2d,
            rows,
            columns,
        }
    }

    pub(crate) fn get_part_numbers(&self) -> Vec<u32> {
        Number::get_numbers(self)
            .iter()
            .filter(|n| self.is_adjacent_to_symbol(n))
            .map(|n| n.value())
            .collect()
    }

    pub(crate) fn get_gear_ratios_sum(&self) -> u32 {
        let numbers = Number::get_numbers(self);

        // Build a map from each cell to the indices of numbers adjacent to it.
        // This lets us look up adjacent numbers for any '*' cell in O(1).
        let mut cell_to_numbers: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        for (idx, number) in numbers.iter().enumerate() {
            for cell in number.position().adjacent_cells(self.rows, self.columns) {
                cell_to_numbers.entry(cell).or_default().push(idx);
            }
        }

        // For each '*' cell, check if exactly 2 numbers are adjacent.
        let mut sum = 0u32;
        for row in 0..self.rows {
            for col in 0..self.columns {
                if self.vec2d[row][col] != '*' {
                    continue;
                }
                if let Some(adjacent) = cell_to_numbers.get(&(row, col))
                    && adjacent.len() == 2
                {
                    sum += numbers[adjacent[0]].value() * numbers[adjacent[1]].value();
                }
            }
        }
        sum
    }

    fn is_adjacent_to_symbol(&self, number: &Number) -> bool {
        number
            .position()
            .adjacent_cells(self.rows, self.columns)
            .any(|(r, c)| {
                let ch = self.vec2d[r][c];
                !ch.is_ascii_digit() && ch != '.'
            })
    }
}
