using TerraFX.Numerics;
using TraceSharp.Core.Math;

namespace TraceSharp.Core.Primitives;

public class Sphere : RenderObject
{
    public float Radius { get; set; }

    public override (bool, float) IntersectWithRay(Ray ray)
    {
        // Create a line segment between the ray origin and the center of the sphere
        var line = Position - ray.Origin;

        // Use line as a hypotenuse and find the length of the adjacent side
        var adjacent = Vector3.Dot(line, ray.Direction);

        // Find the length-squared of the opposite side
        var length2 = Vector3.Dot(line, line) - (adjacent * adjacent);

        // Determine the radius squared
        var radius2 = Radius * Radius;

        // If that length-squared is greater than radius squared, the ray doesn't interact the sphere
        if (length2 > radius2)
        {
            return (false, 0.0f);
        }

        var thc = MathF.Sqrt(radius2 - length2);
        var t0 = adjacent - thc;
        var t1 = adjacent + thc;

        if (t0 < 0.0f && t1 < 0.0f)
        {
            return (false, 0.0f);
        }

        // Determine the intersect distance
        var distance = t0 < t1 ? t0 : t1;
        return (true, distance);
    }

    public override Vector3 SurfaceNormal(Vector3 hitPoint)
    {
        return Vector3.Normalize(hitPoint - Position);
    }
}
