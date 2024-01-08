#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position {
    row: usize,
    column_start: usize,
    column_end: usize,
}

impl Position {
    pub fn new(row: usize, column_start: usize, column_end: usize) -> Self {
        Self {
            row,
            column_start,
            column_end,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column_start(&self) -> usize {
        self.column_start
    }

    pub fn column_end(&self) -> usize {
        self.column_end
    }
}
