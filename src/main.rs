mod terrain;

use bevy::{prelude::*, window::WindowResolution};
use bevy_pixels::prelude::*;
use terrain::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ymir".to_string(),
                resolution: WindowResolution::new(
                    INITIAL_WIDTH as f32 * SCALE_FACTOR,
                    INITIAL_HEIGHT as f32 * SCALE_FACTOR,
                ),
                resize_constraints: WindowResizeConstraints {
                    min_width: INITIAL_WIDTH as f32 * SCALE_FACTOR,
                    min_height: INITIAL_HEIGHT as f32 * SCALE_FACTOR,
                    ..default()
                },
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PixelsPlugin {
            primary_window: Some(PixelsOptions {
                width: INITIAL_WIDTH as u32,
                height: INITIAL_HEIGHT as u32,
                scale_factor: SCALE_FACTOR,
                auto_resize_buffer: false,
                auto_resize_surface: false,
            }),
        })
        .insert_resource(Terrain::new())
        .add_system(draw.in_set(PixelsSet::Draw))
        .run();
}

fn draw(mut wrapper_query: Query<&mut PixelsWrapper>, terrain: Res<Terrain>) {
    // Query the `PixelsWrapper` component that owns an instance of `Pixels` for the given window.
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };

    // Get a mutable slice for the pixel buffer.
    let frame: &mut [u8] = wrapper.pixels.frame_mut();

    // Fill frame with pixel data.
    terrain.height_map(frame);
}
