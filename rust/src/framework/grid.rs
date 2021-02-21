#[derive(Debug)]
pub struct Grid {
    rows: u32,
    columns: u32,
    grid: Vec<Vec<i32>>,
}

impl Grid {
    pub fn create_grid(rows: u32, columns: u32) -> Grid {
        // 2D arr with rows and columns
        let mut grid = vec![vec![0; 2]; 2];
        for (i, row) in grid.iter_mut().enumerate() {
            for (j, column) in grid.iter_mut().enumerate() {

            }
        }
        Grid {
            rows,
            columns,
            grid
        }
    }
}

// #[test]
// fn create_grid() {
//     let grid = Grid {
//         rows: 1,
//         columns: 1,
//     };
//     assert_eq!(grid.rows, 1);
//     assert_eq!(grid.columns, 1);
// }

#[test]
fn create_grid_associated_func() {
    let grid = Grid::create_grid(1, 1);
    assert_eq!(grid.rows, 1);
    assert_eq!(grid.columns, 1);
}