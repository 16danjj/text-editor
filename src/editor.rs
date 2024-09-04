use crossterm::event::{KeyEvent, KeyModifiers, KeyCode, KeyEventKind, KeyEventState};
use std::io::{self};
use crossterm::terminal;
use errno::errno;
use std::collections::HashMap;
use std::path::Path;
use crate::keyboard::*;
use crate::screen::*;

#[derive(Clone, Copy)]
enum EditorKey{
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

#[derive(Default)]
pub struct Position {
    pub x : u16,
    pub y : u16
}

pub struct Editor {
    screen : Screen,
    keyboard : Keyboard,
    cursor : Position,
    keymap : HashMap<char, EditorKey>,
    rows : Vec<String>
}

impl Editor {

    pub fn new<P: AsRef<Path>>(filename: P) -> io::Result<Self>{

        let first_line = std::fs::read_to_string(filename)
        .expect("Unable to open file")
        .split("/n")
        .next()
        .unwrap().to_string();
    
        let mut key_map = HashMap::new();
        key_map.insert('w', EditorKey::ArrowUp);
        key_map.insert('a', EditorKey::ArrowLeft);
        key_map.insert('s', EditorKey::ArrowDown);
        key_map.insert('d', EditorKey::ArrowRight);
        Ok(Self {
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : Position::default(),
            keymap : key_map,
            rows : vec![first_line]
        })
    }

    pub fn process_keypress(&mut self) -> io::Result<bool> {
        
        if let Ok(c) = self.keyboard.read() {
            match c {
                KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE } => return Ok(true),
                KeyEvent {code : KeyCode::Up, ..} => {self.move_cursor(EditorKey::ArrowUp);},
                KeyEvent {code : KeyCode::Down, ..} => {self.move_cursor(EditorKey::ArrowDown);},
                KeyEvent {code : KeyCode::Left, ..} => {self.move_cursor(EditorKey::ArrowLeft);},
                KeyEvent {code : KeyCode::Right, ..} => {self.move_cursor(EditorKey::ArrowRight);},
                KeyEvent {code : KeyCode::PageUp, ..} | KeyEvent {code : KeyCode::PageDown, ..} => 
                {
                    let bounds = self.screen.bounds();
                    for _ in 0..bounds.y{
                        if c.code == KeyCode::PageUp{
                            self.move_cursor(EditorKey::ArrowUp);
                        }
                        else {
                            self.move_cursor(EditorKey::ArrowDown);
                        }
                    }
                },
                KeyEvent {code : KeyCode::Home, ..} => {
                    self.cursor.x  = 0;
                    self.screen.move_to_beginning(&self.cursor)?;
                },
                KeyEvent {code : KeyCode::End, ..} => {
                    let bounds = self.screen.bounds();
                    self.cursor.x = bounds.x;
                    self.screen.move_to_end(&self.cursor)?;
                },
                KeyEvent {code : KeyCode::Char(key), ..} => {
                    match key {
                        'w' | 'a' | 's' | 'd' => {self.move_cursor(self.keymap.get(&key).copied().unwrap());},
                        _ => {}
                    }
                },  
                

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
        self.screen.draw_rows(&self.rows)
    }

    pub fn die<S : Into<String>>(&mut self, message : S){
        let _ = self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}:{}", message.into(), errno());
        std::process::exit(1);
    }

    fn move_cursor(&mut self, key:EditorKey) {

        let bounds = self.screen.bounds();

        match key {
            EditorKey::ArrowLeft => {self.cursor.x = self.cursor.x.saturating_sub(1);},
            EditorKey::ArrowRight if self.cursor.x < bounds.x => {self.cursor.x +=1;},
            EditorKey::ArrowUp => {self.cursor.y = self.cursor.y.saturating_sub(1);},
            EditorKey::ArrowDown if self.cursor.y < bounds.y => {self.cursor.y +=1;},
            _ => {}
        }
    }
}

