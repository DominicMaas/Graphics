using System.Numerics;

namespace TraceSharp.Core;

public struct Scene
{
    public Scene(int width, int height, float fov)
    {
        Width = width;
        Height = height;
        FieldOfView = fov;
        Light = new Light(new Vector3(-0.45f, -2.0f, -1.0f), new Vector3(1, 1, 1), 20.0f);
    }

    public int Width;
    public int Height;
    public float FieldOfView;
    public Light Light;
}
