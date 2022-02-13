use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};

use once_cell::sync::OnceCell;

use rand::RngCore;
use serde::{Serialize, Deserialize};


static SENDER: OnceCell<Mutex<Sender<(Sub, Kind)>>> = OnceCell::new();

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    name: String,
}

enum Kind {
    Register,
    Unregister,
}

type Sub = String;

struct EventLoop<T: Fn(i32, Vec<User>)> {
    sub_ids: HashSet<Sub>,
    receiver: Receiver<(Sub, Kind)>,
    handler: T,
}

impl<T: Fn(i32, Vec<User>)> EventLoop<T> {
    fn start(mut self) {
        let mut rng_thread = rand::thread_rng();
        loop {
            for (sub_id, kind) in self.receiver.try_iter() {
                match kind {
                    Kind::Register => self.sub_ids.insert(sub_id),
                    Kind::Unregister => self.sub_ids.remove(&sub_id),
                };
            }

            let mut users = Vec::with_capacity(1000);
            for _ in 0..1000 {
                users.push(User { id: (rng_thread.next_u32() / 2 ) as i32, name: rng_thread.next_u64().to_string() });
            }

            for sub_id in self.sub_ids.iter() {
                (self.handler)(sub_id.parse().unwrap(), users.clone());
            }

            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }
}

pub fn start_event_loop<T: Fn(i32, Vec<User>)>(f: T) {
    let (sender, receiver) = channel::<(Sub, Kind)>();

    SENDER.set(Mutex::new(sender)).expect("failed to initialize SENDER");

    let event_loop = EventLoop { sub_ids: HashSet::new(), receiver, handler: f };

    event_loop.start()
}

pub fn register_event<'a>(sub_id: Sub) {
    SENDER.get()
        .expect("failed to get SENDER")
        .lock()
        .expect("failed to lock SENDER")
        .send((sub_id, Kind::Register))
        .expect("failed to register event");
}

pub fn unregister_event<'a>(sub_id: Sub) {
    SENDER.get()
        .expect("failed to get SENDER")
        .lock()
        .expect("failed to lock SENDER")
        .send((sub_id, Kind::Unregister))
        .expect("failed to unregister event");
}
