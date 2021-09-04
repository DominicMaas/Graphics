use cgmath::{Matrix3, Matrix4, Quaternion, Vector3};

/// Describes the position, rotation and scale of an object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// The world space position of the Transform.
    pub position: Vector3<f32>,
    /// The world space rotation of the Transform.
    pub rotation: Quaternion<f32>,
    /// The world space scale of the Transform.
    pub scale: Vector3<f32>,
}

impl Transform {
    /// Calculate the model matrix for this transform
    pub fn calculate_model_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    /// Calculate the normal matrix for this transform
    pub fn calculate_normal_matrix(&self) -> Matrix3<f32> {
        let model = self.calculate_model_matrix();
        Matrix3::from_cols(model.x.truncate(), model.y.truncate(), model.z.truncate())
    } //Matrix3::from_cols(m.x.truncate(), m.y.truncate(), m.z.truncate())
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}
