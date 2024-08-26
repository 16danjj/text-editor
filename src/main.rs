use crossterm::{event::Event::*, terminal};
use std::io;
use errno::errno;

mod keyboard;


mod input;
use input::*;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if editor_process_keypress(){
            break;
        }  
    }
    terminal::disable_raw_mode()?;
    Ok(())
} 


fn die<S : Into<String>>(message : S){
    let _ = terminal::disable_raw_mode();
    eprintln!("{}:{}", message.into(), errno());
    std::process::exit(1);
}