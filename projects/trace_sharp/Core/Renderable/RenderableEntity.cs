using System.Numerics;

namespace TraceSharp.Core.Renderable;

/// <summary>
///     A base renderable entity within the renderer
/// </summary>
public struct RenderableEntity
{
    /// <summary>
    ///     The type of object to render
    /// </summary>
    public int Type;

    /// <summary>
    ///     The position of this object
    /// </summary>
    public float3 Position;

    /// <summary>
    ///     The color of this render object (this ray tracer is very basic!!!)
    /// </summary>
    public float3 Color;

    /// <summary>
    ///     The albedo of this render object (this ray tracer is very basic!!!)
    /// </summary>
    public float Albedo;

    /// <summary>
    ///     The radius of this renderable entity (if supported)
    /// </summary>
    public float Radius;

    /// <summary>
    ///     The normal of this renderable entity (if supported)
    /// </summary>
    public float3 Normal;

    /// <summary>
    ///     Create a sphere
    /// </summary>
    public static RenderableEntity CreateSphere(float3 position, float3 color, float albedo, float radius)
    {
        return new RenderableEntity
        {
            Type = RenderableEntities.SPHERE,
            Position = position,
            Color = color,
            Albedo = albedo,
            Radius = radius
        };
    }

    /// <summary>
    ///     Create a plane
    /// </summary>
    public static RenderableEntity CreatePlane(float3 position, float3 color, float albedo, float3 normal)
    {
        return new RenderableEntity
        {
            Type = RenderableEntities.PLANE,
            Position = position,
            Color = color,
            Albedo = albedo,
            Normal = normal
        };
    }
}
