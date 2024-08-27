use std::io::{self, stdout, Stdout, Write};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};


pub struct Screen {
    stdout : Stdout,
    width : u16,
    height : u16
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

    pub fn draw_rows(&mut self) -> io::Result<()>{
        
        for row in 0..self.height {
            self.stdout
            .queue(cursor::MoveTo(0,row))?
            .queue(Print("~".to_string()))?;
        }

        self.stdout.flush()
    }

    pub fn clear(&mut self)  -> io::Result<()>{

        self.stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0,0))?
        .flush()
    
    } 

    pub fn move_cursor_flush(&mut self) -> io::Result<()>{
        self.stdout.queue(cursor::MoveTo(0,0))?.flush()
    }
}