using ComputeSharp;
using System.Diagnostics;
using TraceSharp.Core;
using TraceSharp.Core.Renderable;
using TraceSharp.Shaders;

var scene = new Scene(3840, 2160, 90.0f);
var renderSpheres = new RenderableEntity[]
{
    RenderableEntity.CreateSphere(new float3( 0.0f,  0.0f,  -5.0f), new float3(1, 0, 0), 0.18f, 1.0f),
    RenderableEntity.CreateSphere(new float3( 3.0f,  0.3f,  -3.0f), new float3(0, 1, 1), 0.18f, 0.5f),
    RenderableEntity.CreateSphere(new float3( 8.0f,  0.0f, -10.0f), new float3(0, 1, 0), 0.18f, 1.0f),
    RenderableEntity.CreateSphere(new float3(-8.0f, -0.5f, -16.0f), new float3(0, 0, 1), 0.48f, 1.5f),
    RenderableEntity.CreatePlane(new float3(  0.0f, -2.0f,   0.0f), new float3(0.2f, 0.2f, 0.2f), 0.18f, new float3(0f,-1f,0f))
};

using var textureBuffer = GraphicsDevice.Default.AllocateReadWriteTexture2D<Rgba32, float4>(scene.Width, scene.Height);
using var renderSphereBuffer = GraphicsDevice.Default.AllocateReadOnlyBuffer(renderSpheres);

// Launch the shader
GraphicsDevice.Default.For(scene.Width, scene.Height, new RayShader(textureBuffer, scene, renderSphereBuffer));

// Save the texture and then open it
textureBuffer.Save("output.png");
Process.Start("cmd", "/c output.png");
