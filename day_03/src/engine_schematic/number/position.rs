#[derive(Debug, PartialEq, Eq, Hash)]
pub(in crate::engine_schematic) struct Position {
    row: usize,
    column_start: usize,
    column_end: usize,
}

impl Position {
    pub(in crate::engine_schematic) fn new(
        row: usize,
        column_start: usize,
        column_end: usize,
    ) -> Self {
        Self {
            row,
            column_start,
            column_end,
        }
    }
    /// Returns all (row, column) cells adjacent to this number position,
    /// including diagonals, clamped to the given grid dimensions.
    pub(in crate::engine_schematic) fn adjacent_cells(
        &self,
        rows: usize,
        columns: usize,
    ) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        let row_start = self.row.saturating_sub(1);
        let row_end = (self.row + 1).min(rows - 1);
        let col_start = self.column_start.saturating_sub(1);
        let col_end = (self.column_end + 1).min(columns - 1);

        for r in row_start..=row_end {
            for c in col_start..=col_end {
                // Skip cells that are part of the number itself
                if r == self.row && c >= self.column_start && c <= self.column_end {
                    continue;
                }
                cells.push((r, c));
            }
        }
        cells
    }
}
