using System.Numerics;

namespace TraceSharp.Core;

public struct Light
{
    public Light(Vector3 direction, Vector3 color, float intensity)
    {
        Direction = direction;
        Color = color;
        Intensity = intensity;
    }

    public Vector3 Direction;
    public Vector3 Color;
    public float Intensity;
}
