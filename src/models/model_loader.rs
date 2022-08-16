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

pub(crate) fn get_obj(path:&str) -> Model {
    let input = BufReader::new(File::open(path).unwrap());
    let loaded: Obj = load_obj(input).unwrap();

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
