use crossterm::event::{read, KeyEvent, Event::*};

pub struct Keyboard;

impl Keyboard {
    pub fn read(&self)->Result<KeyEvent, String>{
        loop{
            if let Ok(event) = read(){
                if let Key(key_event) = event {
                   return Ok(key_event)
                } 
            } else{
                return Err(String::from("Read failed"))
            }
        }
    }
}