use std::io;
mod editor;
use editor::*;

mod keyboard;
mod screen;

fn main() -> io::Result<()> {

    let mut args = std::env::args();
    
    let mut editor = if args.len() >=2 {
        Editor::with_file(args.nth(1).unwrap())?
    }
    else {
        Editor::new()?
    };

    editor.start()?;

    Ok(())
} 


