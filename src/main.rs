use bevy::prelude::*;

struct Tower;

struct Height(i32);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TowerPlugin)
        .run();
}

fn add_tower(commands: &mut Commands) {
    commands.spawn((Tower, Height(0)));
}

fn tick_tower(mut query: Query<&mut Height, With<Tower>>) {
    for mut height in query.iter_mut() {
        if height.0 > 100 {
            height.0 = 0
        } else {
            height.0 += 1
        }
    }
}

fn display_tower(query: Query<&Height, With<Tower>>) {
    for h in query.iter() {
        for _ in 0..(h.0) {
            print!(".")
        }
        println!()
    }
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_tower.system())
            .add_system(tick_tower.system())
            .add_system(display_tower.system());
    }
}
