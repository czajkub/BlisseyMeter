use crate::models::{GameState, FieldModifier};

pub fn process_field_modifier(
    state: &mut GameState,
    field_modifier: &Option<FieldModifier>,
    start: bool,
) -> Result<(), String> {
    match field_modifier {
        Some(_mod) => {
            state.field.update_field_modifier(_mod, start);
            Ok(())
        }
        None => {
            Err(format!("Unknown field modifier"))
        }
    }
}
