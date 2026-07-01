use crate::glium_area::vertex::Vertex;

#[derive(Debug)]
pub struct ObjLoadError {
    pub message: String,
}

impl std::fmt::Display for ObjLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ObjLoadError {}

fn err(message: impl Into<String>) -> ObjLoadError {
    ObjLoadError {
        message: message.into(),
    }
}

/// Parses cell meshes exported by `export_obj` (all `v` lines, 4 vertices per cell).
pub fn parse_cell_mesh(obj: &str, color: [f32; 4]) -> Result<Vec<Vertex>, ObjLoadError> {
    let mut vertices = Vec::new();

    for line in obj.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.starts_with("vt") || line.starts_with("vn") {
            continue;
        }
        if !line.starts_with('v') {
            continue;
        }
        let coords: Vec<f32> = line[1..]
            .split_whitespace()
            .map(|s| s.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| err(format!("invalid vertex coordinate: {e}")))?;
        if coords.len() < 3 {
            return Err(err(format!("vertex line needs 3 coordinates: {line}")));
        }
        vertices.push(Vertex {
            position: [coords[0], coords[1], coords[2]],
            color,
        });
    }

    if vertices.is_empty() {
        return Err(err("no vertices in OBJ"));
    }
    if vertices.len() % 4 != 0 {
        return Err(err(format!(
            "cell mesh vertex count must be a multiple of 4, got {}",
            vertices.len()
        )));
    }
    Ok(vertices)
}

/// Parses grid line meshes (pairs of vertices per line segment).
pub fn parse_line_mesh(obj: &str, color: [f32; 4]) -> Result<Vec<Vertex>, ObjLoadError> {
    let mut vertices = Vec::new();

    for line in obj.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.starts_with("vt") || line.starts_with("vn") {
            continue;
        }
        if !line.starts_with('v') {
            continue;
        }
        let coords: Vec<f32> = line[1..]
            .split_whitespace()
            .map(|s| s.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| err(format!("invalid vertex coordinate: {e}")))?;
        if coords.len() < 3 {
            return Err(err(format!("vertex line needs 3 coordinates: {line}")));
        }
        vertices.push(Vertex {
            position: [coords[0], coords[1], coords[2]],
            color,
        });
    }

    if vertices.is_empty() {
        return Err(err("no vertices in OBJ"));
    }
    if vertices.len() % 2 != 0 {
        return Err(err(format!(
            "grid mesh vertex count must be even, got {}",
            vertices.len()
        )));
    }
    Ok(vertices)
}
