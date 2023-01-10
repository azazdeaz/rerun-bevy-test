use pyo3::prelude::*;

pub mod image_copy;
pub mod scene_2d_shapes;
pub mod scene_basic_cube;
pub mod scene_tester;

use bevy::prelude::*;
use scene_tester::{SceneController, SceneTesterPlugin};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rerun_bevy_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(test_images, m)?)?;
    Ok(())
}

/// test saveing images with bevy
#[pyfunction]
fn test_images() -> PyResult<()> {
    let create_images = true;

    App::new()
        .insert_resource(SceneController::new(create_images))
        .add_plugin(SceneTesterPlugin)
        .add_plugin(scene_basic_cube::ScenePlugin)
        .run();

    App::new()
        .insert_resource(SceneController::new(create_images))
        .add_plugin(SceneTesterPlugin)
        .add_plugin(scene_2d_shapes::ScenePlugin)
        .run();
    Ok(())
}
