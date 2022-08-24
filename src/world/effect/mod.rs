use cgmath::Vector3;
use derive_new::new;
use procedural::*;

use crate::graphics::{Camera, MarkerRenderer, Renderer};

#[derive(PrototypeElement, PrototypeWindow, new)]
#[window_title("Effect Source")]
pub struct EffectSource {
    pub name: String,
    pub position: Vector3<f32>,
    pub effect_type: usize, // TODO: fix this
    pub emit_speed: f32,
}

impl EffectSource {

    pub fn offset(&mut self, offset: Vector3<f32>) {
        self.position += offset;
    }

    #[cfg(feature = "debug")]
    pub fn render_marker<T>(&self, render_target: &mut T::Target, renderer: &T, camera: &dyn Camera, hovered: bool)
    where
        T: Renderer + MarkerRenderer,
    {
        renderer.render_marker(render_target, camera, self.position, hovered);
    }
}
