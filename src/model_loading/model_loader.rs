use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

use crate::structs::vertex::{Normal, Vertex};

#[derive(Debug)]
pub struct Model {
    pub positions: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>,
}

/// loads and parses wavefront OBJ file into copper engine compatible format.
/// 
/// # Errors
/// - Path provided does not already exist.
/// - IO error has been occurred during opening the `obj` file.
/// - Tried to parse integer from the `obj` file, but failed.
/// - Tried to parse floating point number from the `obj` file, but failed.
/// - `LoadError` has been occurred during parsing the `obj` file.
/// - Other errors may also be returned according to [`OpenOptions::open`].
/// 
/// # Examples
/// ```
/// let model = get_obj("path/to/the/model.obj");
/// ```
pub fn get_obj(path:&str) -> Model {
    let input = BufReader::new(File::open(path).expect("Unable to open path!"));
    let loaded: Obj = load_obj(input).expect("Failed to load obj from provided buffer reader!");

    let mut model = Model {
        positions: vec![],
        normals: vec![],
        indices: loaded.indices,
    };

    for vertex in loaded.vertices {
        model.positions.push(Vertex {
            position: vertex.position,
        });
        model.normals.push(Normal {
            normal: vertex.normal,
        });
    }
    
    model
}
