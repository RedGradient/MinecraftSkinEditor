# Body part meshes (Wavefront OBJ)

These files are loaded at compile time by `src/glium_area/model/meshes.rs` (`include_str!`). Each **cell** of the skin UV layout is a separate quad (`g cell_N`), with the same winding as `generate_indexes()`.

Edit the `.obj` files to change geometry, then rebuild the app.

## Cell meshes

| File | Used in app for | Cells |
|------|-----------------|-------|
| `head.obj` | Head (inner + outer share shape) | 384 |
| `body.obj` | Torso | 352 |
| `limb_4x12x4.obj` | Classic arms, legs | 224 |
| `limb_3x12x4.obj` | Slim arms | 192 |

Vertex positions are in **part-local space** (unit cubes / cuboids before `translation` and `scale` in `Renderer`).

## Grid overlays

| File | Description |
|------|-------------|
| `head_grid.obj` | Line grid for head |
| `body_grid.obj` | Line grid for torso |
| `limb_4x12x4_grid.obj` | Line grid for 4×12×4 limb |
| `limb_3x12x4_grid.obj` | Line grid for 3×12×4 limb |

Grid files use `l` (line) elements; the editor draws them with `GL_LINES`.

## Regenerate

```bash
cargo test write_obj_assets -- --nocapture
```
