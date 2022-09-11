//! Input manager is responsible of storing user input.

use std::collections::HashSet;
use glium::glutin::event;

/// # Fields
/// - pressed_scancodes: Hash set of u32 numbers indicating key-codes.
/// - mouse_position: position of the mouse inside of the window. (0, 0) is in the top left corner.
/// - modifiers: Contains info if shift, alt, ctrl, or logo is pressed.
/// 
/// Usage of `::new()` is strongly recommended!
pub struct InputManager {
    pub pressed_scancodes: HashSet<u32>,
    pub mouse_position: [f64; 2],
    pub modifiers: event::ModifiersState,
}

impl InputManager {
    /// creates new Input manager 
    /// 
    /// # Examples
    /// ```
    /// let im = InputManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            pressed_scancodes: HashSet::new(),
            mouse_position: [0.0, 0.0],
            modifiers: event::ModifiersState::default(),
        }
        
    }
}