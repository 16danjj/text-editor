use std::{env, io::{self, stdout, Stdout, Write}};
use crossterm::{cursor::{self, position}, style::Print, terminal, QueueableCommand};

use crate::Position;
const TAB_STOP: usize = 8;

pub struct Screen {
    stdout : Stdout,
    width : u16,
    height : u16
}

pub struct Row {
    chars : String,
    render : String,
}

impl Row {
    pub fn new(chars: String) -> Self {
        let mut render = String::new();
        let mut idx = 0;
        for c in chars.chars(){
            match c {
                '\t' => {render.push(' ');
                idx += 1;
                while idx % TAB_STOP != 0 {
                    render.push(' ');
                    idx += 1;
                }}
                _ => {render.push(c);
                     idx += 1;}
            }
        }
        Self {
            chars,
            render 
        }
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn render_len(&self) -> usize {
        self.render.len()
    }

    pub fn cx_to_rx(&self, cx: u16) -> u16{
        let mut rx = 0;
        for c in self.chars.chars().take(cx as usize) {
            if c == '\t' {
                rx += (TAB_STOP - 1) - (rx % TAB_STOP);
            }
            rx += 1;

        }
        rx as u16
    }


}

impl Screen{
    pub fn new() -> io::Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        
        Ok(Self{
            stdout : stdout(),
            width : columns,
            height : rows
        })
    }

    pub fn draw_rows(&mut self, rows: &[Row], rowoff: u16, coloff: u16) -> io::Result<()>{
        
        const VERSION:&str = env!("CARGO_PKG_VERSION");

        for row in 0..self.height {
            let filerow = (row + rowoff) as usize;
            if filerow >= rows.len(){
                if row == self.height / 3 && rows.is_empty() {
                    let mut welcome = format!("Text editor --version {VERSION}");
                    welcome.truncate(self.width as usize);

                    if welcome.len() < self.width as usize {
                        let leftmost = ((self.width as usize - welcome.len()) / 2) as u16;
                        
                        self.stdout.queue(cursor::MoveTo(0,row))?
                                    .queue(Print("~".to_string()))?
                                    .queue(cursor::MoveTo(leftmost, row))?
                                    .queue(Print(welcome))?;
                    }  
                    else {
                        self.stdout.queue(cursor::MoveTo(0,row))?
                                    .queue(Print(welcome))?;
                    }
                    
                }

                else {
                    self.stdout
                    .queue(cursor::MoveTo(0,row))?
                    .queue(Print("~".to_string()))?;
                }

            }

            else {
                
                let mut len = rows[filerow].render_len();

                if len < coloff as usize{ 
                    continue;
                } 

                len -= coloff as usize;

                let start = coloff as usize;
                let end = start + if len >= self.width as usize
                {
                    self.width as usize
                } else {
                    len
                };

                self.stdout.queue(cursor::MoveTo(0,row))?
                .queue(Print(rows[filerow].render[start..end].to_string()))?;

            }

        }

        Ok(())
    }

    pub fn clear(&mut self)  -> io::Result<()>{

        self.stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0,0))?;

        Ok(())
    } 

    pub fn flush_op(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    pub fn cursor_position(&self) -> io::Result<(u16,u16)> {
        cursor::position()
    }

    pub fn move_to(&mut self, pos: &Position, render_x: u16, rowoff: u16, coloff: u16 ) -> io::Result<()> {
        self.stdout.queue(cursor::MoveTo(render_x - coloff, pos.y - rowoff))?;

        Ok(())
    }

    pub fn bounds(&self) -> Position {
        Position{
            x: self.width,
            y: self.height
        }
    }

}