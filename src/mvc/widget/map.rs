use crate::prelude::*;

pub struct MapWidget<A, B> {
    pub(crate) element: Element<A>,
    pub(crate) map: Map<A, B>,
}

impl<A, B> MapWidget<A, B> {
    pub fn new(element: Element<A>, map: Map<A, B>) -> Self {
        Self { element, map }
    }
}

impl<A: 'static, B: 'static + Send + Sync> Widget<B> for MapWidget<A, B> {
    fn on_event(&mut self, event: Event, msg: Sender<B>) -> Result<()> {
        let (sender, mut receiver) = channel::<A>();

        self.element.on_event(event, sender)?;

        while let Ok(message) = receiver.try_recv() {
            msg.send(self.map.map(message));
        }

        Ok(())
    }

    fn draw(&self, scene: &mut Scene) -> Result<()> {
        self.element.draw(scene)
    }
}
