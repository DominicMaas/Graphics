using ComputeSharp;
using System.Numerics;
using TraceSharp.Core;
using TraceSharp.Core.Math;
using TraceSharp.Core.Renderable;

namespace TraceSharp.Shaders;

/// <summary>
///     Shader that runs for every pixel on the image, computes the image
/// </summary>
[AutoConstructor]
public readonly partial struct RayShader : IComputeShader
{
    public readonly IReadWriteTexture2D<float4> textureBuffer;

    public readonly Scene scene;

    public readonly ReadOnlyBuffer<RenderableEntity> renderableEntities;

    public void Execute()
    {
        textureBuffer[ThreadIds.XY] = new Float4(0, 0, 0, 1);

        var ray = Ray.CreatePrime(ThreadIds.X, ThreadIds.Y, scene);

        var intersection = Trace(ray);
        if (intersection.EntityIndex != -1)
        {
            textureBuffer[ThreadIds.XY] = GetColor(scene, ray, intersection);
        }
    }

    /// <summary>
    ///     Trace a ray through all renderable entities
    /// </summary>
    /// <param name="ray">The ray to trace</param>
    /// <returns>The distance and entity id if a ray was intersected. Otherwise the entity id is set to -1</returns>
    private Intersection Trace(Ray ray)
    {
        var entityId = -1;
        var shortestDistance = float.MaxValue;

        // Loop through all entities to trace
        for (var i = 0; i < renderableEntities.Length; i++)
        {
            // See if this ray intersects the renderable entity
            var rayIntersection = RenderableEntities.IntersectWithRay(renderableEntities[i], ray);

            // If not intersecting, skip
            if (!rayIntersection.Intersecting) continue;

            // Compare distances
            if (rayIntersection.Distance < shortestDistance)
            {
                shortestDistance = rayIntersection.Distance;
                entityId = i;
            }
        }

        var intersection = new Intersection();
        intersection.Distance = shortestDistance;
        intersection.EntityIndex = entityId;

        return intersection;
    }

    private Float4 GetColor(Scene scene, Ray ray, Intersection intersection)
    {
        var entity = renderableEntities[intersection.EntityIndex];

        var hitPoint = ray.Origin + (ray.Direction * intersection.Distance);
        var surfaceNormal = RenderableEntities.SurfaceNormal(entity, hitPoint);
        var directionToLight = -Vector3.Normalize(scene.Light.Direction);

        var shadowRay = new Ray();
        shadowRay.Origin = hitPoint + (surfaceNormal * new Vector3(0.0001f, 0.0001f, 0.0001f));
        shadowRay.Direction = directionToLight;

        var inLight = Trace(shadowRay).EntityIndex == -1;

        var lightIntensity = inLight ? scene.Light.Intensity : 0.0f;
        var lightPower = Vector3.Dot(surfaceNormal, directionToLight) * lightIntensity;
        var lightReflected = entity.Albedo / MathF.PI;

        var color = entity.Color * scene.Light.Color * lightPower * lightReflected;
        return new Float4(color.X, color.Y, color.Z, 1);
    }
}
