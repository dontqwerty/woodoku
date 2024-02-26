use pyo3::{exceptions::PyTypeError, prelude::*};
use woodoku_lib::Woodoku;

#[pyclass]
pub struct WoodokuPy(Woodoku);

#[pymethods]
impl WoodokuPy {
    #[new]
    fn new() -> Self {
        Self(Woodoku::new())
    }

    #[getter]
    fn board(&self) -> Vec<usize> {
        self.0
            .board
            .iter()
            .map(|slot| if *slot { 1 } else { 0 })
            .collect()
    }

    #[getter]
    fn shapes_batch(&self) -> Vec<Vec<usize>> {
        self.0
            .shapes_batch
            .iter()
            .map(|shape| {
                if shape.to_be_placed {
                    shape
                        .data
                        .iter()
                        .map(|data| if *data { 1 } else { 0 })
                        .collect()
                } else {
                    vec![0; self.shape_size()]
                }
            })
            .collect()
    }

    #[getter]
    fn game_over(&self) -> bool {
        self.0.game_over
    }

    #[getter]
    fn board_size(&self) -> usize {
        Woodoku::BOARD_SIZE
    }

    #[getter]
    fn shapes_batch_size(&self) -> usize {
        Woodoku::SHAPES_BATCH_SIZE
    }

    #[getter]
    fn shape_size(&self) -> usize {
        Woodoku::SHAPE_SIZE
    }

    fn play_move(&self, shape_ix: usize, position: usize) -> PyResult<Self> {
        self.0
            .play_move(shape_ix, position)
            .map(|woodoku| Self(woodoku))
            .map_err(|err| PyTypeError::new_err(err.to_string()))
    }
}

#[pymodule]
fn woodoku_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WoodokuPy>()?;
    Ok(())
}
