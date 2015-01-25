```rust
frame.rect(10, 10, 50, 50).fill();
frame.rect((10, 10), (50, 50)).fill();
frame.draw(rect(10, 10, 50, 50).fill());
frame.draw(rect((10, 10), (50, 50)).fill());

frame.sprite(sprt, 10, 10, 50, 50);
frame.sprite(sprt, (10, 10), (50, 50));

frame.print("foo", 10, 10);
```

