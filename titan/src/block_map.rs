pub type BlockFace = usize;

pub const FACE_FRONT: BlockFace = 0;
pub const FACE_BACK: BlockFace = 1;
pub const FACE_LEFT: BlockFace = 2;
pub const FACE_RIGHT: BlockFace = 3;
pub const FACE_TOP: BlockFace = 4;
pub const FACE_BOTTOM: BlockFace = 5;

pub const TEX_X_STEP: f32 = 1.0 / 16.0;
pub const TEX_Y_STEP: f32 = 1.0;

// Going forward in the scene increases -Z. -Z = far away

/*

impl From<Box> for Mesh {
    fn from(sp: Box) -> Self {
        let vertices = &[
            // Top
            ([sp.min_x, sp.min_y, sp.max_z], [0., 0., 1.0], [0., 0.]),
            ([sp.max_x, sp.min_y, sp.max_z], [0., 0., 1.0], [1.0, 0.]),
            ([sp.max_x, sp.max_y, sp.max_z], [0., 0., 1.0], [1.0, 1.0]),
            ([sp.min_x, sp.max_y, sp.max_z], [0., 0., 1.0], [0., 1.0]),
            // Bottom
            ([sp.min_x, sp.max_y, sp.min_z], [0., 0., -1.0], [1.0, 0.]),
            ([sp.max_x, sp.max_y, sp.min_z], [0., 0., -1.0], [0., 0.]),
            ([sp.max_x, sp.min_y, sp.min_z], [0., 0., -1.0], [0., 1.0]),
            ([sp.min_x, sp.min_y, sp.min_z], [0., 0., -1.0], [1.0, 1.0]),
            // Right
            ([sp.max_x, sp.min_y, sp.min_z], [1.0, 0., 0.], [0., 0.]),
            ([sp.max_x, sp.max_y, sp.min_z], [1.0, 0., 0.], [1.0, 0.]),
            ([sp.max_x, sp.max_y, sp.max_z], [1.0, 0., 0.], [1.0, 1.0]),
            ([sp.max_x, sp.min_y, sp.max_z], [1.0, 0., 0.], [0., 1.0]),
            // Left
            ([sp.min_x, sp.min_y, sp.max_z], [-1.0, 0., 0.], [1.0, 0.]),
            ([sp.min_x, sp.max_y, sp.max_z], [-1.0, 0., 0.], [0., 0.]),
            ([sp.min_x, sp.max_y, sp.min_z], [-1.0, 0., 0.], [0., 1.0]),
            ([sp.min_x, sp.min_y, sp.min_z], [-1.0, 0., 0.], [1.0, 1.0]),


            // Front
            ([sp.max_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [1.0, 0.]),
            ([sp.min_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [0., 0.]),
            ([sp.min_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [0., 1.0]),
            ([sp.max_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [1.0, 1.0]),
            // Back
            ([sp.max_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [0., 0.]),
            ([sp.min_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [1.0, 0.]),
            ([sp.min_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [1.0, 1.0]),
            ([sp.max_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [0., 1.0]),
        ];

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let indices = Indices::U32(vec![
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ]);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}

*/

/*pub const VERTEX_MAP: [Vec<[f32; 3]>; 2] = [
    // Front
    vec![
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ],
    // Back
    vec![
        [1.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    ],
];*/

pub fn vertex_offset(input: [f32; 3], x: f32, y: f32, z: f32) -> [f32; 3] {
    return [x + input[0], y + input[1], z + input[2]];
}

// Defined in counter clockwise
pub const VERTEX_MAP: [[[f32; 3]; 4]; 6] = [
    [
        // Front
        [0.0, 0.0, 1.0], // Bottom  Left
        [1.0, 0.0, 1.0], // Bottom  Right
        [1.0, 1.0, 1.0], // Top     Right
        [0.0, 1.0, 1.0], // Top     Left
    ],
    [
        // Back
        [1.0, 1.0, 0.0], // Top     Right
        [1.0, 0.0, 0.0], // Bottom  Right
        [0.0, 0.0, 0.0], // Bottom  Left
        [0.0, 1.0, 0.0], // Top     Left
    ],
    [
        // Left
        [0.0, 1.0, 1.0], // Top     Front
        [0.0, 1.0, 0.0], // Top     Back
        [0.0, 0.0, 0.0], // Bottom  Back
        [0.0, 0.0, 1.0], // Bottom  Front
    ],
    [
        // Right
        [1.0, 0.0, 0.0], // Bottom   Back
        [1.0, 1.0, 0.0], // Top      Back
        [1.0, 1.0, 1.0], // Top      Front
        [1.0, 0.0, 1.0], // Bottom   Front
    ],
    [
        // Top
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
    ],
    [
        // Bottom
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ],
];

pub const NORMAL_MAP: [[[f32; 3]; 4]; 6] = [
    [
        // Front
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ],
    [
        // Back
        [1.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    ],
    [
        // Left
        [0.0, 1.0, 1.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
    ],
    [
        // Right
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        [1.0, 0.0, 1.0],
    ],
    [
        // Top
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
    ],
    [
        // Bottom
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ],
];

pub const INDEX_MAP: [[u32; 6]; 6] = [
    [0, 1, 3, 1, 2, 3],
    [0, 1, 3, 1, 2, 3],
    [0, 1, 3, 1, 2, 3],
    [0, 1, 3, 1, 2, 3],
    [0, 1, 3, 1, 2, 3],
    [0, 1, 3, 1, 2, 3],
];
