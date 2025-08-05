use crate::prelude::*;

#[derive(Default)]
pub struct EmptyWidget {
    pub(crate) label: Option<Label>,
    pub(crate) size: Size,
    pub(crate) display_mode: DisplayMode,
}

impl<Message> Widget<Message> for EmptyWidget {
    fn label(&self) -> Option<Label> {
        self.label.clone()
    }

    fn size(&self) -> Size {
        self.size.clone()
    }

    fn display_mode(&self) -> DisplayMode {
        self.display_mode.clone()
    }
}

impl EmptyWidget {
    pub fn label(self, label: impl Into<Label>) -> Self {
        Self {
            label: Some(label.into()),
            ..self
        }
    }

    pub fn size(self, size: impl Into<Size>) -> Self {
        Self {
            size: size.into(),
            ..self
        }
    }

    pub fn display_mode(self, display_mode: impl Into<DisplayMode>) -> Self {
        Self {
            display_mode: display_mode.into(),
            ..self
        }
    }
}

pub fn empty() -> EmptyWidget {
    EmptyWidget::default()
}
