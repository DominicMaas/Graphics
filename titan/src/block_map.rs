use crate::chunk::VoxelType;

pub type BlockFace = usize;

pub const FACE_FRONT: BlockFace = 0;
pub const FACE_BACK: BlockFace = 1;
pub const FACE_RIGHT: BlockFace = 2;
pub const FACE_LEFT: BlockFace = 3;
pub const FACE_TOP: BlockFace = 4;
pub const FACE_BOTTOM: BlockFace = 5;

pub const TEX_X_STEP: f32 = 1.0 / 16.0;
pub const TEX_Y_STEP: f32 = 1.0;

pub fn vertex_offset(input: [f32; 3], x: f32, y: f32, z: f32) -> [f32; 3] {
    return [x + input[0], y + input[1], z + input[2]];
}

pub fn add_uvs(offset: [f32; 2], map: [f32; 2]) -> [f32; 2] {
    return [offset[0] + map[0], offset[1] + map[1]];
}

pub struct TextureOffset {
    pub front: [f32; 2],
    pub back: [f32; 2],
    pub left: [f32; 2],
    pub right: [f32; 2],
    pub top: [f32; 2],
    pub bottom: [f32; 2],
}

pub fn texture_offset_from_block(block_type: VoxelType) -> TextureOffset {
    let side_offset: f32;
    let top_offset: f32;
    let bottom_offset: f32;

    match block_type {
        VoxelType::Dirt(_) => {
            side_offset = 0.0;
            top_offset = 0.0;
            bottom_offset = 0.0
        }
        VoxelType::Grass(_) => {
            side_offset = 2.0;
            top_offset = 2.0;
            bottom_offset = 2.0
        }
        VoxelType::Leaf => {
            side_offset = 3.0;
            top_offset = 3.0;
            bottom_offset = 3.0
        }
        VoxelType::Log => {
            side_offset = 5.0;
            top_offset = 11.0;
            bottom_offset = 11.0
        }
        VoxelType::Stone(_) => {
            side_offset = 5.0;
            top_offset = 5.0;
            bottom_offset = 5.0
        }
        VoxelType::Sand(_) => {
            side_offset = 6.0;
            top_offset = 6.0;
            bottom_offset = 6.0
        }
        VoxelType::Glass => {
            side_offset = 7.0;
            top_offset = 7.0;
            bottom_offset = 7.0
        }
        VoxelType::Brick => {
            side_offset = 8.0;
            top_offset = 8.0;
            bottom_offset = 8.0
        }
        VoxelType::Water => {
            side_offset = 7.0;
            top_offset = 7.0;
            bottom_offset = 7.0
        }
        _ => {
            side_offset = 0.0;
            top_offset = 0.0;
            bottom_offset = 0.0
        }
    }

    TextureOffset {
        front: [TEX_X_STEP * side_offset, 0.0],
        back: [TEX_X_STEP * side_offset, 0.0],
        left: [TEX_X_STEP * side_offset, 0.0],
        right: [TEX_X_STEP * side_offset, 0.0],
        top: [TEX_X_STEP * top_offset, 0.0],
        bottom: [TEX_X_STEP * bottom_offset, 0.0],
    }
}

pub const TEXTURE_MAP: [[[f32; 2]; 4]; 6] = [
    [
        [0.0, 0.0],
        [TEX_X_STEP, 0.0],
        [TEX_X_STEP, TEX_Y_STEP],
        [0.0, TEX_Y_STEP],
    ],
    [
        [TEX_X_STEP, 0.0],
        [0.0, 0.0],
        [0.0, TEX_Y_STEP],
        [TEX_X_STEP, TEX_Y_STEP],
    ],
    [
        [0.0, 0.0],
        [TEX_X_STEP, 0.0],
        [TEX_X_STEP, TEX_Y_STEP],
        [0.0, TEX_Y_STEP],
    ],
    [
        [TEX_X_STEP, 0.0],
        [0.0, 0.0],
        [0.0, TEX_Y_STEP],
        [TEX_X_STEP, TEX_Y_STEP],
    ],
    [
        [TEX_X_STEP, 0.0],
        [0.0, 0.0],
        [0.0, TEX_Y_STEP],
        [TEX_X_STEP, TEX_Y_STEP],
    ],
    [
        [0.0, 0.0],
        [TEX_X_STEP, 0.0],
        [TEX_X_STEP, TEX_Y_STEP],
        [0.0, TEX_Y_STEP],
    ],
];

// Defined in counter clockwise
pub const VERTEX_MAP: [[[f32; 3]; 4]; 6] = [
    [
        // Front
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ],
    [
        //  Back
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
    ],
    [
        // Right
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        [1.0, 0.0, 1.0],
    ],
    [
        // Left
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0],
    ],
    [
        // Top
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
    ],
    [
        // Bottom
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ],
];

pub const NORMAL_MAP: [[[f32; 3]; 4]; 6] = [
    [
        // Front
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ],
    [
        // Back
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
    ],
    [
        // Right
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ],
    [
        // Left
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
    ],
    [
        // Top
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ],
    [
        // Bottom
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
    ],
];

pub const INDEX_MAP: [[u32; 6]; 6] = [
    [0, 1, 2, 2, 3, 0],
    [0, 1, 2, 2, 3, 0],
    [0, 1, 2, 2, 3, 0],
    [0, 1, 2, 2, 3, 0],
    [0, 1, 2, 2, 3, 0],
    [0, 1, 2, 2, 3, 0],
];
