# Solar-System Simuation
### Written in Rust using Bevy

This project is designed to be an n-body simulator for a solar system.

At first, a 2d system will be simulated, to show the mechanics work. Later, I may try to implement a 3d system.

## 2d Goals
- [ ] Simulate our solar system
  - [x] Basic gravity simulation
  - [ ] A moon should be able to orbit a planet which is orbiting a sun
  - [ ] Scale simple model to real life sized numbers
  - [ ] Smaller "playable/navigable" scale
  - [ ] Planet radius should be a factor of it's mass and density?
- [ ] Differing textures for the planets (doesn't have to be accurate)
- [ ] Camera controls
  - [ ] Camera focus on 1 object
  - [ ] Camera auto zoom to show the current system
  - [ ] Show object information when clicked
- [ ] Spawn new objects via a GUI
  - [ ] Drag & let go to add velocity/momentum
  - [ ] or "Orbit" placer, shows a ring around the selected object (the sun usually)
- [ ] Collision & destruction / separation into asteroids
  - Rewatch https://www.youtube.com/watch?v=pm4LLMsKJQg for colission system set solution?
  - [ ] Hitbox should be at the edge of the radius, not the center
  - [ ] Mass addition/subtraction from a collision
    - [ ] Planet will absorb any asteroid 1/100th of it's size, otherwise boom
  - [ ] Planet formation from asteroids?
- [ ] Timekeeping
  - [ ] Be able to rewind time
  - [ ] Show previous path / future path

## 3d Goals
Toggle between 2d and 3d? or just rewrite in 3d...

- [ ] 3d models for all object types
- [ ] Placement of object in 3d space
  - [ ] Switch to a 2d view for simplicity?
- [ ] 3d movement, orbits and collision

## Crazy future goals
- [ ] Configuring planet composition (affects collision, radius, mass etc)
- [ ] Voxel planets / mini spaceship game
