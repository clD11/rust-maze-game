use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Cell {
    row: u32,
    column: u32,
    north: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
    mark: bool,
    linked_cells: HashMap<Cell, bool>,
}

impl Cell {

    pub fn create_cell(row: u32,
                       column: u32,
                       north: Option<Box<Cell>>,
                       south: Option<Box<Cell>>,
                       east: Option<Box<Cell>>,
                       west: Option<Box<Cell>>) -> Cell {
        Cell {
            row,
            column,
            north,
            south,
            east,
            west,
            mark: false,
            linked_cells: HashMap::new()
        }
    }

    pub fn has_link(self, cell: &Cell) -> bool {
       self.linked_cells.contains_key(&cell)
   }
}

impl Eq for Cell {}

impl PartialEq<Cell> for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Hash for Cell {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.column.hash(state);
    }
}

#[test]
fn should_be_equal() {
    let expected = Cell::create_cell(1, 2, None, None, None, None);
    let actual = Cell::create_cell(1, 2, None,None, None, None);
    assert_eq!(actual, expected);
}

#[test]
fn should_not_be_equal() {
    let expected = Cell::create_cell(1, 1, None,None, None, None);
    let actual = Cell::create_cell(1, 2, None,None, None, None);
    assert_ne!(actual, expected);
}

#[test]
fn should_not_has_link() {
    let cell1 = Cell::create_cell(1, 1, None, None, None, None);
    let cell2 = Cell::create_cell(1, 1, None, None, None, None);
    assert_eq!(cell1.has_link(&cell2), false);
}