use std::{env, io::{self, stdout, Stdout, Write}};
use crossterm::{cursor::{self, position}, style::Print, terminal, QueueableCommand};

use crate::Position;

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

    pub fn draw_rows(&mut self, rows: &[String] ) -> io::Result<()>{
        
        const VERSION:&str = env!("CARGO_PKG_VERSION");

        for row in 0..self.height {
            if row >= rows.len() as u16{
                if row == self.height / 3 && rows.len() ==0 {
                    let mut welcome = format!("Text editor --version {VERSION}");
                    welcome.truncate(self.width as usize);

                    if welcome.len() < self.width.into() {
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
                let len = rows[row as usize].len().min(self.width as usize);

                self.stdout.queue(cursor::MoveTo(1,row))?
                .queue(Print(rows[row as usize][0..len].to_string()))?;

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

    pub fn move_to(&mut self, pos: &Position) -> io::Result<()> {
        self.stdout.queue(cursor::MoveTo(pos.x, pos.y))?;

        Ok(())
    }

    pub fn bounds(&self) -> Position {
        Position{
            x: self.width,
            y: self.height
        }
    }

    pub fn move_to_beginning(&mut self, pos: &Position) -> io::Result<()> {
        self.stdout.queue(cursor::MoveTo(pos.x, pos.y))?;

        Ok(())
    }

    pub fn move_to_end(&mut self, pos: &Position) -> io::Result<()> {
        self.stdout.queue(cursor::MoveTo(pos.x, pos.y))?;

        Ok(())
    }
}