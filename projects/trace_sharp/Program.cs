using SixLabors.ImageSharp;
using SixLabors.ImageSharp.PixelFormats;
using System.Diagnostics;
using TerraFX.Numerics;
using TraceSharp.Core;
using TraceSharp.Core.Math;
using TraceSharp.Core.Primitives;

var JitterMatrix = new float[4 * 2] {
    -1.0f/4.0f,  3.0f/4.0f,
     3.0f/4.0f,  1.0f/3.0f,
    -3.0f/4.0f, -1.0f/4.0f,
     1.0f/4.0f, -3.0f/4.0f,
};

Vector3 GetColor(Scene scene, Ray ray, Intersection intersection)
{
    var hitPoint = ray.Origin + (ray.Direction * intersection.Distance);
    var surfaceNormal = intersection.Object.SurfaceNormal(hitPoint);
    var directionToLight = -Vector3.Normalize(scene.Light.Direction);

    var shadowRay = new Ray(hitPoint + (surfaceNormal * new Vector3(0.0001f)), directionToLight);
    var inLight = scene.Trace(shadowRay) == null;

    var lightIntensity = inLight ? scene.Light.Intensity : 0.0f;
    var lightPower = Vector3.Dot(surfaceNormal, directionToLight) * lightIntensity;
    var lightReflected = intersection.Object.Albedo / MathF.PI;

    var color = intersection.Object.Color * scene.Light.Color * lightPower * lightReflected;
    return color;
}

void Render(Scene scene)
{
    using var image = new Image<Rgba32>(scene.Width, scene.Height);

    // how many segments to split the image up to (how many threads)
    const int HeightSegments = 16;
    var HeightPerThread = scene.Height / HeightSegments;

    void ProcessHeightSegment(int y0, int y1)
    {
        for (int y = y0; y < y1; y++)
        {
            var rowSpan = image.GetPixelRowSpan(y);
            for (int x = 0; x < scene.Width; x++)
            {
                var color = new Vector3(0, 0, 0);

                // Create ray
                var ray = Ray.CreatePrime(x, y, scene);

                var intersection = scene.Trace(ray);
                if (intersection != null)
                {
                    color = GetColor(scene, ray, intersection);
                }

                rowSpan[x] = new Rgba32(color.X, color.Y, color.Z);
            }
        }
    }

    Parallel.For(0, HeightSegments, i => {
        var y0 = HeightPerThread * i;
        var y1 = (HeightPerThread * i) + HeightPerThread;

        ProcessHeightSegment(y0, y1);
    });

    image.SaveAsPng("output.png");
}

var myScene = new Scene
{
    Width = 1920,
    Height = 1080,
    FieldOfView = 75.0f,
    RenderObjects = new RenderObject[]
    {
        new Sphere { Position = new Vector3( 0.0f,  0.0f,  -5.0f), Radius = 1.0f, Color = new Vector3(1, 0, 0) },
        new Sphere { Position = new Vector3( 0.0f,  0.3f,  -3.0f), Radius = 0.5f, Color = new Vector3(0, 1, 1) },
        new Sphere { Position = new Vector3( 8.0f,  0.0f, -10.0f), Radius = 1.0f, Color = new Vector3(0, 1, 0) },
        new Sphere { Position = new Vector3(-8.0f, -4.0f, -16.0f), Radius = 1.0f, Color = new Vector3(0, 0, 1) }
    }
};

var watch = new System.Diagnostics.Stopwatch();
watch.Start();

Render(myScene);

watch.Stop();

Console.WriteLine($"Finished in {watch.ElapsedMilliseconds}ms!");

Process.Start("cmd", "/c output.png");

Console.ReadLine();