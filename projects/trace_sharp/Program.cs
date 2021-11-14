using ComputeSharp;
using System.Diagnostics;
using System.Numerics;
using TraceSharp.Core;
using TraceSharp.Core.Renderable;
using TraceSharp.Shaders;

var scene = new Scene(1920, 1080, 75.0f);
var renderSpheres = new RenderableEntity[]
{
    RenderableEntity.CreateSphere(new Vector3( 0.0f,  0.0f,  -5.0f), new Vector3(1, 0, 0), 0.18f, 1.0f),
    RenderableEntity.CreateSphere(new Vector3( 0.0f,  0.3f,  -3.0f), new Vector3(0, 1, 1), 0.18f, 0.5f),
    RenderableEntity.CreateSphere(new Vector3( 8.0f,  0.0f, -10.0f), new Vector3(0, 1, 0), 0.18f, 1.0f),
    RenderableEntity.CreateSphere(new Vector3(-8.0f, -4.0f, -16.0f), new Vector3(0, 0, 1), 0.18f, 1.0f)
};

using var textureBuffer = Gpu.Default.AllocateReadWriteTexture2D<Rgba32, float4>(scene.Width, scene.Height);
using var renderSphereBuffer = Gpu.Default.AllocateReadOnlyBuffer(renderSpheres);

// Launch the shader
Gpu.Default.For(scene.Width, scene.Height, new RayShader(textureBuffer, scene, renderSphereBuffer));

// Save the texture and then open it
textureBuffer.Save("output.png");
Process.Start("cmd", "/c output.png");
