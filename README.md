# Graphics

Where I mess around with Rust and WGPU in my own time

Other projects not included in this repo:

- [SpaceChunks](https://github.com/DominicMaas/SpaceChunks) Voxel Engine that I orginally wrote in 2014 using OpenGL and SDL.
- [Project Tital](https://github.com/DominicMaas/ProjectTitan) A rewrite of SpaceChunks but in Vulkan (as a learning exercise to understand Vulkan).
- [Project Apollo](https://github.com/DominicMaas/Apollo) Another game of some sorts, never really finished. Ideas used in this project are incoroperated into Vesta.

Eventually the goal is for all these games to be WebGPU and WebAssembly compatable, so they may be played in the browser. This is probably only going to happen late 2021 when browser support is stablised.

## Vesta

A light game engine that wraps around WGPU. Is extended when I need new features. Supports 2D and 3D games.

## Example

A very simple example using the Vesta engine. Shows the absolute minimum to render a cube and fly around it using a first person controller.

## Projects/Pixel2D

(Under Development) A pixel simulation game

## Projects/Eris

(Under Development) A 3D solar system simulation game

## Projects/Titan

(Under Development) A port of project titan / Space Chunks to the vesta engine. Hopefully actually getting things like physics, infinite chunks, and raycasting working.

![image](https://user-images.githubusercontent.com/5589453/121789150-b062fc00-cc27-11eb-8c96-9bcbdc080f6c.png)

### Initial Goals

Initial goals required to get this project to the supported features of SpaceChunks (OpenGL) and ProjectTitan (Vulkan). Where appropiate, some features will be built into the Vesta engine so other projects can use them (lighting, skybox etc.)

- [x] Move around the world with a camera
- [x] Debug information stating the current position etc
- [x] World with multiple chunks
- [ ] Correct culling of unwanted faces and faces between chunks
- [x] Frustum Culling (probably make this generic and chuck in vesta as well tbh)
- [x] Only render chunks in view of the camera
- [x] Generate new chunks on the fly (and delete old chunks from memory)
- [ ] Normals and lighting
- [ ] Ability to add and remove blocks from chunks (key presses)

### Future Goals

Now for the fun stuff!

- [ ] Using rapier.rs, implement mesh coliders for the chunks, implement a ridged body for the player (and bam, physics!)
- [ ] Raycasting!!! Nobody likes pressing keys to add and remove blocks, use raycasting to make this a true Minecraft clone!
- [ ] Wait, you want to save your progress!? Implement the ability to load and save worlds (this will depend on the next task).\
- [ ] Scene Mangement. Port the scene manager and start working on an ECS system for vesta. Scenes structs are pretty much like ap structs, but you can switch between them! (main menu, pause menu, game, etc.)
