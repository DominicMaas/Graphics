using ComputeSharp;
using TraceSharp.Core.Math;

namespace TraceSharp.Core.Renderable
{
    public static class RenderableEntities
    {
        public const int SPHERE = 0;

        public static RayIntersection IntersectWithRay(RenderableEntity entity, Ray ray)
        {
            return IntersectSphereWithRay(entity, ray);
        }

        private static RayIntersection IntersectSphereWithRay(RenderableEntity entity, Ray ray)
        {
            // Create a line segment between the ray origin and the center of the sphere
            var line = entity.Position - ray.Origin;

            // Use line as a hypotenuse and find the length of the adjacent side
            var adjacent = Hlsl.Dot(line, ray.Direction);

            // Find the length-squared of the opposite side
            var length2 = Hlsl.Dot(line, line) - (adjacent * adjacent);

            // Determine the radius squared
            var radius2 = entity.Radius * entity.Radius;

            // If that length-squared is greater than radius squared, the ray doesn't interact the sphere
            if (length2 > radius2)
            {
                return new RayIntersection();
            }

            var thc = MathF.Sqrt(radius2 - length2);
            var t0 = adjacent - thc;
            var t1 = adjacent + thc;

            if (t0 < 0.0f && t1 < 0.0f)
            {
                return new RayIntersection();
            }

            // Determine the intersect distance
            var distance = t0 < t1 ? t0 : t1;

#pragma warning disable IDE0017 // Simplify object initialization
            var rayIntersection = new RayIntersection();
            rayIntersection.Intersecting = true;
            rayIntersection.Distance = distance;
#pragma warning restore IDE0017 // Simplify object initialization

            return rayIntersection;
        }

        public static float3 SurfaceNormal(RenderableEntity entity, float3 hitPoint)
        {
            return Hlsl.Normalize(hitPoint - entity.Position);
        }
    }
}
