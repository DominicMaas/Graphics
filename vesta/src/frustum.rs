use cgmath::{InnerSpace, Matrix, Matrix3, Matrix4, Vector3, Vector4};

type Plane = usize;

const PLANE_LEFT: Plane = 0;
const PLANE_RIGHT: Plane = 1;
const PLANE_BOTTOM: Plane = 2;
const PLANE_TOP: Plane = 3;
const PLANE_NEAR: Plane = 4;
const PLANE_FAR: Plane = 5;

pub enum IntersectType {
    Outside,
    Intersecting,
    Inside,
}

pub struct Frustum {
    planes: [Vector4<f32>; 6],
    points: [Vector3<f32>; 8],
}

impl Frustum {
    /// Extract frustum planes from a projection matrix.
    pub fn new(m: Matrix4<f32>) -> Self {
        let planes: [Vector4<f32>; 6] = [
            (m.row(3) + m.row(0)).normalize(), // Left
            (m.row(3) - m.row(0)).normalize(), // Right
            (m.row(3) + m.row(1)).normalize(), // Bottom
            (m.row(3) - m.row(1)).normalize(), // Top
            (m.row(3) + m.row(2)).normalize(), // Near
            (m.row(3) - m.row(2)).normalize(), // Far
        ];

        let crosses = [
            planes[PLANE_LEFT]
                .truncate()
                .cross(planes[PLANE_RIGHT].truncate()),
            planes[PLANE_LEFT]
                .truncate()
                .cross(planes[PLANE_BOTTOM].truncate()),
            planes[PLANE_LEFT]
                .truncate()
                .cross(planes[PLANE_TOP].truncate()),
            planes[PLANE_LEFT]
                .truncate()
                .cross(planes[PLANE_NEAR].truncate()),
            planes[PLANE_LEFT]
                .truncate()
                .cross(planes[PLANE_FAR].truncate()),
            planes[PLANE_RIGHT]
                .truncate()
                .cross(planes[PLANE_BOTTOM].truncate()),
            planes[PLANE_RIGHT]
                .truncate()
                .cross(planes[PLANE_TOP].truncate()),
            planes[PLANE_RIGHT]
                .truncate()
                .cross(planes[PLANE_NEAR].truncate()),
            planes[PLANE_RIGHT]
                .truncate()
                .cross(planes[PLANE_FAR].truncate()),
            planes[PLANE_BOTTOM]
                .truncate()
                .cross(planes[PLANE_TOP].truncate()),
            planes[PLANE_BOTTOM]
                .truncate()
                .cross(planes[PLANE_NEAR].truncate()),
            planes[PLANE_BOTTOM]
                .truncate()
                .cross(planes[PLANE_FAR].truncate()),
            planes[PLANE_TOP]
                .truncate()
                .cross(planes[PLANE_NEAR].truncate()),
            planes[PLANE_TOP]
                .truncate()
                .cross(planes[PLANE_FAR].truncate()),
            planes[PLANE_NEAR]
                .truncate()
                .cross(planes[PLANE_FAR].truncate()),
        ];

        let points: [Vector3<f32>; 8] = [
            Self::intersection(PLANE_LEFT, PLANE_BOTTOM, PLANE_NEAR, crosses, planes),
            Self::intersection(PLANE_LEFT, PLANE_TOP, PLANE_NEAR, crosses, planes),
            Self::intersection(PLANE_RIGHT, PLANE_BOTTOM, PLANE_NEAR, crosses, planes),
            Self::intersection(PLANE_RIGHT, PLANE_TOP, PLANE_NEAR, crosses, planes),
            Self::intersection(PLANE_LEFT, PLANE_BOTTOM, PLANE_FAR, crosses, planes),
            Self::intersection(PLANE_LEFT, PLANE_TOP, PLANE_FAR, crosses, planes),
            Self::intersection(PLANE_RIGHT, PLANE_BOTTOM, PLANE_FAR, crosses, planes),
            Self::intersection(PLANE_RIGHT, PLANE_TOP, PLANE_FAR, crosses, planes),
        ];

        Self { planes, points }
    }

    fn intersection(
        a: Plane,
        b: Plane,
        c: Plane,
        crosses: [Vector3<f32>; 15],
        planes: [Vector4<f32>; 6],
    ) -> Vector3<f32> {
        let d = cgmath::dot(planes[a].truncate(), crosses[Self::ij2k(b, c)]);
        let res = Matrix3 {
            x: crosses[Self::ij2k(b, c)],
            y: -crosses[Self::ij2k(a, c)],
            z: crosses[Self::ij2k(a, b)],
        } * Vector3::new(planes[a].w, planes[b].w, planes[c].w);

        res * (-1.0 / d)
    }

    fn ij2k(i: Plane, j: Plane) -> Plane {
        i * (9 - i) / 2 + j - 1
    }

    /// Sourced from here aparently: https://gist.github.com/podgorskiy/e698d18879588ada9014768e3e82a644
    /// https://iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm
    pub fn is_box_visible(&self, min: Vector3<f32>, max: Vector3<f32>) -> bool {
        for i in 0..6 {
            if (cgmath::dot(self.planes[i], Vector4::new(min.x, min.y, min.z, 1.0)) < 0.0)
                && (cgmath::dot(self.planes[i], Vector4::new(max.x, min.y, min.z, 1.0)) < 0.0)
                && (cgmath::dot(self.planes[i], Vector4::new(min.x, max.y, min.z, 1.0)) < 0.0)
                && (cgmath::dot(self.planes[i], Vector4::new(max.x, max.y, min.z, 1.0)) < 0.0)
                && (cgmath::dot(self.planes[i], Vector4::new(min.x, min.y, max.z, 1.0)) < 0.0)
                && (cgmath::dot(self.planes[i], Vector4::new(max.x, min.y, max.z, 1.0)) < 0.0)
                && (cgmath::dot(self.planes[i], Vector4::new(min.x, max.y, max.z, 1.0)) < 0.0)
                && (cgmath::dot(self.planes[i], Vector4::new(max.x, max.y, max.z, 1.0)) < 0.0)
            {
                return false;
            }
        }

        // check frustum outside/inside box
        let mut out = 0;

        for i in 0..8 {
            out += if self.points[i].x > max.x { 1 } else { 0 };
        }

        if out == 8 {
            return false;
        }

        out = 0;

        for i in 0..8 {
            out += if self.points[i].x < min.x { 1 } else { 0 };
        }

        if out == 8 {
            return false;
        }

        out = 0;

        for i in 0..8 {
            out += if self.points[i].y > max.y { 1 } else { 0 };
        }

        if out == 8 {
            return false;
        }

        out = 0;

        for i in 0..8 {
            out += if self.points[i].y < min.y { 1 } else { 0 };
        }

        if out == 8 {
            return false;
        }

        out = 0;

        for i in 0..8 {
            out += if self.points[i].z > max.z { 1 } else { 0 };
        }

        if out == 8 {
            return false;
        }

        out = 0;

        for i in 0..8 {
            out += if self.points[i].z < min.z { 1 } else { 0 };
        }

        if out == 8 {
            return false;
        }

        true
    }
}
