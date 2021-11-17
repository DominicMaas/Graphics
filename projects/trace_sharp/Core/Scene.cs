namespace TraceSharp.Core;

public struct Scene
{
    public Scene(int width, int height, float fov)
    {
        Width = width;
        Height = height;
        FieldOfView = fov;
        Light = new Light(new float3(-0.25f, -1.0f, -1.0f), new float3(1, 1, 1), 20.0f);
    }

    public int Width;
    public int Height;
    public float FieldOfView;
    public Light Light;
}
