use crate::*;

pub(crate) struct EventLoop {
    pub(crate) handle: Box<dyn FnMut(&mut Shell, Event) -> Result<()>>,
}

impl EventLoop {
    pub(crate) fn call(&mut self, shell: &mut Shell, event: Event) -> Result<()> {
        (self.handle)(shell, event)
    }
}

pub(crate) trait IntoEventLoop {
    fn event_loop(self) -> EventLoop;
}

impl<T: 'static> IntoEventLoop for T
where
    T: FnMut(&mut Shell, Event) -> Result<()>,
{
    fn event_loop(self) -> EventLoop {
        EventLoop {
            handle: Box::new(self),
        }
    }
}
