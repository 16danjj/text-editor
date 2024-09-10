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
    rows : Vec<String>,
    rowoff : u16,
    coloff : u16
}

impl Editor {

    pub fn with_file<P: AsRef<Path>>(filename: P) -> io::Result<Self>{

        let lines = std::fs::read_to_string(filename)
        .expect("Unable to open file")
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        Editor::build(&lines)
    }

    pub fn new() -> io::Result<Self> {
        Editor::build(&[])
    }

    fn build(data: &[String]) -> io::Result<Self>
    {
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
            rows : if data.is_empty() {Vec::new()} else {Vec::from(data)}  ,
            rowoff : 0, 
            coloff : 0
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

            self.screen.move_to(&self.cursor, self.rowoff, self.coloff)?;
            
            self.screen.flush_op()?;
            
            if self.process_keypress()?{
                self.screen.clear()?;
                break;
            }  
        }
    
        terminal::disable_raw_mode()?; 

        Ok(())
    }

    pub fn refresh_screen(&mut self) -> io::Result<()>{
        self.scroll();
        self.screen.clear()?;  
        self.screen.draw_rows(&self.rows, self.rowoff, self.coloff)
    }

    pub fn die<S : Into<String>>(&mut self, message : S){
        let _ = self.screen.clear();
        let _ = terminal::disable_raw_mode();
        eprintln!("{}:{}", message.into(), errno());
        std::process::exit(1);
    }

    fn move_cursor(&mut self, key:EditorKey) {

        let row_idx = if self.cursor.y as usize >= self.rows.len(){
            None
        }
        else {
            Some(self.cursor.y as usize)
        };

        match key {
            EditorKey::ArrowLeft => {self.cursor.x = self.cursor.x.saturating_sub(1);},
            EditorKey::ArrowRight => {
                if let Some(idx) = row_idx {
                    if self.cursor.x < (self.rows[idx].len() as u16){
                        self.cursor.x += 1;
                    }
                }
            },
            EditorKey::ArrowUp => {self.cursor.y = self.cursor.y.saturating_sub(1);},
            EditorKey::ArrowDown if self.cursor.y < self.rows.len() as u16 => self.cursor.y += 1,
            _ => {}
        }

        let row_idx = if self.cursor.y as usize >= self.rows.len(){
            None
        }
        else {
            Some(self.cursor.y as usize)
        };

        let row_len = if let Some(idx) = row_idx{
            self.rows[idx].len()
        }
        else {0} as u16;

        if self.cursor.x > row_len {
            self.cursor.x = row_len;
        }
    }

    fn scroll(&mut self){
        let bounds = self.screen.bounds();

        if self.cursor.y < self.rowoff {
            self.rowoff = self.cursor.y;
        }

        if self.cursor.y >= self.rowoff + bounds.y{
            self.rowoff = self.cursor.y - bounds.y + 1;
        }

        if self.cursor.x < self.coloff {
            self.coloff = self.cursor.x;
        }

        if self.cursor.x >= self.coloff + bounds.x {
            self.coloff = self.cursor.x - bounds.x + 1;
        }
    }


}

