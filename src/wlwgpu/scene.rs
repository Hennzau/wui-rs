pub use vello::peniko::Color;
pub use vello::peniko::color::palette;

use vello::{
    kurbo::{Affine, Circle, Rect},
    peniko::Fill,
};

pub struct Scene(pub(crate) vello::Scene);

impl From<vello::Scene> for Scene {
    fn from(scene: vello::Scene) -> Self {
        Scene(scene)
    }
}

impl From<Scene> for vello::Scene {
    fn from(scene: Scene) -> Self {
        scene.0
    }
}

impl AsRef<vello::Scene> for Scene {
    fn as_ref(&self) -> &vello::Scene {
        &self.0
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Self {
        vello::Scene::new().into()
    }

    pub fn clear(&mut self) {
        self.0.reset();
    }

    pub fn fill(&mut self, width: u32, height: u32, color: Color) {
        let rect = Rect::new(0.0, 0.0, width as f64, height as f64);

        self.0
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, &rect);
    }

    pub fn add_circle(&mut self, cx: f64, cy: f64, radius: f64, color: Color) {
        let circle = Circle::new((cx, cy), radius);

        self.0
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, &circle);
    }
}
