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
    public Vector3 Position;

    /// <summary>
    ///     The color of this render object (this ray tracer is very basic!!!)
    /// </summary>
    public Vector3 Color;

    /// <summary>
    ///     The albedo of this render object (this ray tracer is very basic!!!)
    /// </summary>
    public float Albedo;

    /// <summary>
    ///     The radius of this shape (if supported)
    /// </summary>
    public float Radius;

    /// <summary>
    ///     Create a sphere
    /// </summary>
    public static RenderableEntity CreateSphere(Vector3 position, Vector3 color, float albedo, float radius)
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
}
