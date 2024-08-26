use std::io::{self, Stdout, Write};
use crossterm::{cursor, style::Print, terminal, QueueableCommand};
use errno::errno;

pub fn editor_draw_rows(stdout : &mut Stdout) -> io::Result<()>{
    let mut stdout = io::stdout();

    for row in 0..24 {
        stdout
        .queue(cursor::MoveTo(0,row))?
        .queue(Print("~".to_string()))?;
    }

    Ok(())

}

pub fn clear_screen(stdout : &mut Stdout)  -> io::Result<()>{

    stdout
    .queue(terminal::Clear(terminal::ClearType::All))?
    .queue(cursor::MoveTo(0,0))?
    .flush()
 
}

pub fn editor_refresh_screen() -> io::Result<()>{
    let mut stdout = io::stdout();

    clear_screen(&mut stdout)?;
    editor_draw_rows(&mut stdout)?;
    
    stdout.queue(cursor::MoveTo(0,0))?.flush()
}

pub fn die<S : Into<String>>(message : S){
    let mut stdout = io::stdout();
    let _ = clear_screen(&mut stdout);
    let _ = terminal::disable_raw_mode();
    eprintln!("{}:{}", message.into(), errno());
    std::process::exit(1);
}