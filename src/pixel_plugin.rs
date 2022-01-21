use bevy::prelude::{App, CoreStage, IntoSystem, Plugin};
use bevy::render::camera;

/// Provides the camera system, and the quad resource for sprite meshes.
pub struct PixelCameraPlugin;

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            camera::camera_system::<super::PixelProjection>.system(),
        );
    }
}
