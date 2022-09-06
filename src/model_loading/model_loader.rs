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
/// # Errors
/// - Object parsing is failed:
///     - OpenFileFailed
///     - ReadError
///     - UnrecognizedCharacter
///     - PositionParseError
///     - NormalParseError
///     - TexcoordParseError
///     - FaceParseError
///     - MaterialParseError
///     - InvalidObjectName
///     - InvalidPolygon
///     - FaceVertexOutOfBounds
///     - FaceTexCoordOutOfBounds
///     - FaceNormalOutOfBounds
///     - FaceColorOutOfBounds
///     - InvalidLoadOptionConfig
///     - GenericFailure
/// - Vertexes have been parsed invalidly
/// - Normals have been parsed invalidly
/// - Models indices exceed u16 max limit *65 535* or are smaller than *0*
///
/// # Examples
/// ```
/// let model = get_obj("path/to/the/model.obj");
/// ```
pub fn get_obj(path: &str) -> Model {
    let (models, _materials) = tobj::load_obj(
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

    assert!(
        m_vtx.len() % 3 == 0,
        "Vertexes of object: {}, was parsed invalidly.",
        path
    );
    assert!(
        m_nor.len() % 3 == 0,
        "Normals of object: {}, was parsed invalidly.",
        path
    );

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
