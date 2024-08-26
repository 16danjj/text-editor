use crossterm::event::{read, KeyEvent, Event::*};

use crate::*;


pub fn editor_read_key()->Result<KeyEvent, String>{
    loop{
        if let Ok(event) = read(){
            if let Key(key_event) = event {
               return Ok(key_event)
            } 
        } else{
            die("read failed"); 
            return Err(String::from("Read failed"))
        }
    }
}