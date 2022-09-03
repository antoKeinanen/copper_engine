use std::collections::HashSet;

use glium::glutin::event;

/// # Fields
/// - pressed_scancodes: Hash set of u32 numbers indicating key-codes.
/// - modifiers: Contains info if shift, alt, ctrl, or logo is pressed.
/// 
/// Usage of `::new()` is strongly recommended!
pub struct InputManager {
    pub pressed_scancodes: HashSet<u32>,
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
            modifiers: event::ModifiersState::default(),
        }
        
    }
}