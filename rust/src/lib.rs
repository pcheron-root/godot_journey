use godot::prelude::*;
use godot::classes::{Node3D, INode3D};

struct SimExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SimExtension {}

#[derive(GodotClass)]
#[class(base=Node3D)]
struct SimObject {
    base: Base<Node3D>,
    velocity: Vector3,
}

#[godot_api]
impl INode3D for SimObject {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            velocity: Vector3::new(1.0, 0.0, 0.0),
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let velocity = self.velocity;
        let pos = self.base().get_position();
        self.base_mut().set_position(pos + velocity * delta as f32);
    }
}
