# Lux

### A 2d game engine built with a focus on user-frendliness and speed.

Lux is a 2d game engine for Rust programmers that makes deploying as
simple as `cargo build`.  The entire Lux dependency stack can be built
using Cargo, so never worry about dependency hell ever again!

### Windowing
Lux can create native windows on OSX, Windows, and Linux (iOS, Android,
and web browsers coming soon).
All mouse and keyboard input is captured and is exposed to the game
developer in two ways:

1. A iterator of event objects (`window.events()`)
2. Input device queries (`window.mos_pos()`, `window.is_key_pressed('A')`)

### 2d Graphics
Lux offers an intuitive and safe graphics API that makes developing 2d games
completely painless!
The API can be used in an entirely stateless maner, and resources are
automatically cleaned up when no longer in use.
We use OpenGL and OpenGL ES under the hood, so most platforms are supported
already or are being planned.

### Game Loop (optional)
Simply by implementing the `update(..)` and `render(..)` methods on the
`Game` trait, you'll be given a robust game loop with a fixed-time
update and lag compensation.


## Credits

* Lux design and implementation by [Ty Overby](https://github.com/TyOverby)
* Dependencies Glutin and Glium by [Tomaka](https://github.com/Tomaka)
* Dependencies Image and Freetype by the [Piston Developers](https://github.com/PistonDevelopers)
