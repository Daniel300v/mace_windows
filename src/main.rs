use bevy::prelude::*;

fn main() {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(hello_world);

    app.run();
}


fn hello_world() {
    println!("hello world!");
}