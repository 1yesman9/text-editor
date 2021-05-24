//mod
mod structures;

//internal
use structures::{
    gap::GapBuffer
};

//std
use std::{
    io::{stdout, Write},
    time::Duration
};

//third party
use crossterm::{
    cursor::{MoveTo, MoveRight, MoveLeft, position},
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};

//APP Structure
struct App {
    gap_buffers: Vec<GapBuffer>,
    cursor: Cursor,
    mode: Mode
}

struct Cursor(usize, usize);



enum Mode { Insert, Control }
enum EventResult {
    Ok,
    Break
}

impl<'a> App {
    fn new() -> App {
        App {
            gap_buffers: vec![GapBuffer::new(10)],
            cursor: Cursor(0,0),
            mode: Mode::Control
        }
    }

    fn move_right(&mut self) {
        self.cursor.0 += 1;
    }
    
    fn move_left(&mut self) {
        if self.cursor.0 == 0 { return }
        self.cursor.0 -= 1;
    }
    
    fn move_down(&mut self) {
        if self.cursor.1 == self.gap_buffers.len() - 1 { return }
        self.cursor.1 += 1;
    }
    
    fn move_up(&mut self) {
        if self.cursor.1 == 0 { return }
        self.cursor.1 -= 1;
    }

    //TODO: refactor ( move event in another module )
    fn event_poll(&mut self) -> Result<EventResult> {
        let event = read()?;

        //control mode
        if event == Event::Key(KeyCode::Esc.into()) {
            self.mode = Mode::Control;
            return Ok(EventResult::Ok);        
        }

        match self.mode {
            Mode::Insert => {

                //TODO: refactor ( less repetition )
                //new line
                if event == Event::Key(KeyCode::Enter.into()) {
                    self.cursor.1 += 1;
                    self.cursor.0 = 0;
                    self.gap_buffers.insert(self.cursor.1, GapBuffer::new(10));
                    return Ok(EventResult::Ok);

                } else if event == Event::Key(KeyCode::Right.into()) {
                    self.move_right();
                    return Ok(EventResult::Ok);

                } else if event == Event::Key(KeyCode::Left.into()) {
                    self.move_left();
                    return Ok(EventResult::Ok);

                } else if event == Event::Key(KeyCode::Up.into()) {
                    self.move_up();
                    return Ok(EventResult::Ok);

                } else if event == Event::Key(KeyCode::Down.into()) {
                    self.move_down();
                    return Ok(EventResult::Ok);

                }

                //edits
                let gap_buffer = &mut self.gap_buffers[self.cursor.1];

                //deletion
                if event == Event::Key(KeyCode::Backspace.into()) {
                    if self.cursor.0 == 0 { 
                        if self.cursor.1 == 0 {
                            
                        } else {
                            //TODO: append the remaining gap buffer text to the previous line
                            self.gap_buffers.remove(self.cursor.1);
                            self.move_up();
                        }

                        return Ok(EventResult::Ok) 

                    }
                    gap_buffer.deletion(self.cursor.0-1);
                    self.cursor.0 -= 1;
                }    
            
                //insert char
                if let Event::Key(key_event) = event {
                    if let KeyCode::Char(c) = key_event.code {
                        gap_buffer.insert(&[c as u8], self.cursor.0);
                        self.cursor.0 += 1;
                    }
                }
            },

            Mode::Control => {
                if let Event::Key(key_event) = event {
                    if let KeyCode::Char(c) = key_event.code {
                        //c: char
                        if c == 'q' {
                            return Ok(EventResult::Break);
                        } else if c == 'i' {
                            self.mode = Mode::Insert;
                        } else if c == 'h' { // move left
                            self.move_left();
                        } else if c == 'j' { // move up
                            self.move_up();
                        } else if c == 'k' { // move down
                            self.move_down();
                        } else if c == 'l' { // move right
                            self.move_right();
                        }
                    }
                }
            }
        }

        Ok(EventResult::Ok)
    }

    fn render(&mut self) -> Result<()> {
        let mut stdout = stdout();

        //clear from cursor down
        execute!(
            stdout,
            MoveTo(0,self.cursor.1 as u16),
            Clear(ClearType::FromCursorDown),
        );

        for gap_buffer in self.gap_buffers.iter().skip(self.cursor.1) {
            stdout.write(b"\r").unwrap(); // carriage return to overwrite line
            stdout.write(gap_buffer.get_left()).unwrap();
            stdout.write(gap_buffer.get_right()).unwrap();
            stdout.flush().unwrap();
            stdout.write(b"\n").unwrap(); // new line
        }

        //render cursor
        execute!(
            stdout,
            MoveTo(self.cursor.0 as u16, self.cursor.1 as u16)
        );

        Ok(())
    }

    fn execute(mut self) -> Result<()> {
        loop {
            if poll(Duration::from_millis(1_000))? {
                if let EventResult::Break = self.event_poll()? { break }
                self.render()?;
            }
        }

        Ok(())
    }
}

fn main() {
    //enable raw mode
    enable_raw_mode().unwrap();

    //get reference to stdout
    let mut stdout = stdout();

    //enable mouse capture
    execute!(stdout, EnableMouseCapture).unwrap();

    //run app
    let mut app = App::new();
    println!("{:?}", app.execute());

    //back to normal
    execute!(stdout, DisableMouseCapture).unwrap();
    disable_raw_mode();
}