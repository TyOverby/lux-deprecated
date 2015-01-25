```rust
frame.rect(10, 10, 50, 50).fill();
frame.rect((10, 10), (50, 50)).fill();
frame.draw(rect(10, 10, 50, 50).fill());
frame.draw(rect((10, 10), (50, 50)).fill());

frame.sprite(sprt, 10, 10, 50, 50);
frame.sprite(sprt, (10, 10), (50, 50));

<<<<<<< HEAD
frame.print("foo", 10, 10);
```

=======
NOTE: border and padding seriously screw with rotate_around.
>>>>>>> parent of 886b3e8... figurized most things
