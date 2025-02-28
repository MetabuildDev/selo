# Selo

[glam](https://github.com/bitshifter/glam-rs)-based 2d and 3d geometric primitives with [geo](https://github.com/georust/geo) interoperability.

# Test app controls:

## Drawing Modes:

- `C` - Place point with left click 
- `L` - Draw line with left clicks
- `T` - Draw triangle with left clicks
- `P` - Draw polygon points with left click, close polygon with right click
- `W` - Edit workplane properties via UI
- `ESC` - Go back to Algorithm Mode

## Algorithm Mode

- `Space` - Hold and drag to move camera
- `MiddleMouse` - Hold and drag to rotate camera
- `MouseWheel` - Scroll to "zoom" (moves camera forward and backward)
- `MouseWheel+LeftControl` - Scroll through the Algorithms, some of them have extra UI for interaction
- `LeftClick` - Click points and drag them to move them inside of the current workplane

## Pasting geometry

You can paste geometric objects in various format in the input box to quickly inspect them when debugging your programs. Currently, the following formats are implemented:

| format            | example                                                                                                                                                                                                                                                                                                                  |
|-------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| selo debug output | `Polygon(Ring([Vec2(0.0, 0.0), Vec2(5.0, 0.0), Vec2(5.0, 5.0), Vec2(0.0, 5.0)]), MultiRing([Ring([Vec2(1.0, 1.0), Vec2(2.0, 1.0), Vec2(2.0, 2.0), Vec2(1.0, 2.0)]), Ring([Vec2(3.0, 3.0), Vec2(4.0, 3.0), Vec2(4.0, 4.0), Vec2(3.0, 4.0)])]))`                                                                           |
| geo debug output  | `Polygon { exterior: LineString([Coord { x: 173.45856, y: 77.282646 }, Coord { x: 154.34856, y: 119.78603 }, Coord { x: 143.0181, y: 114.67684 }, Coord { x: 161.94347, y: 72.56411 }, Coord { x: 162.9144, y: 70.43421 }, Coord { x: 174.25348, y: 75.52239 }, Coord { x: 173.45856, y: 77.282646 }]), interiors: [] }` |
| WKT               | `POLYGON Z ((50.373783 -29.84498 8.300001,49.9107 -28.105574 8.300001,50.142242 -28.975283 10.800003,50.373783 -29.84498 8.300001))`                                                                                                                                                                                     |
