use crossterm::event::{KeyEvent, KeyModifiers, KeyCode, KeyEventKind, KeyEventState};
use std::io::{self};
use crossterm::terminal;
use errno::errno;

use crate::keyboard::*;
use crate::screen::*;

pub struct Editor {
    screen : Screen,
    keyboard : Keyboard
}

impl Editor {

    pub fn new() -> io::Result<Self>{
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {}
        })
    }

    pub fn process_keypress(&self) -> bool {
        
        if let Ok(c) = self.keyboard.read() {
            match c {
                KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE } => return true,
                _ => {return false}
            }
        }

        false
    }

    pub fn start(&mut self) -> io::Result<()> {

        terminal::enable_raw_mode()?;

        loop {
            if self.refresh_screen().is_err()
            {
                self.die("unable to refresh screen");
            }
            
            self.screen.flush_op()?;
            
            if self.process_keypress(){
                break;
            }  
        }
    
        terminal::disable_raw_mode()?; 

        Ok(())
    }

    pub fn refresh_screen(&mut self) -> io::Result<()>{

        self.screen.clear()?;  
        self.screen.draw_rows()?;
        self.screen.move_cursor()
        
    }

    pub fn die<S : Into<String>>(&mut self, message : S){
        let _ = self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}:{}", message.into(), errno());
        std::process::exit(1);
    }

}