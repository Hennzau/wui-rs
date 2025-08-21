// use std::sync::Arc;

// use parley::{FontContext, Glyph, GlyphRun, LayoutContext, RangedBuilder, fontique::Collection};
// use vello::{
//     Scene,
//     kurbo::{Affine, Circle},
//     peniko::{Blob, Color, Fill, Font},
// };

// use crate::*;

// pub struct Text {
//     pub(crate) text: String,
//     pub(crate) font: Font,
// }

// impl Text {
//     pub fn font(mut self, font: Font) -> Self {
//         self.font = font;
//         self
//     }
// }

// impl<Message> Widget<Message> for Text {
//     fn handle_event(&mut self, _: &mut Vec<Message>, _: Event) -> Result<()> {
//         Ok(())
//     }

//     fn draw(&self, scene: &mut Scene) -> Result<()> {
//         let circle = Circle::new((0.0, 0.0), 100.0);
//         let circle_fill_color = Color::new([0.9529, 0.5451, 0.6588, 1.]);

//         scene.fill(
//             vello::peniko::Fill::NonZero,
//             Affine::IDENTITY,
//             circle_fill_color,
//             None,
//             &circle,
//         );

//         // scene.draw_glyphs(&self.font).draw(Fill::NonZero, glyphs);

//         Ok(())
//     }
// }

// pub fn text<Message>(str: impl Into<String>) -> Text {
//     Text {
//         text: str.into(),
//         font: Font::new(
//             Blob::new(Arc::new(include_bytes!("JetBrainsMono-Regular.ttf"))),
//             0,
//         ),
//     }
// }
