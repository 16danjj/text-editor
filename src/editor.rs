use crossterm::event::{KeyEvent, KeyModifiers, KeyCode, KeyEventKind, KeyEventState};
use std::io::{self};
use std::time::{Instant,Duration};
use crossterm::terminal;
use errno::errno;
use std::path::Path;
use crate::keyboard::*;
use crate::screen::*;
use crate::row::*;

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
    filename : String,
    status_msg : String, 
    status_time : Instant,
    screen : Screen,
    keyboard : Keyboard,
    cursor : Position,
    render_x : u16, 
    rows : Vec<Row>,
    rowoff : u16,
    coloff : u16
}

impl Editor {

    pub fn with_file<P: AsRef<Path> + ToString>(filename: P) -> io::Result<Self>{
        let fn_string = filename.to_string();
        let lines = std::fs::read_to_string(filename)
        .expect("Unable to open file")
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        Editor::build(&lines, fn_string)
    }

    pub fn new() -> io::Result<Self> {
        Editor::build(&[], "")
    }

    fn build<T: Into<String>>(data: &[String], filename: T) -> io::Result<Self>
    {
        Ok(Self {
            filename: filename.into(),
            status_msg : String::from("HELP: Ctrl -Q = quit"),
            status_time : Instant::now(),
            screen : Screen::new()?,
            keyboard : Keyboard {},
            cursor : Position::default(),
            render_x : 0,
            rows : if data.is_empty() {
                Vec::new()
            } else {
                let v = Vec::from(data);
                let mut rows = Vec::new();
                for row in v {
                    rows.push(Row::new(row))
                }
                rows
            },
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
                    match c.code {
                        KeyCode::PageUp => {
                            self.cursor.y = self.rowoff
                        }
                        KeyCode::PageDown => {
                            self.cursor.y = (self.rowoff + bounds.y - 1).min(self.rows.len() as u16)
                        }
                        _ => panic!("rust compiler broke"),
                    }
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
                },
                KeyEvent {code : KeyCode::End, ..} => {
                    if self.cursor.y < self.rows.len() as u16{
                        self.cursor.x = self.rows[self.cursor.y as usize].len() as u16;
                    }
                },
                KeyEvent {code : KeyCode::Char(key), ..} => {
                    self.insert_char(key)
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

            self.screen.move_to(&self.cursor, self.render_x, self.rowoff, self.coloff)?;
            
            self.screen.flush_op()?;
            
            if self.process_keypress()?{
                self.screen.clear()?;
                break;
            }  
        }
    
        terminal::disable_raw_mode()

    }

    pub fn refresh_screen(&mut self) -> io::Result<()>{
        self.scroll();
        self.screen.clear()?;  
        self.screen.draw_rows(&self.rows, self.rowoff, self.coloff)?;
        if !self.status_msg.is_empty() && self.status_time.elapsed() > Duration::from_secs(5) {
                self.status_msg.clear();
        }
        
        self.screen.draw_status_bar(format!("{:20} - {} lines", self.filename, self.rows.len()), 
        format!("{}/{}", self.cursor.y, self.rows.len()),
        &self.status_msg)
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
            EditorKey::ArrowLeft => {
                if self.cursor.x != 0 {self.cursor.x -= 1;}
                else if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                    self.cursor.x = self.rows[self.cursor.y as usize].len() as u16;
                }}
            EditorKey::ArrowRight => {
                if let Some(idx) = row_idx {
                    if self.cursor.x < (self.rows[idx].len() as u16){
                        self.cursor.x += 1;
                    }
                    else if self.cursor.x == self.rows[idx].len() as u16 && self.cursor.y < self.rows.len() as u16{
                        self.cursor.y += 1;
                        self.cursor.x = 0;
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
        self.render_x = if self.cursor.y < self.rows.len() as u16 {
            self.rows[self.cursor.y as usize].cx_to_rx(self.cursor.x) 
        }
        else {
            0
        };

        if self.cursor.y < self.rowoff {
            self.rowoff = self.cursor.y;
        }

        if self.cursor.y >= self.rowoff + bounds.y{
            self.rowoff = self.cursor.y - bounds.y + 1;
        }

        if self.render_x < self.coloff {
            self.coloff = self.render_x;
        }

        if self.render_x >= self.coloff + bounds.x {
            self.coloff = self.render_x - bounds.x + 1;
        }
    }

    fn set_status_message<T:Into<String>>(&mut self, message: T){
        self.status_time = Instant::now();
        self.status_msg = message.into();
    }

    fn insert_char(&mut self, c: char){
         if self.cursor.y == self.rows.len() as u16 {
            self.rows.push(Row::new(String::new()));
         }

         self.rows[self.cursor.y as usize].insert_char(self.cursor.x as usize, c);
         self.cursor.x += 1;   
    }

}

