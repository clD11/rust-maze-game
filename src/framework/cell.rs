use std::collections::HashMap;

#[derive(Debug)]
struct Cell {
    row: u32,
    column: u32,
    linked_cells: HashMap<Cell, bool>
}