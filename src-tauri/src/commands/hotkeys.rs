use tauri::State;

use crate::app::state::RuntimeState;
use crate::domain::hotkeys::Keybind;

#[tauri::command]
pub fn sync_keybinds(
    keybinds: Vec<Keybind>,
    state: State<'_, RuntimeState>,
) -> Result<Vec<Keybind>, String> {
    {
        let mut data = state
            .data
            .lock()
            .map_err(|_| "Settings lock poisoned".to_string())?;
        data.keybinds = keybinds.clone();
        let snapshot = data.clone();
        state.save(&snapshot)?;
    }
    crate::domain::hotkeys::sync_hotkeys(&keybinds);
    Ok(keybinds)
}

#[tauri::command]
pub fn get_keybinds(state: State<'_, RuntimeState>) -> Result<Vec<Keybind>, String> {
    let snapshot = state.snapshot()?;
    Ok(snapshot.keybinds)
}
