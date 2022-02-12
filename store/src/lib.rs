use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};

use once_cell::sync::OnceCell;


static SENDER: OnceCell<Mutex<Sender<(SubId, Kind)>>> = OnceCell::new();

enum Kind {
    Register,
    Unregister,
}

type SubId = String;

struct EventLoop<T: Fn(i32)> {
    sub_ids: HashSet<SubId>,
    receiver: Receiver<(SubId, Kind)>,
    handler: T,
}

impl<T: Fn(i32)> EventLoop<T> {
    fn start(mut self) {
        loop {
            for (sub_id, kind) in self.receiver.try_iter() {
                match kind {
                    Kind::Register => self.sub_ids.insert(sub_id),
                    Kind::Unregister => self.sub_ids.remove(&sub_id),
                };
            }

            std::thread::sleep(std::time::Duration::from_secs(1));

            for sub_id in self.sub_ids.iter() {
                (self.handler)(sub_id.parse().unwrap());
            }
        }
    }
}

pub fn start_event_loop<T: Fn(i32)>(f: T) {
    let (sender, receiver) = channel::<(SubId, Kind)>();

    SENDER.set(Mutex::new(sender)).expect("failed to initialize SENDER");

    let event_loop = EventLoop { sub_ids: HashSet::new(), receiver, handler: f };

    event_loop.start()
}

pub fn register_event<'a>(sub_id: SubId) {
    SENDER.get()
        .expect("failed to get SENDER")
        .lock()
        .expect("failed to lock SENDER")
        .send((sub_id, Kind::Register))
        .expect("failed to register event");
}

pub fn unregister_event<'a>(sub_id: SubId) {
    SENDER.get()
        .expect("failed to get SENDER")
        .lock()
        .expect("failed to lock SENDER")
        .send((sub_id, Kind::Unregister))
        .expect("failed to unregister event");
}
