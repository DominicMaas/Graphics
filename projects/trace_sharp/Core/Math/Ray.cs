using TerraFX.Numerics;

namespace TraceSharp.Core.Math;

public class Ray
{
    public Vector3 Origin { get; set; }
    public Vector3 Direction { get; set; }

    public static Ray CreatePrime(int x, int y, Scene scene)
    {
        var fovAdjustment = MathF.Tan((scene.FieldOfView * (MathF.PI / 180.0f)) / 2);
        var aspectRatio = (float)scene.Width / (float)scene.Height;

        var sensorX = ((((x + 0.5f) / scene.Width) * 2.0f - 1.0f) * aspectRatio) * fovAdjustment;
        var sensorY = (1.0f - ((y + 0.5f) / scene.Height) * 2.0f) * fovAdjustment;

        return new Ray
        {
            Origin = new Vector3(),
            Direction = Vector3.Normalize(new Vector3(sensorX, sensorY, -1.0f))
        };
    }
}
