use vesta::{
    cgmath::Vector3,
    cgmath::{InnerSpace, Vector2},
    Mesh,
};

use crate::c_body::{CelestialBodySettings, CelestialBodyTerrainGenerator};

/// Represents a single face of terrain for a celestial body
pub struct TerrainFace {
    pub mesh: Option<Mesh>,
    resolution: u32,
    up: Vector3<f32>,
    axis_a: Vector3<f32>,
    axis_b: Vector3<f32>,
}

impl TerrainFace {
    pub fn new(resolution: u32, up: Vector3<f32>) -> Self {
        let axis_a = Vector3::new(up.y, up.z, up.x);
        let axis_b = up.cross(axis_a);

        Self {
            mesh: None,
            resolution,
            up,
            axis_a,
            axis_b,
        }
    }

    pub fn construct_mesh(
        &mut self,
        renderer: &vesta::Renderer,
        settings: CelestialBodySettings,
        generator: &CelestialBodyTerrainGenerator,
    ) {
        let mut vertices = vec![Default::default(); (self.resolution * self.resolution) as usize];
        let mut triangles = vec![0; ((self.resolution - 1) * (self.resolution - 1) * 6) as usize];

        let mut tri_index = 0;

        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let i = x + y * self.resolution;

                let percent = Vector2::new(x as f32, y as f32) / (self.resolution - 1) as f32;
                let point_on_unit_cube = self.up
                    + (percent.x - 0.5) * 2.0 * self.axis_a
                    + (percent.y - 0.5) * 2.0 * self.axis_b;

                let point_on_unit_sphere = point_on_unit_cube.normalize()
                    * settings.radius
                    * (1.0 + generator.evaluate(point_on_unit_cube, settings));

                vertices[i as usize] = vesta::Vertex::with_tex_coords(
                    point_on_unit_sphere,
                    Vector2::new(
                        x as f32 / self.resolution as f32,
                        y as f32 / self.resolution as f32,
                    ),
                );

                if x != self.resolution - 1 && y != self.resolution - 1 {
                    triangles[tri_index] = i;
                    triangles[tri_index + 1] = i + self.resolution + 1;
                    triangles[tri_index + 2] = i + self.resolution;

                    triangles[tri_index + 3] = i;
                    triangles[tri_index + 4] = i + 1;
                    triangles[tri_index + 5] = i + self.resolution + 1;

                    tri_index += 6;
                }
            }
        }

        // Create a mesh with the specified vertices and indices
        self.mesh = Some(renderer.create_mesh(vertices, triangles));
    }
}
