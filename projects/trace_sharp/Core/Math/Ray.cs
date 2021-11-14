using System.Numerics;

namespace TraceSharp.Core.Math;

public struct Ray
{
    public Vector3 Origin;
    public Vector3 Direction;

    public Ray(Vector3 origin, Vector3 direction)
    {
        Origin = origin;
        Direction = direction;
    }

    public Vector3 At(float t)
    {
        return Origin + Direction * t;
    }

    public static Ray CreatePrime(int x, int y, Scene scene)
    {
        var fovAdjustment = MathF.Tan((scene.FieldOfView * (MathF.PI / 180.0f)) / 2);
        var aspectRatio = (float)scene.Width / (float)scene.Height;

        var sensorX = ((((x + 0.5f) / scene.Width) * 2.0f - 1.0f) * aspectRatio);
        var sensorY = (1.0f - ((y + 0.5f) / scene.Height) * 2.0f);

        // Adjust for the FOV
        sensorX *= fovAdjustment;
        sensorY *= fovAdjustment;

#pragma warning disable IDE0017 // Simplify object initialization
        var ray = new Ray();
        ray.Origin = new Vector3();
        ray.Direction = Vector3.Normalize(new Vector3(sensorX, sensorY, -1.0f));
#pragma warning restore IDE0017 // Simplify object initialization

        return ray;
    }
}
