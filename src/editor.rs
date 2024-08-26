use crossterm::event::{read, KeyEvent, KeyModifiers, KeyCode, KeyEventKind, KeyEventState, Event::*};
use std::io::{self, Stdout, Write};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};
use errno::errno;


pub struct Editor {
    width : u16,
    height : u16
}

impl Editor {
    pub fn new() -> io::Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        
        Ok(Self{
            width : columns,
            height : rows
        })
    }


    pub fn process_keypress(&self) -> bool {
        
        if let Ok(c) = self.read_key() {
            match c {
                KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE } => return true,
                _ => {return false}
            }
        }

        false
    }

    pub fn draw_rows(&self, stdout : &mut Stdout) -> io::Result<()>{
        
        for row in 0..self.height {
            stdout
            .queue(cursor::MoveTo(0,row))?
            .queue(Print("~".to_string()))?;
        }

        Ok(())
    }

    pub fn clear_screen(&self, stdout : &mut Stdout)  -> io::Result<()>{

        stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0,0))?
        .flush()
    
    }

    pub fn refresh_screen(&self) -> io::Result<()>{
        let mut stdout = io::stdout();

        self.clear_screen(&mut stdout)?;
        self.draw_rows(&mut stdout)?;
        
        stdout.queue(cursor::MoveTo(0,0))?.flush()

    }

    pub fn die<S : Into<String>>(&self, message : S){
        let mut stdout = io::stdout();
        let _ = self.clear_screen(&mut stdout);
        let _ = terminal::disable_raw_mode();
        eprintln!("{}:{}", message.into(), errno());
        std::process::exit(1);
    }

    pub fn read_key(&self)->Result<KeyEvent, String>{
        loop{
            if let Ok(event) = read(){
                if let Key(key_event) = event {
                   return Ok(key_event)
                } 
            } else{
                self.die("read failed"); 
                return Err(String::from("Read failed"))
            }
        }
    }

}