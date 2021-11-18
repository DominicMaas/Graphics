using ComputeSharp;
using System.Numerics;

namespace TraceSharp.Core.Math;

public struct Ray
{
    public float3 Origin;
    public float3 Direction;

    public Ray(float3 origin, float3 direction)
    {
        Origin = origin;
        Direction = direction;
    }

    public float3 At(float t)
    {
        return Origin + Direction * t;
    }

    public static Ray CreatePrime(int x, int y, Scene scene, int sample, float4x2 jitterMatrix)
    {
        // Determine X and Y
        var sensorX = (2f * (x + jitterMatrix[sample][0]) / scene.Width) - 1f;
        var sensorY = 1f - (2f * (y + jitterMatrix[sample][1]) / scene.Height);

        // Adjust for the aspect ratio
        var aspectRatio = (float)scene.Width / (float)scene.Height;
        sensorX *= aspectRatio;

        // Adjust for the FOV
        var fovAdjustment = Hlsl.Tan((scene.Camera.FOV * (MathF.PI / 180f)) / 2f);
        sensorX *= fovAdjustment;
        sensorY *= fovAdjustment;

#pragma warning disable IDE0017 // Simplify object initialization
        var ray = new Ray();
        ray.Origin = new Vector3();
        ray.Direction = Hlsl.Normalize(new float3(sensorX, sensorY, -1.0f));
#pragma warning restore IDE0017 // Simplify object initialization

        return ray;
    }
}
