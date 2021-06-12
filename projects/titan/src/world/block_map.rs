use vesta::cgmath::{Vector2, Vector3};

pub type BlockFace = usize;

pub const FACE_FRONT: BlockFace = 0;
pub const FACE_BACK: BlockFace = 1;
pub const FACE_LEFT: BlockFace = 2;
pub const FACE_RIGHT: BlockFace = 3;
pub const FACE_TOP: BlockFace = 4;
pub const FACE_BOTTOM: BlockFace = 5;

pub const TEX_X_STEP: f32 = 0.1667;
pub const TEX_Y_STEP: f32 = 1.0;

pub const TEXTURE_MAP: [[Vector2<f32>; 4]; 6] = [
    [
        // Front
        Vector2::new(0.0, TEX_Y_STEP),        // Bottom Left
        Vector2::new(TEX_X_STEP, TEX_Y_STEP), // Bottom Right
        Vector2::new(TEX_X_STEP, 0.0),        // Top Right
        Vector2::new(0.0, 0.0),               // Top Left
    ],
    [
        // Back
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, TEX_Y_STEP),
        Vector2::new(TEX_X_STEP, TEX_Y_STEP),
        Vector2::new(TEX_X_STEP, 0.0),
    ],
    [
        // Left
        Vector2::new(TEX_X_STEP, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, TEX_Y_STEP),
        Vector2::new(TEX_X_STEP, TEX_Y_STEP),
    ],
    [
        // Right
        Vector2::new(TEX_X_STEP, TEX_Y_STEP),
        Vector2::new(TEX_X_STEP, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, TEX_Y_STEP),
    ],
    [
        // Top
        Vector2::new(TEX_X_STEP, TEX_Y_STEP),
        Vector2::new(TEX_X_STEP, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, TEX_Y_STEP),
    ],
    [
        // Bottom
        Vector2::new(0.0, TEX_Y_STEP),
        Vector2::new(TEX_X_STEP, TEX_Y_STEP),
        Vector2::new(TEX_X_STEP, 0.0),
        Vector2::new(0.0, 0.0),
    ],
];

// Going forward in the scene increases -Z. -Z = far away

// Defined in counter clockwise
pub const VERTEX_MAP: [[Vector3<f32>; 4]; 6] = [
    [
        Vector3::new(0.0, 0.0, 1.0), // Bottom  Left
        Vector3::new(1.0, 0.0, 1.0), // Bottom  Right
        Vector3::new(1.0, 1.0, 1.0), // Top     Right
        Vector3::new(0.0, 1.0, 1.0), // Top     Left
    ],
    [
        // Back
        Vector3::new(1.0, 1.0, 0.0), // Top     Right
        Vector3::new(1.0, 0.0, 0.0), // Bottom  Right
        Vector3::new(0.0, 0.0, 0.0), // Bottom  Left
        Vector3::new(0.0, 1.0, 0.0), // Top     Left
    ],
    [
        // Left
        Vector3::new(0.0, 1.0, 1.0), // Top     Front
        Vector3::new(0.0, 1.0, 0.0), // Top     Back
        Vector3::new(0.0, 0.0, 0.0), // Bottom  Back
        Vector3::new(0.0, 0.0, 1.0), // Bottom  Front
    ],
    [
        // Right
        Vector3::new(1.0, 0.0, 0.0), // Bottom   Back
        Vector3::new(1.0, 1.0, 0.0), // Top      Back
        Vector3::new(1.0, 1.0, 1.0), // Top      Front
        Vector3::new(1.0, 0.0, 1.0), // Bottom   Front
    ],
    [
        // Top
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 1.0),
    ],
    [
        // Bottom
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0),
    ],
];

pub const NORMAL_MAP: [[Vector3<f32>; 4]; 6] = [
    [
        // Front
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ],
    [
        // Back
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ],
    [
        // Left
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ],
    [
        // Right
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ],
    [
        // Top
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ],
    [
        // Bottom
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
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
