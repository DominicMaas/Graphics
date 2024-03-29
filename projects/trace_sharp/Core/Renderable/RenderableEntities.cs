﻿using ComputeSharp;
using TraceSharp.Core.Math;

namespace TraceSharp.Core.Renderable
{
    public static class RenderableEntities
    {
        public const int SPHERE = 0;
        public const int PLANE = 1;

        public static RayIntersection IntersectWithRay(RenderableEntity entity, Ray ray)
        {
            switch (entity.Type)
            {
                case SPHERE:
                    return IntersectSphereWithRay(entity, ray);

                case PLANE:
                    return IntersectPlaneWithRay(entity, ray);

                default:
                    return new RayIntersection();
            }
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

        private static RayIntersection IntersectPlaneWithRay(RenderableEntity entity, Ray ray)
        {
            var denom = Hlsl.Dot(entity.Normal, ray.Direction);
            if (denom > 0.000001f)
            {
                var v = entity.Position - ray.Origin;
                var distance = Hlsl.Dot(v, entity.Normal) / denom;
                if (distance >= 0.0)
                {
#pragma warning disable IDE0017 // Simplify object initialization
                    var rayIntersection = new RayIntersection();
                    rayIntersection.Intersecting = true;
                    rayIntersection.Distance = distance;
#pragma warning restore IDE0017 // Simplify object initialization

                    return rayIntersection;
                }
            }

            return new RayIntersection();
        }

        public static float3 SurfaceNormal(RenderableEntity entity, float3 hitPoint)
        {
            switch (entity.Type)
            {
                case SPHERE:
                    return SphereSurfaceNormal(entity, hitPoint);

                case PLANE:
                    return PlaneSurfaceNormal(entity, hitPoint);

                default:
                    return new float3(0, 0, 0);
            }
        }

        private static float3 SphereSurfaceNormal(RenderableEntity entity, float3 hitPoint)
        {
            return Hlsl.Normalize(hitPoint - entity.Position);
        }

        private static float3 PlaneSurfaceNormal(RenderableEntity entity, float3 hitPoint)
        {
            return -Hlsl.Normalize(entity.Normal);
        }
    }
}
