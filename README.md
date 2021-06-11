# Graphics

Where I mess around with Rust and WGPU in my own time

Other projects not included in this repo:
- [SpaceChunks](https://github.com/DominicMaas/SpaceChunks) Voxel Engine that I orginally wrote in 2014 using OpenGL and SDL.
- [Project Tital](https://github.com/DominicMaas/ProjectTitan) A rewrite of SpaceChunks but in Vulkan (as a learning exercise to understand Vulkan).
- [Project Apollo](https://github.com/DominicMaas/Apollo) Another game of some sorts, never really finished. Ideas used in this project are incoroperated into Vesta.

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

### Initial Goals
Initial goals required to get this project to the supported features of SpaceChunks (OpenGL) and ProjectTitan (Vulkan). Where appropiate, some features will be built into the Vesta engine so other projects can use them (lighting, skybox etc.)

- [ ] Move around the world with a camera
- [ ] Debug information stating the current position etc
- [ ] World with multiple chunks
- [ ] Correct culling of unwanted faces and faces between chunks
- [ ] Only render chunks in view of the camera
- [ ] Generate new chunks on the fly (and delete old chunks from memory)
- [ ] Normals and lighting
- [ ] Ability to add and remove blocks from chunks (key presses)

### Future Goals
Now for the fun stuff!

- [ ] Using rapier.rs, implement mesh coliders for the chunks, implement a ridged body for the player (and bam, physics!)
- [ ] Raycasting!!! Nobody likes pressing keys to add and remove blocks, use raycasting to make this a true Minecraft clone!
- [ ] Wait, you want to save your progress!? Implement the ability to load and save worlds (this will depend on the next task).\
- [ ] Scene Mangement. Port the scene manager and start working on an ECS system for vesta. Scenes structs are pretty much like ap structs, but you can switch between them! (main menu, pause menu, game, etc.)
