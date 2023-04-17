use bevy::prelude::*;
use bevy_pixels::prelude::*;

const INITIAL_WIDTH: u32 = 320;
const INITIAL_HEIGHT: u32 = 240;
const SCALE_FACTOR: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin::default())
        .add_system(draw.in_set(PixelsSet::Draw))
        .run();
}

fn draw(mut wrapper_query: Query<&mut PixelsWrapper>) {
    // Query the `PixelsWrapper` component that owns an instance of `Pixels` for the given window.
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };

    // Get a mutable slice for the pixel buffer.
    let frame: &mut [u8] = wrapper.pixels.frame_mut();

    // Fill frame with pixel data.
    // ...
}
