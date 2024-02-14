// lifted from here https://users.rust-lang.org/t/how-to-make-an-eventbus-using-a-list-of-fnmut/58996/4

pub struct EventBus<'b, E>
where
    E: Copy,
{
    pub subscribers: Vec<Box<dyn FnMut(E) + 'b>>,
}

impl<'b, E> EventBus<'b, E>
where
    E: Copy,
{
    pub fn new() -> EventBus<'b, E> {
        EventBus {
            subscribers: Vec::new(),
        }
    }

    pub fn post_event(&mut self, event: E) {
        for sub in self.subscribers.iter_mut() {
            sub(event);
        }
    }

    pub fn subscribe(&mut self, subscriber: Box<dyn FnMut(E) + 'b>) {
        self.subscribers.push(subscriber);
    }
}
// !TESTS----------------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn constructor_works() {
        EventBus::<i32>::new();
    }

    #[test]
    fn subscribe_adds_subscriber() {
        // arrange
        let mut event_bus = EventBus::<i32>::new();
        // act
        event_bus.subscribe(Box::new(|_| print!("heyo!")));
        // assert
        assert_eq!(1, event_bus.subscribers.len());
    }

    #[test]
    fn calling_post_event_alerts_all_subscribers() {
        let subscriber_a_was_alerted = Mutex::new(false);
        let subscriber_b_was_alerted = Mutex::new(false);

        let mut event_bus = EventBus::<i32>::new();
        event_bus.subscribe(Box::new(|_: i32| {
            let mut subscriber_a_was_alerted = subscriber_a_was_alerted.lock().unwrap();
            *subscriber_a_was_alerted = true;
        }));

        event_bus.subscribe(Box::new(|_: i32| {
            let mut subscriber_b_was_alerted = subscriber_b_was_alerted.lock().unwrap();
            *subscriber_b_was_alerted = true;
        }));

        event_bus.post_event(6);

        assert!(true, "{}", *subscriber_a_was_alerted.lock().unwrap());
        assert!(true, "{}", *subscriber_b_was_alerted.lock().unwrap());
    }
}
