//! Demonstrates sending events from frame update to fixed update.

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
        .add_event::<MyEvent>()
        .add_fixed_event::<MyFixedEvent>()
        .add_startup_system(setup)
        .add_system(frame_update)
        .add_system_to_schedule(CoreSchedule::FixedUpdate, fixed_update)
        .run();
}

#[derive(Default)]
pub struct MyEvent;

#[derive(Default)]
pub struct MyFixedEvent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Press space to send events, see received events in console\n\nUsually, MyEvent will be missed, but MyFixedEvent will be received",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 40.,
                ..Default::default()
            },
        ).with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}

fn frame_update(
    mut my_event_writer: EventWriter<MyEvent>,
    mut my_fixed_event_writer: EventWriter<MyFixedEvent>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        my_event_writer.send_default();
        my_fixed_event_writer.send_default();
    }
}

fn fixed_update(
    mut my_event_reader: EventReader<MyEvent>,
    mut my_fixed_event_reader: EventReader<MyFixedEvent>,
) {
    for _ in my_event_reader.iter() {
        println!("Received MyEvent!");
    }

    for _ in my_fixed_event_reader.iter() {
        println!("Received MyFixedEvent!");
    }
}
