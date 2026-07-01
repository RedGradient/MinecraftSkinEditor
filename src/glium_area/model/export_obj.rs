//! Regenerate Wavefront OBJ assets from embedded meshes (round-trip check).

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use crate::glium_area::model::generate_indexes;
use crate::glium_area::model::meshes::{
    body_grid, body_vertices, cuboid_3x12x4, cuboid_4x12x4, grid_3x12x4, grid_4x12x4, head_grid,
    head_vertices,
};
use crate::glium_area::vertex::Vertex;

const MODELS_DIR: &str = "resources/models";

struct ModelSpec {
    filename: &'static str,
    object_name: &'static str,
    header: &'static str,
}

const CELL_MODELS: &[ModelSpec] = &[
    ModelSpec {
        filename: "head.obj",
        object_name: "head",
        header: "8x8x8 cube. Face order: front, left, back, right, top, bottom (64 cells each).",
    },
    ModelSpec {
        filename: "body.obj",
        object_name: "body",
        header: "Torso. Face order: front, left, back, right, top, bottom. Cells: 96+48+96+48+32+32.",
    },
    ModelSpec {
        filename: "limb_4x12x4.obj",
        object_name: "limb_4x12x4",
        header: "Classic arm / leg (4x12x4). Face order: front, left, back, right, top, bottom.",
    },
    ModelSpec {
        filename: "limb_3x12x4.obj",
        object_name: "limb_3x12x4",
        header: "Slim arm (3x12x4). Face order: front, left, back, right, top, bottom.",
    },
];

const GRID_MODELS: &[(&str, &str, fn() -> &'static [Vertex])] = &[
    ("head_grid.obj", "head_grid", head_grid),
    ("body_grid.obj", "body_grid", body_grid),
    ("limb_4x12x4_grid.obj", "limb_4x12x4_grid", grid_4x12x4),
    ("limb_3x12x4_grid.obj", "limb_3x12x4_grid", grid_3x12x4),
];

fn write_cell_mesh(
    writer: &mut impl Write,
    vertices: &[Vertex],
    object_name: &str,
    header: &str,
) -> io::Result<()> {
    let cell_count = vertices.len() / 4;
    writeln!(writer, "# Minecraft Skin Editor — cell mesh")?;
    writeln!(writer, "#")?;
    for line in header.lines() {
        writeln!(writer, "# {line}")?;
    }
    writeln!(writer, "o {object_name}")?;

    for v in vertices {
        let p = v.position;
        writeln!(writer, "v {} {} {}", p[0], p[1], p[2])?;
    }

    let indices = generate_indexes(cell_count);
    for cell in 0..cell_count {
        writeln!(writer, "g cell_{cell}")?;
        let base = cell * 6;
        let a = indices[base] as usize + 1;
        let b = indices[base + 1] as usize + 1;
        let c = indices[base + 2] as usize + 1;
        let d = indices[base + 3] as usize + 1;
        let e = indices[base + 4] as usize + 1;
        let f = indices[base + 5] as usize + 1;
        writeln!(writer, "f {a} {b} {c}")?;
        writeln!(writer, "f {d} {e} {f}")?;
    }
    Ok(())
}

fn write_line_mesh(
    writer: &mut impl Write,
    vertices: &[Vertex],
    object_name: &str,
) -> io::Result<()> {
    writeln!(writer, "# Minecraft Skin Editor — grid lines")?;
    writeln!(writer, "o {object_name}")?;

    for v in vertices {
        let p = v.position;
        writeln!(writer, "v {} {} {}", p[0], p[1], p[2])?;
    }

    for line in 0..(vertices.len() / 2) {
        let a = line * 2 + 1;
        let b = line * 2 + 2;
        writeln!(writer, "g line_{line}")?;
        writeln!(writer, "l {a} {b}")?;
    }
    Ok(())
}

pub fn export_all(base: &Path) -> io::Result<()> {
    fs::create_dir_all(base)?;

    let datasets: [(&ModelSpec, &'static [Vertex]); 4] = [
        (&CELL_MODELS[0], head_vertices()),
        (&CELL_MODELS[1], body_vertices()),
        (&CELL_MODELS[2], cuboid_4x12x4()),
        (&CELL_MODELS[3], cuboid_3x12x4()),
    ];

    for (spec, vertices) in datasets {
        let path = base.join(spec.filename);
        let mut file = File::create(&path)?;
        write_cell_mesh(&mut file, vertices, spec.object_name, spec.header)?;
        println!("wrote {} ({} cells)", path.display(), vertices.len() / 4);
    }

    for (filename, object_name, generator) in GRID_MODELS {
        let vertices = generator();
        let path = base.join(filename);
        let mut file = File::create(&path)?;
        write_line_mesh(&mut file, vertices, object_name)?;
        println!(
            "wrote {} ({} lines)",
            path.display(),
            vertices.len() / 2
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glium_area::model::meshes;

    #[test]
    fn embedded_meshes_have_expected_topology() {
        assert_eq!(meshes::head_vertices().len() / 4, 384);
        assert_eq!(meshes::body_vertices().len() / 4, 352);
        assert_eq!(meshes::cuboid_4x12x4().len() / 4, 224);
        assert_eq!(meshes::cuboid_3x12x4().len() / 4, 192);
        assert_eq!(meshes::head_grid().len() % 2, 0);
    }

    #[test]
    fn write_obj_assets() {
        export_all(Path::new(MODELS_DIR)).expect("export OBJ models");
    }
}
