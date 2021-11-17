namespace TraceSharp.Core;

public struct Scene
{
    public Scene(int width, int height, float fov)
    {
        Width = width;
        Height = height;
        Light = new Light(new float3(-0.25f, -1.0f, -1.0f), new float3(1, 1, 1), 20.0f);
        Camera = new Camera { Direction = new float3(0, 0, 0), Position = new float3(0, 0, 0), FOV = fov };
    }

    public int Width;
    public int Height;
    public Light Light;
    public Camera Camera;
}
