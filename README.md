# roguelike experiment

Experimenting with roguelike implementation in Rust, using the `specs` Entity-Component-System framework.

## Implemented

- ncurses graphics
- event bus
- action + energy based AI system
- terrain that can block vision and movement
- pathfinding (using the `pathfinding` crate)
- field-of-view (brute force, using the `line_drawing` crate)
- a `@` that can move around
- NPCs that follow the `@` around
- Occluding terrain outside of FOV

## Todo

- Critters filling space
- Any commands other than movement
- Stats
- Combat
- Fog of war/map memory
- Terrain other than walls
- All kinds of AI stuff
- Map generation
- Movement between maps
- Saving and loading
- Graphical tiles