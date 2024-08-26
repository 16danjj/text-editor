use crate::keyboard::*;
use crossterm::event::{KeyEvent, KeyModifiers, KeyCode, KeyEventKind, KeyEventState};

pub fn editor_process_keypress() -> bool {
    
    if let Ok(c) = editor_read_key() {
        match c {
            KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE } => return true,
            _ => {return false}
        }
    }

    false
}