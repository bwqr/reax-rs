use serde_reflection::{Tracer, TracerConfig};

use store::User;

fn main() {
    let mut tracer = Tracer::new(TracerConfig::default());

    tracer.trace_simple_type::<User>().unwrap();

    let registry = tracer.registry().unwrap();

    println!("{}", serde_yaml::to_string(&registry).unwrap());
}
