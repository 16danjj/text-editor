use crossterm::terminal;
use std::io;
mod editor;
use editor::*;




fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new()?;

    loop {
        if editor.refresh_screen().is_err()
        {
            editor.die("unable to refresh screen");
        }

        if editor.process_keypress(){
            break;
        }  
    }

    terminal::disable_raw_mode()?;
    Ok(())
} 


