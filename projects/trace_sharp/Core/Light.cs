using TerraFX.Numerics;

namespace TraceSharp.Core;

public class Light
{
    public Vector3 Direction { get; set; }
    public Vector3 Color { get; set; }
    public float Intensity { get; set; }
}
