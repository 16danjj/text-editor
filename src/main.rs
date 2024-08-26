use crossterm::{event::Event::*, terminal};
use std::io;


mod keyboard;
mod output;
use output::*;

mod input;
use input::*;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if editor_refresh_screen().is_err()
        {
            die("unable to refresh screen");
        }

        if editor_process_keypress(){
            break;
        }  
    }

    terminal::disable_raw_mode()?;
    Ok(())
} 


