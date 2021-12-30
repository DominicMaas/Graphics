use vesta::{
    cgmath::{num_traits::pow, Vector3},
    cgmath::{InnerSpace, Vector2},
    DrawMesh, Mesh,
};

use crate::c_body::{CelestialBodySettings, CelestialBodyTerrainGenerator};

/// Represents a single face of terrain for a celestial body
/// This terrain face is split up into a quadtree, each child has
/// double the resolution up until a certain depth
pub struct TerrainFace {
    mesh: Option<Mesh>,
    resolution: u32,
    depth_scale: f32,
    // Represents the top left base offset of this terrain face (used to render the children)
    quad_offset: Vector2<f32>,
    up: Vector3<f32>,
    axis_a: Vector3<f32>,
    axis_b: Vector3<f32>,
    children: Option<Vec<TerrainFace>>,
}

impl TerrainFace {
    pub fn new(
        resolution: u32,
        depth: u32,
        max_depth: u32,
        quad_offset: Vector2<f32>,
        up: Vector3<f32>,
    ) -> Self {
        let axis_a = Vector3::new(up.y, up.z, up.x);
        let axis_b = up.cross(axis_a);

        let depth_scale = pow(2, depth as usize) as f32;

        let mut children_optional: Option<Vec<TerrainFace>> = None;

        if depth < max_depth {
            let unit_scale = (1.0 / depth_scale) / 2.0;
            let res_scale = resolution * 2;

            let mut children: Vec<TerrainFace> = Vec::new();

            children.push(TerrainFace::new(
                res_scale,
                depth + 1,
                max_depth,
                quad_offset + Vector2::new(0.0, 0.0),
                up,
            ));

            children.push(TerrainFace::new(
                res_scale,
                depth + 1,
                max_depth,
                quad_offset + Vector2::new(unit_scale, 0.0),
                up,
            ));

            children.push(TerrainFace::new(
                res_scale,
                depth + 1,
                max_depth,
                quad_offset + Vector2::new(0.0, unit_scale),
                up,
            ));

            children.push(TerrainFace::new(
                res_scale,
                depth + 1,
                max_depth,
                quad_offset + Vector2::new(unit_scale, unit_scale),
                up,
            ));

            children_optional = Some(children);
        }

        Self {
            mesh: None,
            resolution,
            depth_scale,
            quad_offset,
            up,
            axis_a,
            axis_b,
            children: children_optional,
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
                // Index in the vertices array
                let i = x + y * self.resolution;

                // The percentage we are through the current resolution
                let percent = self.calc_mesh_percentage(x as f32, y as f32);

                // Generates a Vector 3 ranging from 1 to -1 (if % is 0 to 100)
                let point_on_unit_cube = self.up
                    + (percent.x - 0.5) * 2.0 * self.axis_a
                    + (percent.y - 0.5) * 2.0 * self.axis_b;

                let point_on_unit_sphere = point_on_unit_cube.normalize()
                    * settings.radius
                    * (1.0 + generator.evaluate(point_on_unit_cube, settings));

                vertices[i as usize] =
                    vesta::Vertex::with_tex_coords(point_on_unit_sphere, percent);

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

        // Build children mesh
        match self.children.as_mut() {
            Some(children) => {
                for child in children.iter_mut() {
                    child.construct_mesh(renderer, settings, generator);
                }
            }
            None => {}
        }
    }

    fn calc_mesh_percentage(&self, x: f32, y: f32) -> Vector2<f32> {
        // The percentage we are through the current resolution, this changes based on the current
        // depth
        let mut percent = Vector2::new(x, y) / (self.resolution - 1) as f32;

        // e.g. if depth is 2, percent ranges from 0 to 50
        percent /= self.depth_scale;

        // Apply the top left offset from the parent (we still need
        // to calculate the local offsets below)
        percent += self.quad_offset;

        return percent;
    }
}

impl vesta::components::GameObject for TerrainFace {
    /// Renders this terrain face (or the appropriate children)
    fn render<'a>(
        &'a mut self,
        render_pass: &mut vesta::wgpu::RenderPass<'a>,
        engine: &vesta::Engine,
    ) {
        match self.children.as_mut() {
            Some(children) => {
                for child in children.iter_mut() {
                    child.render(render_pass, engine);
                }
            }
            None => match &self.mesh {
                Some(mesh) => {
                    render_pass.draw_mesh(&mesh);
                }
                None => {}
            },
        }
    }
}
