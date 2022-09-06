use crate::structs::vertex::{Normal, Vertex};

// **Should not be created by user: use `::get_obj()`**
#[derive(Debug)]
pub struct Model {
    pub positions: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>,
}

/// loads and parses wavefront OBJ file into copper engine compatible format.
///
/// # Examples
/// ```
/// let model = get_obj("path/to/the/model.obj");
/// ```
pub fn get_obj(path: &str) -> Model {
    let (models, materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            ..Default::default()
        },
    )
    .expect(format!("Failed to load or parse provided obj: {}", path).as_str());

    let model = &models[0];

    let m_vtx = &model.mesh.positions;
    let m_nor = &model.mesh.normals;
    let m_ind = &model.mesh.indices;

    let mut vertexes: Vec<Vertex> = vec![];
    let mut normals: Vec<Normal> = vec![];
    let mut indices: Vec<u16> = vec![];

    assert!(m_vtx.len() % 3 == 0);
    assert!(m_nor.len() % 3 == 0);

    for vtx in 0..m_vtx.len() / 3 {
        vertexes.push(Vertex {
            position: [m_vtx[3 * vtx], m_vtx[3 * vtx + 1], m_vtx[3 * vtx + 2]],
        });
    }

    for nor in 0..model.mesh.normals.len() / 3 {
        normals.push(Normal {
            normal: [m_nor[3 * nor], m_nor[3 * nor + 1], m_nor[3 * nor + 2]],
        })
    }

    for ind in m_ind {
        assert!(
            *ind >= u16::MIN.into() && *ind <= u16::MAX.into(),
            "Failed to parse obj {}, file is too large!",
            path
        );
        indices.push(*ind as u16);
    }

    Model {
        positions: vertexes,
        normals,
        indices,
    }
}