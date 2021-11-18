using ComputeSharp;
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

    // https://computergraphics.stackexchange.com/questions/4248/how-is-anti-aliasing-implemented-in-ray-tracing
    public static readonly float4x2 JitterMatrix = new(
        new(-1.0f / 4.0f, 3.0f / 4.0f),
        new(3.0f / 4.0f, 1.0f / 3.0f),
        new(-3.0f / 4.0f, -1.0f / 4.0f),
        new(1.0f / 4.0f, -3.0f / 4.0f));

    public void Execute()
    {
        // Base color
        float3 baseColor = new(0, 0, 0);

        // 4 samples
        for (var sample = 0; sample < 4; sample++)
        {
            // Create a prime ray from the camera for our initial intersection
            var primeRay = Ray.CreatePrime(ThreadIds.X, ThreadIds.Y, scene, sample, JitterMatrix);

            // Determine if there is an intersection, if so, shade, otherwise set to black
            var intersection = PathTrace(primeRay);
            if (intersection.EntityIndex != -1)
            {
                baseColor += GetColor(scene, primeRay, intersection);
            }
        }

        textureBuffer[ThreadIds.XY] = new float4(baseColor / 4f, 1);
    }

    /// <summary>
    ///     Trace a ray through all renderable entities
    /// </summary>
    /// <param name="ray">The ray to trace</param>
    /// <returns>The distance and entity id if a ray was intersected. Otherwise the entity id is set to -1</returns>
    private Intersection PathTrace(Ray ray)
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

    private float3 GetColor(Scene scene, Ray ray, Intersection intersection)
    {
        var entity = renderableEntities[intersection.EntityIndex];

        float3 hitPoint = ray.Origin + (ray.Direction * intersection.Distance);
        float3 surfaceNormal = RenderableEntities.SurfaceNormal(entity, hitPoint);
        float3 directionToLight = -Hlsl.Normalize(scene.Light.Direction);

        var shadowRay = new Ray();
        shadowRay.Origin = hitPoint + (surfaceNormal * 0.0001f);
        shadowRay.Direction = directionToLight;

        var inLight = PathTrace(shadowRay).EntityIndex == -1;

        var lightIntensity = inLight ? scene.Light.Intensity : 0.0f;
        var lightPower = Hlsl.Dot(surfaceNormal, directionToLight) * lightIntensity;
        var lightReflected = entity.Albedo / MathF.PI;

        float3 color = entity.Color * scene.Light.Color * lightPower * lightReflected;
        return color;
    }
}
