using SixLabors.ImageSharp.PixelFormats;
using TerraFX.Numerics;
using TraceSharp.Core.Math;

namespace TraceSharp.Core;

public class Scene
{
    public int Width { get; set; }
    public int Height { get; set; }
    public float FieldOfView { get; set; }

    public Light Light { get; set; } = new Light
    {
        Direction = new Vector3(-0.25f, -1.0f, -1.0f),
        Color = new Vector3(1, 1, 1),
        Intensity = 20.0f
    };

    public IEnumerable<RenderObject> RenderObjects { get; set; } = new List<RenderObject>();

    public Intersection? Trace(Ray ray)
    {
        return RenderObjects
            .Select(x => new { Obj = x, Ray = x.IntersectWithRay(ray) })
            .Where(x => x.Ray.Intersecting)
            .Select(x => new Intersection(x.Ray.Distance, x.Obj))
            .OrderBy(x => x.Distance).FirstOrDefault();
    }
}
