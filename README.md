# jabu_fixed

Fixes events and input for fixed timesteps in Bevy 0.10

## Fixed Events

```rust
use bevy::prelude::*;
use jabu_fixed::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // add plugin
        .add_plugin(FixedPlugin)
        // use `add_fixed_event` instead of `add_event`
        .add_fixed_event::<MyFixedEvent>()
        // ...
        .run();
}
```

## Fixed Input

```rust
use bevy::prelude::*;
use jabu_fixed::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // add plugin
        .add_plugin(FixedPlugin)
        // run after `FixedInputSystem`
        .add_system_to_schedule(
            CoreSchedule::FixedUpdate,
            my_system.after(FixedInputSystem),
        )
        // ...
        .run();
}

// use `FixedInput` instead of `Input`
fn my_system(keys: Res<FixedInput<KeyCode>>) {}
```