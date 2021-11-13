using SixLabors.ImageSharp.PixelFormats;
using TerraFX.Numerics;
using TraceSharp.Core.Math;

namespace TraceSharp.Core;

/// <summary>
///     A base render object within the renderer
/// </summary>
public abstract class RenderObject
{
    /// <summary>
    ///     The position of this object
    /// </summary>
    public Vector3 Position { get; set; }

    /// <summary>
    ///     The color of this render object (this ray tracer is very basic!!!)
    /// </summary>
    public Vector3 Color { get; set; }

    /// <summary>
    ///     The albedo of this render object (this ray tracer is very basic!!!)
    /// </summary>
    public float Albedo { get; set; } = 0.18f;

    /// <summary>
    ///     Determine if a provided ray intersects with this object
    /// </summary>
    /// <param name="ray">The ray to test</param>
    /// <returns>True if intersecting, otherwise false</returns>
    public abstract (bool Intersecting, float Distance) IntersectWithRay(Ray ray);

    public abstract Vector3 SurfaceNormal(Vector3 hitPoint);
}
