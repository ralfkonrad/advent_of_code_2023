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

    /// Returns an iterator over all (row, column) cells adjacent to this number position,
    /// including diagonals, clamped to the given grid dimensions.
    /// Zero allocations — yields cells lazily.
    pub(in crate::engine_schematic) fn adjacent_cells(
        &self,
        rows: usize,
        columns: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let row_start = self.row.saturating_sub(1);
        let row_end = (self.row + 1).min(rows - 1);
        let col_start = self.column_start.saturating_sub(1);
        let col_end = (self.column_end + 1).min(columns - 1);

        (row_start..=row_end).flat_map(move |r| {
            (col_start..=col_end)
                .filter(move |&c| {
                    // Skip cells that are part of the number itself
                    !(r == self.row && c >= self.column_start && c <= self.column_end)
                })
                .map(move |c| (r, c))
        })
    }
}
