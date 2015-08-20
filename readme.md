# Lux

## A 2d game engine built with a focus on user-frendliness and speed.

Lux is a Rust library with no external dependencies that provides three main
pieces of functinoality.

1. ### Windowing
   Lux uses [Glutin](https://github.com/tomaka/glutin) to create windows with
   a suitable OpenGL context inside of them.  All mouse and keyboard input is
   captured and given to the user as both a stream of events and as current
   data values that can be queried (eg calling `window.mouse_pos()` will
   return the current mouse position).
2. ### 2d Graphics
   Lux uses [Glium](https://github.com/tomaka/glium) behind the scenes as a
   layer on top of modern OpenGL, but has it's own 2d graphics api that you
   can use to easily draw primitives, sprites, and text.
3. ### Game Loop
   Simply by implementing the `Game` trait, you'll be given a robust game loop
   with a fixed-time update and lag compensation.  The `Game` trait has some
   advanced configuration options, but only requires the programmer to implement
   the `render(...)` and `update(...)` functions.


