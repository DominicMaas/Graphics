use cgmath::{BaseFloat, Matrix3, Matrix4, Quaternion, Rotation, Vector3};

/// Describes the position, rotation and scale of an object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform<S> {
    /// The world space position of the Transform.
    pub position: Vector3<S>,
    /// The world space rotation of the Transform.
    pub rotation: Quaternion<S>,
    /// The world space scale of the Transform.
    pub scale: Vector3<S>,
}

impl<S: BaseFloat> Transform<S> {
    /// Calculate the model matrix for this transform
    pub fn calculate_model_matrix(&self) -> Matrix4<S> {
        Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    /// Calculate the normal matrix for this transform
    pub fn calculate_normal_matrix(&self) -> Matrix3<S> {
        let model = self.calculate_model_matrix();
        Matrix3::from_cols(model.x.truncate(), model.y.truncate(), model.z.truncate())
    }

    /// Rotate the transform so the forward vector points towards at the specified world position
    pub fn look_at(&mut self, world_pos: Vector3<S>, up: Vector3<S>) {
        self.rotation = Quaternion::look_at(world_pos, up)
    }
}

impl Default for Transform<f32> {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}
