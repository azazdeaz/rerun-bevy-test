use pyo3::prelude::*;

pub mod image_copy;
pub mod scene_2d_shapes;
pub mod scene_basic_cube;
pub mod scene_tester;

use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver};
use scene_tester::{SceneController, SceneTesterPlugin};
use std::io::stdin;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[derive(Resource)]
struct StdinReceiver(Receiver<String>);
struct StdinLine(String);
fn read_stdin_system(mut receiver: ResMut<StdinReceiver>, mut stdin: EventWriter<StdinLine>) {
    let rx = receiver;
    if let Ok(line) = rx.0.try_recv() {
        stdin.send(StdinLine(line));
    }
}

fn log_stdin_system(mut reader: EventReader<StdinLine>) {
    for line in reader.iter() {
        println!("Stdin gave {:?}", line.0);
    }
}

#[pyclass(unsendable)]
struct SimTest {
    pub num: usize,
    app: App,
}

#[pymethods]
impl SimTest {
    #[new]
    fn new() -> Self {
        Self { num: 0, app: App::new() }
    }

    pub fn method(&mut self) -> PyResult<usize> {
        self.num += 1;
        Ok(self.num)
    }

    pub fn run(&mut self) -> PyResult<()> {
        let (tx, rx) = bounded::<String>(1);
        std::thread::spawn(move || loop {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            tx.send(buf).unwrap();
        });

        self.app.add_event::<StdinLine>()
            .add_plugins(DefaultPlugins)
            .insert_resource(StdinReceiver(rx))
            .add_system(read_stdin_system)
            .add_system(log_stdin_system);

        Ok(())
    }

    pub fn step(&mut self) {
        self.app.update();
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rerun_bevy_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(test_images, m)?)?;
    m.add_function(wrap_pyfunction!(get_image, m)?)?;
    m.add_class::<SimTest>()?;
    Ok(())
}

#[pyfunction]
fn get_image(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::async_std::future_into_py(py, async {
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        Ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    })
}

/// test saveing images with bevy
#[pyfunction]
fn test_images() -> PyResult<()> {
    let create_images = true;

    // let (tx, rx) = bounded::<String>(1);
    // std::thread::spawn(move || loop {
    //     let mut buf = String::new();
    //     stdin().read_line(&mut buf).unwrap();
    //     tx.send(buf).unwrap();
    // });

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
