# Solar-System Simuation
### Written in Rust using Bevy

This project is designed to be an n-body simulator for a solar system.

At first, a 2d system will be simulated, to show the mechanics work. Later, I may try to implement a 3d system.

## 2d Goals
- [ ] Simulate our solar system
  - [x] Basic gravity simulation
  - [ ] Scale simple model to real life sized numbers
  - [ ] A moon should be able to orbit a planet which is orbiting a sun
  - [ ] Smaller "playable/navigable" scale
  - [ ] Planet radius should be a factor of it's mass and density?
- [ ] Differing textures for the planets (doesn't have to be accurate)
- [ ] Background with starts of different shades of white & scattering of other colours
- [ ] Camera controls
  - [ ] Camera focus on 1 object
  - [ ] Camera auto zoom to show the current system
      - [ ] Auto follow the largest object on launch
      - [ ] Generally "follow" the largest mass (shower thought)
  - [ ] Show object information when clicked
- [ ] Spawn new objects via a GUI
  - [ ] Drag & let go to add velocity/momentum
  - [ ] or "Orbit" placer, shows a ring around the selected object (the sun usually)
      - Shows future orbit (todo write document on how to predict orbits)
      - Show tails (KISS)
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
- [ ] Heat/temperature from proximity to the sun/other planets and affecting water/ice etc


### Dev history / coming back from a break

Something to keep in mind during development
> I look at it this way: if your system's purpose is to call a method on a component, then the system IS the method. This was something I had to get used to when transitioning from OOP to ECS.
> https://redd.it/16lcs6o


Create a single object system for easier testing. Test/implement the radius calculation from mass, density etc
Then make the camera auto zoom work for one body, then two
you could even do the orbit calculations with this setup

Also time.. I need to fix time. Fast forwarding, rewinding etc
// Time fix? https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html

then hopefully be familiar with bevy again to fix the click actions and continue with the goals.
  It seems like bevy_mod_picking has been changed to be more agnostic and work with a variety of rendering backends
  Source:
  https://docs.rs/bevy_mod_picking/0.17.0/bevy_mod_picking/index.html
    > You will eventually need to choose which picking backend(s) you want to use. This plugin uses bevy_mod_raycast by default; it works with bevy Meshes out of the box and requires no extra dependencies. These qualities make it useful when prototyping, however it is not particularly performant for large meshes. Consider switching to the rapier backend if performance becomes a problem or if you already have the dependency in-tree. For simple or low-poly games, it may never be an issue.

  I don't know if I can/should use this bevy_mod_picking library for spawning new entities with a drag + drop.
  Unless I immediately spawn the entity, excude it from the gravity calculations, then while still clicked, drag + let go (because I need an entity to exist to click it)
  This may give me what I need. But I need to make sure it's adding value and not just using the library for the sake of it
  Like, could I just add a click + drag line myself? Then spawn in with the correct variables?

tldr; add click events back to the entities

I do need to properly investigate the rapier engine

## TIME
Thankyou to this guide 🙏🙏🙏
https://github.com/bevyengine/bevy/pull/10204/files

### Rough guide as-is

Alright, so we have 3 time dimensions here:

- Real time - that's real to use (tracked by bevy)
- Virtual time - a "scaled" time, also controlled by bevy. Which the excellent guide above describes. Allows slomo and pause. But does not go backwards
- SimTime - This is my time. Allows going backwards and can allow spawn events to be timestamped

Within `update_positions`, I use a `flow` variable to reverse the position result, by reversing the time constant in the calculations if my SimTime is decrementing

To ensure SimTime stays in step with the VirtualTime, we always increment and decrement it by the virutal time's (delta) seconds. SimTime == VirtualTime (sort of)!


**Any bodies that are spawned, destroyed or updated must be logged against a `SimTime` and dealt with at the time the event happens. So if we rewound time to before they existed, they won't cause issues by still being present in calculations**

- This is a current issue.

### How to deal with X when rewinding time:
**Spawned**
- Put entity id in table with SpawnTime
- Remove Simulated and GUI components
**Destroyed**
- Log position
- I am out of ideas
**Updated**
