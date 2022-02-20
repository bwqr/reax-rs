use serde_reflection::{Samples, Tracer, TracerConfig};

use store::User;

fn main() {
    let mut tracer = Tracer::new(TracerConfig::default());

    let mut samples = Samples::new();

    tracer
        .trace_value(
            &mut samples,
            &vec![User {
                id: 1,
                name: "Hola".to_string(),
            }],
        )
        .expect("failed to trace value");

    tracer.trace_type_once::<Vec<User>>(&samples).unwrap();

    let registry = tracer.registry().unwrap();

    println!("{}", serde_yaml::to_string(&registry).unwrap());
}
