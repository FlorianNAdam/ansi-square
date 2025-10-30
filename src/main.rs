use clap::Parser;
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::{io::stdout, thread::sleep, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Width of the animation grid
    #[arg(short = 'W', long, default_value_t = 16)]
    width: usize,

    /// Height of the animation grid
    #[arg(short = 'H', long, default_value_t = 16)]
    height: usize,

    /// Delay between drawing each cell (ms)
    #[arg(short = 'd', long, default_value_t = 10)]
    draw_delay: u64,

    /// Delay between animation cycles (ms)
    #[arg(short = 'i', long, default_value_t = 200)]
    interval_delay: u64,

    /// Color of the cells (red, green, blue, yellow, magenta, cyan, white, black, or random)
    #[arg(short = 'C', long, default_value = "green")]
    color: String,

    /// Disable the decay (disappearing) animation
    #[arg(long)]
    no_decay: bool,

    /// Number of animation cycles (0 for infinite)
    #[arg(short, long, default_value_t = 1)]
    cycles: usize,

    /// Seed for random number generator (for reproducible patterns)
    #[arg(short, long)]
    seed: Option<u64>,
}

fn parse_color(color_str: &str) -> Option<Color> {
    match color_str.to_lowercase().as_str() {
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        "yellow" => Some(Color::Yellow),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "black" => Some(Color::Black),
        "random" => None, // Special case for random colors
        _ => None,
    }
}

fn get_random_color(rng: &mut StdRng) -> Color {
    let colors = [
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];
    *colors.choose(rng).unwrap()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let fixed_color = parse_color(&args.color);
    let use_random_colors = args.color.to_lowercase() == "random";

    let mut cycles_remaining = args.cycles;

    // Create RNG with optional seed
    let mut rng = if let Some(seed) = args.seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_os_rng()
    };

    loop {
        execute!(stdout, Clear(ClearType::All))?;

        let mut cells = Vec::new();

        for y in 0..args.height {
            for x in 0..args.width {
                cells.push((x, y));
            }
        }

        cells.shuffle(&mut rng);

        // Draw cells
        for (x, y) in cells.clone() {
            // Set color for this cell
            if use_random_colors {
                execute!(stdout, SetForegroundColor(get_random_color(&mut rng)))?;
            } else if let Some(color) = fixed_color {
                execute!(stdout, SetForegroundColor(color))?;
            } else {
                // Default to green if color parsing failed
                execute!(stdout, SetForegroundColor(Color::Green))?;
            }

            execute!(stdout, MoveTo(x as u16 * 2, y as u16), Print("██"))?;
            sleep(Duration::from_millis(args.draw_delay));
        }

        sleep(Duration::from_millis(args.interval_delay));

        // Decay animation (if enabled)
        if !args.no_decay {
            for (x, y) in cells.into_iter().rev() {
                execute!(stdout, MoveTo(x as u16 * 2, y as u16), Print("  "))?;
                sleep(Duration::from_millis(args.draw_delay));
            }
        }

        // Handle cycle counting
        if args.cycles > 0 {
            cycles_remaining -= 1;
            if cycles_remaining == 0 {
                break;
            }
        }
    }

    // Cleanup
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
