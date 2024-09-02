use crossterm::event::{KeyEvent, KeyModifiers, KeyCode, KeyEventKind, KeyEventState};
use std::io::{self};
use crossterm::terminal;
use errno::errno;

use crate::keyboard::*;
use crate::screen::*;

#[derive(Default)]
pub struct Position {
    pub x : u16,
    pub y : u16
}

pub struct Editor {
    screen : Screen,
    keyboard : Keyboard,
    cursor : Position
}

impl Editor {

    pub fn new() -> io::Result<Self>{
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : Position::default()
        })
    }

    pub fn process_keypress(&mut self) -> io::Result<bool> {
        
        if let Ok(c) = self.keyboard.read() {
            match c {
                KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE } => return Ok(true),
                _ => {return Ok(false)}
            }
        }

        Ok(false)
    }

    pub fn start(&mut self) -> io::Result<()> {

        terminal::enable_raw_mode()?;

        loop {
            if self.refresh_screen().is_err()
            {
                self.die("unable to refresh screen");
            }

            self.screen.move_to(&self.cursor)?;
            
            self.screen.flush_op()?;
            
            if self.process_keypress()?{
                break;
            }  
        }
    
        terminal::disable_raw_mode()?; 

        Ok(())
    }

    pub fn refresh_screen(&mut self) -> io::Result<()>{

        self.screen.clear()?;  
        self.screen.draw_rows()
    }

    pub fn die<S : Into<String>>(&mut self, message : S){
        let _ = self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}:{}", message.into(), errno());
        std::process::exit(1);
    }

}