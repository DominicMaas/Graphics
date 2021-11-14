using ComputeSharp;
using SixLabors.ImageSharp;
using System.Diagnostics;
using System.Numerics;
using TraceSharp.Core;
using TraceSharp.Core.Renderable;
using TraceSharp.Shaders;
using Rgba32 = SixLabors.ImageSharp.PixelFormats.Rgba32;

var scene = new Scene(1920, 1080, 75.0f);
var renderSpheres = new RenderableEntity[]
{
    RenderableEntity.CreateSphere(new Vector3( 0.0f,  0.0f,  -5.0f), new Vector3(1, 0, 0), 0.18f, 1.0f),
    RenderableEntity.CreateSphere(new Vector3( 0.0f,  0.3f,  -3.0f), new Vector3(0, 1, 1), 0.18f, 0.5f),
    RenderableEntity.CreateSphere(new Vector3( 8.0f,  0.0f, -10.0f), new Vector3(0, 1, 0), 0.18f, 1.0f),
    RenderableEntity.CreateSphere(new Vector3(-8.0f, -4.0f, -16.0f), new Vector3(0, 0, 1), 0.18f, 1.0f)
};

// Create a storage array for all pixels colors (rgb, 0-1)
var pixelData = new Vector3[scene.Width * scene.Height];

// Allocate a GPU buffer and copy the data to it.
// We want the shader to modify the items in-place, so we
// can allocate a single read-write buffer to work on.
using var buffer = Gpu.Default.AllocateReadWriteBuffer(pixelData);
using var renderSphereBuffer = Gpu.Default.AllocateReadOnlyBuffer(renderSpheres);

var watch = new Stopwatch();
watch.Start();

// Launch the shader
Gpu.Default.For(buffer.Length, new RayShader(buffer, scene, renderSphereBuffer));

// Get the data back
buffer.CopyTo(pixelData);

watch.Stop();
Debug.WriteLine($"Finished GPU in {watch.ElapsedMilliseconds}ms!");

using var image = Image.LoadPixelData(pixelData.Select(x => new Rgba32(x)).ToArray(), scene.Width, scene.Height);
image.SaveAsPng("output.png");

Process.Start("cmd", "/c output.png");
