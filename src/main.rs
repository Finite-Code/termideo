use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::{stdout, Write};
use std::{thread, time::Duration};

fn main() -> std::io::Result<()>{
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen, Hide)?;

    let density = " .:-=*#%@";

    for frame_count in 0..100{
        execute!(stdout, MoveTo(0, 0))?;

        let mut frame_buffer = String::new();
        for y in 0..20{
            for x in 0..60{
                let char_index = (x+y+frame_count) % density.len();
                let ascii_char = density.chars().nth(char_index).unwrap();
                frame_buffer.push(ascii_char);
            } frame_buffer.push('\n');
        }

        print!("{}", frame_buffer);
        stdout.flush()?;

        thread::sleep(Duration::from_millis(33));
    }

    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}