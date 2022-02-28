use std::{sync::{RwLock, mpsc::Sender, Mutex}, collections::HashSet};

use once_cell::sync::OnceCell;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

static RUNTIME: OnceCell<Runtime> = OnceCell::new();
static STORE: OnceCell<RwLock<Store>> = OnceCell::new();
static HANDLER: OnceCell<Mutex<Sender<(HashSet<i32>, Vec<u8>)>>> = OnceCell::new();

struct Observable<T: Serialize> {
    state: State<T>,
    subs: HashSet<i32>,
}

impl<T: Serialize> Observable<T> {
    fn next(&mut self, val: T) {
        let ser_val = bincode::serialize(&val).expect("failed to serialize val");
        self.state = State::Some(val);
        HANDLER.get()
            .expect("failed to get handler")
            .lock()
            .expect("failed to lock handler")
            .send((self.subs.clone(), ser_val))
            .expect("failed to send payload to handler");
    }
}

enum State<T> {
    Some(T),
    Loading,
    None,
}

struct Store {
    user: Observable<User>,
}

pub fn init_runtime() {
    eprintln!("starting the runtime");
    RUNTIME
        .set(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to initialize tokio runtime"),
        )
        .expect("failed to set tokio runtime");
}

pub fn init_store() {
    STORE
        .set(RwLock::new(Store {
            user: Observable {
                state: State::None,
                subs: HashSet::new(),
            },
        }))
        .map_err(|_| "StoreError")
        .expect("failed to set store");
}

pub fn init_handler<F: Fn(HashSet<i32>, Vec<u8>)>(handler: F) {
    let (send, recv) = std::sync::mpsc::channel();
    HANDLER
        .set(Mutex::new(send))
        .map_err(|_| "HandlerError")
        .expect("failed to set handler");

    while let Ok(payload) = recv.recv() {
        handler(payload.0, payload.1)
    }
}

pub fn user() -> i32 {
    let sub = rand::thread_rng().next_u32() as i32;

    let mut store = STORE.get().expect("failed get STORE").write().expect("failed to acquire write access");
    store.user.subs.insert(sub);

    match &store.user.state {
        State::Some(user) => {
            let mut subs = HashSet::new();
            subs.insert(sub);
            HANDLER.get().expect("failed to get handler").lock().expect("failed to lock handler").send((subs, bincode::serialize(&user).expect("failed to serialize user"))).expect("failed to send payload to handler");
        },
        State::None => {
            store.user.state = State::Loading;
            fetch_user();
        },
        State::Loading => {},
    }

    sub
}

pub fn fetch_user() {
    RUNTIME.get().expect("failed to get runtime").spawn(async {
        let body = reqwest::get("https://random.justyy.workers.dev/api/random/?cached&n=128")
            .await
            .expect("failed to connect")
            .text()
            .await
            .expect("failed to retrieve contents");

        let mut thread_rng = rand::thread_rng();
        STORE.get().expect("failed to fetch store").write().unwrap().user.next(User { id: thread_rng.next_u32() as i32, name: body });
    });
}

pub fn unsubscribe(sub: i32) {
    STORE.get().expect("failed to get store").write().expect("failed to acquire write access").user.subs.remove(&sub);
}
