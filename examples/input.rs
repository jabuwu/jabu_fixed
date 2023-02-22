//! Demonstrates input in fixed update.

use bevy::prelude::*;
use jabu_fixed::prelude::*;

const FIXED_TIMESTEP: f32 = 0.5;
//const FIXED_TIMESTEP: f32 = 1. / 300.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FixedPlugin)
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .add_startup_system(setup)
        .add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            fixed_update.after(FixedInputSystem),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Press any key and see output\n\nUsually, normal input will be missed, but fixed input will be received",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 40.,
                ..Default::default()
            },
        )
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}

fn fixed_update(fixed_input: Res<FixedInput<KeyCode>>, input: Res<Input<KeyCode>>) {
    for pressed in input.get_just_pressed() {
        println!("just pressed (normal): {:?}", pressed);
    }
    for released in input.get_just_released() {
        println!("just released (normal): {:?}", released);
    }
    for pressed in fixed_input.get_just_pressed() {
        println!("just pressed (fixed): {:?}", pressed);
    }
    for released in fixed_input.get_just_released() {
        println!("just released (fixed): {:?}", released);
    }
}
