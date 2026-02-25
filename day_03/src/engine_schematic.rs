mod number;
mod parser;

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
        let mut sum = 0u32;

        for row in 0..self.rows {
            for col in 0..self.columns {
                if self.vec2d[row][col] != '*' {
                    continue;
                }
                let adjacent_numbers: Vec<&Number> = numbers
                    .iter()
                    .filter(|n| {
                        n.position()
                            .adjacent_cells(self.rows, self.columns)
                            .contains(&(row, col))
                    })
                    .collect();

                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers[0].value() * adjacent_numbers[1].value();
                }
            }
        }
        sum
    }

    fn is_adjacent_to_symbol(&self, number: &Number) -> bool {
        number
            .position()
            .adjacent_cells(self.rows, self.columns)
            .iter()
            .any(|&(r, c)| {
                let ch = self.vec2d[r][c];
                !ch.is_ascii_digit() && ch != '.'
            })
    }
}
