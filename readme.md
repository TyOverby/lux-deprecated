```rust
lux.rect(pos, sz).draw();
lux.rect(pos, sz).margin(4.0).draw();
lux.rect(pos, sz).border(5.0).draw();
lux.rect(pos, sz).border(5.0).stroke();
```

NOTE: border and padding seriously screw with rotate_around.
