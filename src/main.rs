use std::io::stdin;

use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver};

fn main() {
    let (tx, rx) = bounded::<String>(1);
    std::thread::spawn(move || loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        tx.send(buf).unwrap();
    });
    App::new()
        .add_event::<StdinLine>()
        .add_plugins(DefaultPlugins)
        .insert_resource(StdinReceiver(rx))
        .add_system(read_stdin_system)
        .add_system(log_stdin_system)
        .run();
}

#[derive(Resource)]
struct StdinReceiver(Receiver<String>);
struct StdinLine(String);
fn read_stdin_system(
    mut receiver: ResMut<StdinReceiver>,
    mut stdin: EventWriter<StdinLine>,
) {
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
