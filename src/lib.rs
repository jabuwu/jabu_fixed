use std::marker::PhantomData;

use bevy::prelude::*;

pub struct FixedPlugin;

impl Plugin for FixedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FixedInputPlugin);
    }
}

#[derive(Resource)]
pub(crate) struct ClearFlag<T> {
    clear: bool,
    _marker: PhantomData<T>,
}

impl<T> Default for ClearFlag<T> {
    fn default() -> Self {
        Self {
            clear: false,
            _marker: PhantomData,
        }
    }
}

mod events;
mod input;

pub use events::*;
pub use input::*;

pub mod prelude {
    pub use super::{AddFixedEvent, FixedInput, FixedInputSystem, FixedPlugin};
}
