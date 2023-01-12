use image_copy::ImageDataEvent;
use pyo3::prelude::*;

pub mod image_copy;
pub mod scene_2d_shapes;
pub mod scene_basic_cube;
pub mod scene_tester;

use bevy::prelude::*;
use scene_tester::{SceneController, SceneTesterPlugin};

#[pyclass(unsendable)]
struct SimTest {
    pub num: usize,
    app: App,
}

#[pymethods]
impl SimTest {
    #[new]
    fn new() -> Self {
        Self {
            num: 0,
            app: App::new(),
        }
    }

    pub fn method(&mut self) -> PyResult<usize> {
        self.num += 1;
        Ok(self.num)
    }

    pub fn run(&mut self) -> PyResult<()> {
        self.app
            .insert_resource(SceneController::new(true))
            .add_plugin(SceneTesterPlugin)
            .add_plugin(scene_basic_cube::ScenePlugin)
            .add_event::<ImageDataEvent>();

        Ok(())
    }

    pub fn get_images   (&mut self) -> PyResult<Option<Vec<(Vec<u8>, u32, u32)>>> {
        let events = self.app.world.resource_mut::<Events<ImageDataEvent>>();
        if events.is_empty() {
            return Ok(None);
        }

        let mut reader = events.get_reader();
        let images = reader
            .iter(&events)
            .map(|event| (event.data.clone(), event.width, event.height))
            .collect::<Vec<_>>();
        Ok(Some(images))
    }

    pub fn step(&mut self) {
        self.app.update();
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rerun_bevy_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimTest>()?;
    Ok(())
}
