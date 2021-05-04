use bevy::{app::{ScheduleRunnerSettings, ScheduleRunnerPlugin}, prelude::*, utils::Duration};

// This example only enables a minimal set of plugins required for bevy to run.
// You can also completely remove rendering / windowing Plugin code from bevy
// by making your import look like this in your Cargo.toml
//
// [dependencies]
// bevy = { version = "*", default-features = false }
// # replace "*" with the most recent version of bevy
// Note that feature/plugin `winit` provides a scheduler, thus without it,
// another scheduler must be provided.

fn main() {
    // this app runs once
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_system(hello_world_system.system())
        .run();

    // this app loops forever at 60 fps
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_system(counter.system())
        .run();

    // this app loops without a fixed timestep
    App::build()
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugins(MinimalPlugins)
        .add_system(counter.system())
        .run();
}

fn hello_world_system() {
    println!("hello world");
}

fn counter(mut state: Local<CounterState>) {
    if state.count % 60 == 0 {
        println!("{}", state.count);
    }
    state.count += 1;
}

#[derive(Default)]
struct CounterState {
    count: u32,
}
