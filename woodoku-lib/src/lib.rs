use anyhow::{anyhow, Ok, Result};
use rand::seq::SliceRandom;

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub data: Vec<bool>,
    pub available: bool,
}

impl Shape {
    fn new(data: Vec<bool>) -> Self {
        Self {
            data,
            available: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Woodoku {
    pub board: Vec<bool>,
    pub shapes_batch: Vec<Shape>,
    pub game_over: bool,
}

impl Default for Woodoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Woodoku {
    pub const BOARD_SIZE: usize = 81;
    const BOARD_SIDE_SIZE: usize = 9;
    const GRID_SIDE_SIZE: usize = 3;
    pub const SHAPES_BATCH_SIZE: usize = 3;
    pub const SHAPE_SIZE: usize = 25;
    const SHAPE_SIDE_SIZE: usize = 5;

    pub fn new() -> Self {
        Self {
            board: vec![false; Self::BOARD_SIZE],
            shapes_batch: Self::get_new_shapes_batch(),
            game_over: false,
        }
    }

    pub fn play_move(&self, shape_ix: usize, position: usize) -> Result<Self> {
        // Get shape from its index
        let shape = self
            .get_shape_if_not_used(shape_ix)
            .ok_or(anyhow!("Invalid move: shape already used"))?;

        // Validate move and fill overlapping slots
        let mut board = self.board.clone();
        Self::apply_move(&mut board, &shape.data, position)?;

        // Clear full rows, columns, grids
        Self::clear_indices(&mut board);

        // Update shapes batch
        let mut shapes_batch = self.shapes_batch.clone();
        Self::update_shapes_batch(&mut shapes_batch, shape_ix);

        let game_over = Self::is_game_over(&board, &shapes_batch);

        Ok(Self {
            board,
            shapes_batch,
            game_over,
        })
    }

    pub fn move_preview(&self, shape_ix: usize, position: usize) -> Result<Vec<bool>> {
        // Get shape from its index
        let shape = self
            .get_shape_if_not_used(shape_ix)
            .ok_or(anyhow!("Invalid move: shape already used"))?;

        // Validate move and fill overlapping slots
        let mut board = self.board.clone();
        Self::apply_move(&mut board, &shape.data, position)?;

        Ok(board)
    }

    pub fn get_placeable_shapes(board: &[bool], shapes_batch: &[Shape]) -> Vec<bool> {
        let mut placeable_shapes = vec![];
        for shape in shapes_batch {
            let mut shape_can_be_placed = false;
            if shape.available {
                for board_ix in 0..Self::BOARD_SIZE {
                    let mut board = board.to_owned();
                    if Self::apply_move(&mut board, &shape.data, board_ix).is_ok() {
                        shape_can_be_placed = true;
                        break;
                    }
                }
            }
            placeable_shapes.push(shape_can_be_placed);
        }
        placeable_shapes
    }

    pub fn get_indices_to_clear_with_duplicates(board: &[bool]) -> Vec<usize> {
        let mut indices_to_clear = vec![];
        Self::get_rows_indices_to_clear(board, &mut indices_to_clear);
        Self::get_columns_indices_to_clear(board, &mut indices_to_clear);
        Self::get_grids_indices_to_clear(board, &mut indices_to_clear);
        indices_to_clear
    }

    fn get_shape_if_not_used(&self, shape_ix: usize) -> Option<Shape> {
        let shape = self.shapes_batch[shape_ix].clone();

        if shape.available {
            Some(shape)
        } else {
            None
        }
    }

    fn is_game_over(board: &[bool], shapes_batch: &[Shape]) -> bool {
        Self::get_placeable_shapes(board, shapes_batch)
            .iter()
            .all(|placeable| !placeable)
    }

    fn apply_move(board: &mut [bool], shape: &[bool], position: usize) -> Result<()> {
        // Calculate which board slots are impacted by overlapping the shape
        let board_indices = Self::get_impacted_board_indices(shape, position)?;

        // Update board: fill slots
        for board_ix in board_indices {
            if board[board_ix] {
                return Err(anyhow!("Invalid move: shape overlapping"));
            } else {
                board[board_ix] = true;
            }
        }

        Ok(())
    }

    fn get_impacted_board_indices(shape: &[bool], position: usize) -> Result<Vec<usize>> {
        let mut board_indices = vec![];
        for shape_row in 0..Self::SHAPE_SIDE_SIZE {
            for shape_col in 0..Self::SHAPE_SIDE_SIZE {
                let shape_ix = shape_row * Self::SHAPE_SIDE_SIZE + shape_col;

                // Number of slots in the full rows above the row of `position`
                let q1 = (position / Self::BOARD_SIDE_SIZE) * Self::BOARD_SIDE_SIZE;
                // Number of slots in the full rows starting from the row of `position`
                // to the row above the one we are currently in
                let q2 = shape_row * Self::BOARD_SIDE_SIZE;
                // Number of slots in the row we are currently in before `position`
                let q3 = position % Self::BOARD_SIDE_SIZE;
                // Number of slots in the row we are currently in after `position`
                let q4 = shape_col;

                let board_ix = q1 + q2 + q3 + q4;

                if board_ix >= Self::BOARD_SIZE || q3 + q4 >= Self::BOARD_SIDE_SIZE {
                    if shape[shape_ix..shape_ix + Self::SHAPE_SIDE_SIZE - q4].contains(&true) {
                        return Err(anyhow!("Invalid move: shape out of range"));
                    }
                    continue;
                }

                if shape[shape_ix] {
                    board_indices.push(board_ix);
                }
            }
        }
        Ok(board_indices)
    }

    fn clear_indices(board: &mut [bool]) {
        let indices_to_clear = Self::get_indices_to_clear_with_duplicates(board);
        for ix_to_clear in indices_to_clear {
            board[ix_to_clear] = false;
        }
    }

    fn get_rows_indices_to_clear(board: &[bool], indices_to_clear: &mut Vec<usize>) {
        for row_ix in 0..Self::BOARD_SIDE_SIZE {
            let board_ix = row_ix * Self::BOARD_SIDE_SIZE;
            if board[board_ix..board_ix + Self::BOARD_SIDE_SIZE]
                .iter()
                .all(|slot| *slot)
            {
                indices_to_clear.extend(board_ix..board_ix + Self::BOARD_SIDE_SIZE);
            }
        }
    }

    fn get_columns_indices_to_clear(board: &[bool], indices_to_clear: &mut Vec<usize>) {
        for col_ix in 0..Self::BOARD_SIDE_SIZE {
            let board_indices = (0..Self::BOARD_SIDE_SIZE)
                .map(|i| (i * Self::BOARD_SIDE_SIZE) + col_ix)
                .collect::<Vec<usize>>();
            if board_indices.iter().all(|board_ix| board[*board_ix]) {
                indices_to_clear.extend(board_indices);
            }
        }
    }

    pub fn get_grid_indices() -> Vec<Vec<usize>> {
        let mut grids_indices = vec![];
        for grid_ix_0 in 0..Self::GRID_SIDE_SIZE {
            let q1 = grid_ix_0 * Self::BOARD_SIDE_SIZE * Self::GRID_SIDE_SIZE;
            for grid_ix_1 in 0..Self::GRID_SIDE_SIZE {
                let q2 = grid_ix_1 * Self::GRID_SIDE_SIZE;
                let mut grid_indices = vec![];
                for grid_row in 0..Self::GRID_SIDE_SIZE {
                    let q3 = grid_row * Self::BOARD_SIDE_SIZE;
                    for grid_col in 0..Self::GRID_SIDE_SIZE {
                        grid_indices.push(q1 + q2 + q3 + grid_col);
                    }
                }
                grids_indices.push(grid_indices)
            }
        }
        grids_indices
    }

    fn get_grids_indices_to_clear(board: &[bool], indices_to_clear: &mut Vec<usize>) {
        for grid_indices in Self::get_grid_indices() {
            if grid_indices.iter().all(|board_ix| board[*board_ix]) {
                indices_to_clear.extend(grid_indices);
            }
        }
    }

    fn update_shapes_batch(shapes_batch: &mut Vec<Shape>, used_shape_ix: usize) {
        shapes_batch[used_shape_ix].available = false;
        if shapes_batch.iter().all(|sb| !sb.available) {
            *shapes_batch = Self::get_new_shapes_batch();
        }
    }

    fn get_new_shapes_batch() -> Vec<Shape> {
        Self::get_all_possible_shapes()
            .choose_multiple(&mut rand::thread_rng(), Self::SHAPES_BATCH_SIZE)
            .map(|data| Shape::new((*data).clone()))
            .collect::<Vec<Shape>>()
    }

    fn get_all_possible_shapes() -> Vec<Vec<bool>> {
        let possible_shapes_file = include_str!("data/shapes.json");
        serde_json::from_str(possible_shapes_file).expect("Should read shapes file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_new_should_succeed() {
        // Arrange
        let all_shapes = Woodoku::get_all_possible_shapes();

        // Act
        let w = Woodoku::new();

        // Assert
        assert_eq!(all_shapes.len(), 57);
        assert!(w.board.iter().all(|slot| !slot));
        assert!(w.shapes_batch.iter().all(|shape| shape.available));
        assert!(w
            .shapes_batch
            .iter()
            .map(|shape| all_shapes.contains(&shape.data))
            .collect::<Vec<bool>>()
            .iter()
            .all(|is_contained| *is_contained));
    }

    #[test]
    fn fn_get_grids_indices_to_clear_should_succeed() {
        // Arrange
        let grid_0_indices = vec![0, 1, 2, 9, 10, 11, 18, 19, 20];
        let grid_1_indices = vec![3, 4, 5, 12, 13, 14, 21, 22, 23];
        let grid_2_indices = vec![6, 7, 8, 15, 16, 17, 24, 25, 26];
        let grid_3_indices = vec![27, 28, 29, 36, 37, 38, 45, 46, 47];
        let grid_4_indices = vec![30, 31, 32, 39, 40, 41, 48, 49, 50];
        let grid_5_indices = vec![33, 34, 35, 42, 43, 44, 51, 52, 53];
        let grid_6_indices = vec![54, 55, 56, 63, 64, 65, 72, 73, 74];
        let grid_7_indices = vec![57, 58, 59, 66, 67, 68, 75, 76, 77];
        let grid_8_indices = vec![60, 61, 62, 69, 70, 71, 78, 79, 80];
        let grids_indices = vec![
            grid_0_indices,
            grid_1_indices,
            grid_2_indices,
            grid_3_indices,
            grid_4_indices,
            grid_5_indices,
            grid_6_indices,
            grid_7_indices,
            grid_8_indices,
        ];

        // Act, Assert
        for grid_indices in grids_indices {
            let mut board = vec![false; Woodoku::BOARD_SIZE];
            for ix in &grid_indices {
                board[*ix] = true;
            }
            let mut indices_to_clear = vec![];
            Woodoku::get_grids_indices_to_clear(&board, &mut indices_to_clear);
            assert_eq!(indices_to_clear, grid_indices);
        }
    }

    #[test]
    fn fn_play_move_should_succeed_place_single_square_sequentially_everywhere() {
        // Arrange
        let mut w = Woodoku::new();
        let shape_0 = vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false,
        ];

        // Act, Assert
        for board_ix in 0..Woodoku::BOARD_SIZE {
            let mut mv = String::from("0");
            mv.push_str(&board_ix.to_string());
            w.shapes_batch = vec![
                Shape {
                    data: shape_0.clone(),
                    available: true,
                },
                Shape {
                    data: vec![],
                    available: false,
                },
                Shape {
                    data: vec![],
                    available: false,
                },
            ];
            w = w
                .play_move(0, board_ix)
                .expect(&format!("Move should be valid"));

            if (board_ix + 1) % Woodoku::BOARD_SIDE_SIZE == 0 {
                // If we are at the end of a row, the board should be empty
                assert!(w.board.iter().all(|slot| !slot));
            } else {
                // Else, the current row should be full up until where we are
                // and the rest should be empty
                assert!(w.board
                    [0..(board_ix / Woodoku::BOARD_SIDE_SIZE) * Woodoku::BOARD_SIDE_SIZE]
                    .iter()
                    .all(|slot| !slot));
                assert!(
                    w.board[(board_ix / Woodoku::BOARD_SIDE_SIZE) * Woodoku::BOARD_SIDE_SIZE
                        ..board_ix + 1]
                        .iter()
                        .all(|slot| *slot)
                );
                assert!(w.board[board_ix + 1..].iter().all(|slot| !slot));
            }
        }
    }

    #[test]
    fn fn_play_move_should_succeed_fill_grid_with_two_shapes() {
        // Arrange
        let mut w = Woodoku::new();
        let shape_0 = vec![
            true, true, true, false, false, true, false, false, false, false, true, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
        ];
        let shape_1 = vec![
            true, true, false, false, false, true, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
        ];
        w.shapes_batch = vec![
            Shape {
                data: shape_0,
                available: true,
            },
            Shape {
                data: shape_1,
                available: true,
            },
            Shape {
                data: vec![],
                available: false,
            },
        ];

        // Act, Assert
        w = w.play_move(0, 0).expect(&format!("Move should be valid"));
        assert!(w.board[0..3].iter().all(|slot| *slot));
        assert!(w.board[9]);
        assert!(w.board[18]);

        w = w.play_move(1, 10).expect(&format!("Move should be valid"));
        assert!(w.board[0..3].iter().all(|slot| !slot));
        assert!(w.board[9..12].iter().all(|slot| !slot));
        assert!(w.board[18..21].iter().all(|slot| !slot));
    }

    #[test]
    fn fn_play_move_should_game_over() {
        // Arrange
        let mut w = Woodoku::new();
        let shape_0 = vec![
            true, true, true, false, false, true, false, false, false, false, true, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
        ];

        w.board = vec![false; Woodoku::BOARD_SIZE];
        // Place a block on every secondo index on the board
        for board_ix in 0..Woodoku::BOARD_SIZE {
            if board_ix % 2 == 0 {
                w.board[board_ix] = true;
            }
        }
        // Free the blocks needed to place the shape in pos 0
        for ix in Woodoku::get_impacted_board_indices(&shape_0, 0).unwrap() {
            w.board[ix] = false;
        }
        // Free the blocks needed to place the shape in pos 49
        for ix in Woodoku::get_impacted_board_indices(&shape_0, 49).unwrap() {
            w.board[ix] = false;
        }

        w.shapes_batch = vec![
            Shape {
                data: shape_0.clone(),
                available: true,
            },
            Shape {
                data: shape_0.clone(),
                available: true,
            },
            Shape {
                data: shape_0,
                available: true,
            },
        ];

        // Act, Assert
        w = w.play_move(0, 0).expect(&format!("Move should be valid"));
        assert!(!w.game_over);
        w = w.play_move(1, 49).expect(&format!("Move should be valid"));
        assert!(w.game_over);
    }
}
