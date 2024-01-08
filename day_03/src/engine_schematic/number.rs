use ndarray::{ArrayBase, Dim, ViewRepr};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Number {
    pub value: u32,
}

pub type Numbers = Vec<Number>;

impl Number {
    pub fn get_numbers(engine_schematic: &super::EngineSchematic) -> Numbers {
        let mut numbers = Vec::<Number>::new();
        for row in engine_schematic.array2d.rows() {
            numbers.append(get_numbers_from_row(&row))
        }
        todo!();
    }
}

fn get_numbers_from_row<'a>(
    row: &'a ArrayBase<ViewRepr<&char>, Dim<[usize; 1]>>,
) -> &'a mut Vec<Number> {
    row.into_iter()
}
