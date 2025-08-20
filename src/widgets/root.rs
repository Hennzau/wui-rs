use crate::*;

pub struct RootWidget<Message> {
    pub(crate) label: String,

    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) x: u32,
    pub(crate) y: u32,

    pub(crate) layer: Layer,

    pub(crate) anchor: Anchor,

    pub(crate) title: String,

    pub(crate) window: bool,

    pub(crate) keyboard: KeyboardInteractivity,

    pub(crate) background: Color,

    pub(crate) child: Option<Element<Message>>,
}

pub fn root<Message>(label: impl Into<String>) -> RootWidget<Message> {
    RootWidget {
        label: label.into(),

        width: 800,
        height: 600,

        x: 0,
        y: 0,

        layer: Layer::Top,

        anchor: Anchor::None,

        title: "WLWGPU".to_string(),

        window: false,

        keyboard: KeyboardInteractivity::None,

        background: Color::TRANSPARENT,

        child: None,
    }
}

impl<Message> RootWidget<Message> {
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn x(mut self, x: u32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: u32) -> Self {
        self.y = y;
        self
    }

    pub fn layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn windowed(mut self) -> Self {
        self.window = true;
        self
    }

    pub fn keyboard(mut self, keyboard: KeyboardInteractivity) -> Self {
        self.keyboard = keyboard;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    pub fn child(mut self, child: impl IntoElement<Message>) -> Self {
        self.child = Some(child.element());
        self
    }
}

impl<Message: 'static> Widget<Message> for RootWidget<Message> {}

pub trait IntoRootWidgets<Message> {
    fn root_widgets(self) -> Vec<RootWidget<Message>>;
}

impl<Message> IntoRootWidgets<Message> for RootWidget<Message> {
    fn root_widgets(self) -> Vec<RootWidget<Message>> {
        vec![self]
    }
}
impl<Message> IntoRootWidgets<Message> for Vec<RootWidget<Message>> {
    fn root_widgets(self) -> Vec<RootWidget<Message>> {
        self
    }
}
