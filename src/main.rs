use std::{
    time::Duration, io::{
        self,
        Write,
    },
    // env,
    fs,
    path::PathBuf,
};
use crossterm::{
    ExecutableCommand, QueueableCommand, 
    event::{
        self, Event, KeyCode,
    }, 
    terminal::{
        enable_raw_mode, disable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
    },
    cursor::{
        MoveTo, Hide, Show,
    },
};
use clap::Parser;

#[allow(unused)]
static FPS: f32 = 30.0;
#[allow(unused)]
static TEXT: &str = "DVD";
#[allow(unused)]
static LEN: u16 = TEXT.len() as u16;

#[allow(unused)]
fn main_characters() -> io::Result<()> {
    let mut stdout = io::stdout();
    let duration = Duration::from_secs_f32(1f32 / FPS);
    let mut x = 0;
    let mut y = 0;
    let mut vx = 1i16;
    let mut vy = 1i16;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    let (mut w, mut h) = size()?;
    enable_raw_mode()?;
    loop {
        if event::poll(duration)? {
            match event::read()? {
                Event::Key(e) => {
                    if let KeyCode::Char(c) = e.code && c == 'q' {
                        break;
                    }
                },
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                },
                _ => {},
            }
        }
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(x, y))?;
        write!(stdout, "{}", TEXT)?;
        stdout.flush()?;
        x = (x as i16 + vx) as u16;
        y = (y as i16 + vy) as u16;
        if x + LEN == w || x == 0 {
            vx = -vx;
        }
        if y + 1 == h || y == 0 {
            vy = -vy;
        }
    }
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom file to show
    #[arg(short = 'f', long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Sets the FPS
    #[arg(short = 'F', long, value_name = "FPS", default_value_t = 10.0)]
    fps: f32,
}

static DEFAULT_SIGN: &str = "#####     #           #   #####\n#     #    #         #    #     #\n#      #    #       #     #      #\n#      #     #     #      #      #\n#      #      #   #       #      #\n#     #        # #        #     #\n#####           #         #####";

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    // let mut args = env::args();
    // args.next().expect("unreachable");
    // let sign = match args.next() {
    //     Some(file_name) => {
    //         fs::read_to_string(file_name)?
    //     },
    //     None => DEFAULT_SIGN.to_string(),
    // };
    let sign = match cli.file {
        Some(file_name) => fs::read_to_string(file_name)?,
        None => DEFAULT_SIGN.to_string(),
    };
    let lines: Vec<_> = sign.lines().collect();
    let tw = lines.iter().map(|line| line.len()).max().unwrap() as u16;
    let th = lines.len() as u16;
    let mut stdout = io::stdout();
    // let duration = Duration::from_secs_f32(1f32 / FPS);
    let duration = Duration::from_secs_f32(1f32 / cli.fps);
    let mut x = 0;
    let mut y = 0;
    let mut vx = 1i16;
    let mut vy = 1i16;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    let (mut w, mut h) = size()?;
    enable_raw_mode()?;
    loop {
        if event::poll(duration)? {
            match event::read()? {
                Event::Key(e) => {
                    if let KeyCode::Char(c) = e.code && c == 'q' {
                        break;
                    }
                },
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                },
                _ => {},
            }
        }
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(x, y))?;
        let mut ty = y;
        for line in &lines {
            write!(stdout, "{}", line)?;
            ty += 1;
            stdout.queue(MoveTo(x, ty))?;
        }
        stdout.flush()?;
        x = (x as i16 + vx) as u16;
        y = (y as i16 + vy) as u16;
        if x + tw == w || x == 0 {
            vx = -vx;
        }
        if y + th == h || y == 0 {
            vy = -vy;
        }
    }
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
