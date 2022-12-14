//! Object is 3d model with translation, scale, and rotation in scene space.  

use super::{
    vertex::{Normal, Vertex},
};
use crate::object::{model_loader::Model, Material};
use crate::structs::Scene;
use glium::{IndexBuffer, Program, VertexBuffer};

/// # fields
/// - name: Can be used to distinguish between objects and shows up in the debug menu. **Should be unique.**
/// - model: Loaded `.obj` file should be loaded with `model_loading::model_loader::get_obj()`
/// - position: Set of \[x, y, z] coordinates indicating the translation of the object form the world origin (0, 0, 0).
/// - rotation: Set of \[x, y, z] f32. Indicates object's rotation in **radian**.
/// - scale: Set of \[x, y, z] f32. Indicates the scale of the object on each axis.
/// - tick_update_func: `tick_update_func` is called every drawn frame. For more info about function call order refer to github wiki pages.
/// - on_awake: `on_awake` is called once after model loading is completed.
///
/// Other fields should not be set by the user and should be left as `None`. Usage of `::new()` is strongly recommended.
pub struct GameObject {
    pub name: String,
    pub model: Model,
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub tick_update_func: fn(&mut Scene),
    pub on_awake: fn(&mut Scene),

    pub material: Material,

    pub(crate) program: Option<Program>,
    pub(crate) vertices: Option<VertexBuffer<Vertex>>,
    pub(crate) normals: Option<VertexBuffer<Normal>>,
    pub(crate) indices: Option<IndexBuffer<u16>>,
}

impl GameObject {
    /// creates new 3d model object
    ///
    /// # Examples
    /// ```
    /// let game_object = Object::new(
    ///     "Unique name",
    ///     get_obj("path/to/the/model.obj"),
    ///     [0.0, 0.0, 0.0],
    ///     [0.0, 0.0, 0.0],
    ///     [1.0, 1.0, 1.0],
    ///     blank_tick_update,
    ///     blank_on_awake,
    /// );
    /// ```

    pub fn new(
        name: &str,
        model: Model,
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
        material: Material,
        tick_update_func: fn(&mut Scene),
        on_awake: fn(&mut Scene),
    ) -> Self {
        Self {
            name: String::from(name),
            model: model,
            translation: position,
            rotation: rotation,
            scale: scale,
            material,
            tick_update_func: tick_update_func,
            on_awake: on_awake,

            program: None,
            vertices: None,
            normals: None,
            indices: None,
        }
    }
}
