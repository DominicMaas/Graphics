using System.Numerics;

namespace TraceSharp.Core;

public struct Light
{
    public Light(float3 direction, float3 color, float intensity)
    {
        Direction = direction;
        Color = color;
        Intensity = intensity;
    }

    public float3 Direction;
    public float3 Color;
    public float Intensity;
}
