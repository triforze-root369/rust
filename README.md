# Rust Ray Tracer

A simple ray tracer implementation in Rust that demonstrates 3D graphics programming, performance optimization, and mathematical concepts in computer graphics.

## Features

- Renders 3D scenes with spheres
- Supports diffuse lighting with shadows
- Configurable camera with perspective projection
- Parallel rendering using rayon
- Outputs to PPM image format
- Command-line interface for image resolution

## Requirements

- Rust (latest stable version)
- Cargo package manager

## Building and Running

1. Build the project:
```bash
cargo build --release
```

2. Run the ray tracer with desired resolution:
```bash
cargo run --release -- 800 600
```

This will create an `output.ppm` file in the current directory.

## Scene Description

The default scene contains:
- Three spheres:
  - Red sphere at (0, 0, -5)
  - Green sphere at (2, 0, -6)
  - Blue sphere at (-2, 0, -4)
- Point light source at (5, 5, 5)
- Camera at origin (0, 0, 0) looking down the negative z-axis

## Converting PPM to PNG

To convert the PPM output to a more common format like PNG, you can use ImageMagick:

```bash
convert output.ppm output.png
```

## Implementation Details

### Ray-Sphere Intersection

The ray-sphere intersection is calculated using the quadratic formula:
```
(P + tD - C) · (P + tD - C) = r²

where:
P = ray origin
D = ray direction
C = sphere center
r = sphere radius
t = intersection distance
```

### Performance Optimizations

1. Parallel rendering using rayon's parallel iterator
2. Minimal allocations with pre-allocated vectors
3. Efficient vector operations with SIMD-friendly structure
4. Early exit for shadow rays

## Code Structure

- `Vec3`: 3D vector implementation with common operations
- `Ray`: Ray representation with origin and direction
- `Sphere`: Sphere object with intersection testing
- `Camera`: Perspective camera with configurable parameters
- `Scene`: Scene management and ray tracing logic

## License

MIT 