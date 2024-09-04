use std::io;
mod editor;
use editor::*;

mod keyboard;
mod screen;

fn main() -> io::Result<()> {
    
    let mut editor = Editor::new("C:\\Users\\jose\\Desktop\\input.txt")?;

    editor.start()?;

    Ok(())
} 


