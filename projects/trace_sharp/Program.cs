using SixLabors.ImageSharp;
using SixLabors.ImageSharp.PixelFormats;
using System.Diagnostics;
using TerraFX.Numerics;
using TraceSharp.Core;
using TraceSharp.Core.Math;
using TraceSharp.Core.Primitives;

Vector3 GetColor(Scene scene, Ray ray, Intersection intersection)
{
    var hitPoint = ray.Origin + (ray.Direction * intersection.Distance);
    var surfaceNormal = intersection.Object.SurfaceNormal(hitPoint);
    var directionToLight = -Vector3.Normalize(scene.Light.Direction);

    var shadowRay = new Ray { Origin = hitPoint + (surfaceNormal * new Vector3(0.0001f)), Direction = directionToLight };
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

    for (int y = 0; y < scene.Height; y++)
    {
        var rowSpan = image.GetPixelRowSpan(y);

        for (int x = 0; x < scene.Width; x++)
        {
            // Create ray
            var ray = Ray.CreatePrime(x, y, scene);

            var intersection = scene.Trace(ray);
            if (intersection != null)
            {
                var color = GetColor(scene, ray, intersection);
                rowSpan[x] = new Rgba32(color.X, color.Y, color.Z);
            }
        }
    }

    image.SaveAsPng("output.png");
}

var myScene = new Scene
{
    Width = 3840,
    Height = 2160,
    FieldOfView = 60.0f,
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