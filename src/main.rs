use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::{
    io::{Result, stdout},
    thread,
    time::Duration,
};

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_direction(current: Direction) -> Direction {
    event::poll(Duration::from_millis(0))
        .ok()
        .filter(|&ready| ready)
        .and_then(|_| event::read().ok())
        .and_then(|ev| match ev {
            Event::Key(key) => Some(key.code),
            _ => None,
        })
        .map(|code| match code {
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            _ => current,
        })
        .unwrap_or(current)
}

fn draw_point(p: Point, symbol: &str) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(p.x, p.y), Print(symbol))?;
    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    let mut snake = vec![Point { x: 10, y: 10 }, Point { x: 9, y: 10 }];

    let mut dir = Direction::Right;
    let mut food = Point { x: 15, y: 10 };
    let tick = Duration::from_millis(120);

    loop {
        dir = read_direction(dir);

        let new_head = next_head(snake[0], &dir);

        // Wall collision
        if new_head.x == 0 || new_head.y == 0 {
            break;
        }

        snake.insert(0, new_head);

        if new_head == food {
            food = Point {
                x: rand::rng().random_range(1..50),
                y: rand::rng().random_range(1..20),
            };
        } else {
            snake.pop();
        }

        execute!(stdout, terminal::Clear(ClearType::All))?;

        for segment in &snake {
            draw_point(*segment, "█")?;
        }

        draw_point(food, "●")?;

        thread::sleep(tick);
    }
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    Ok(())
}

fn next_head(head: Point, dir: &Direction) -> Point {
    match dir {
        Direction::Up => Point {
            x: head.x,
            y: head.y - 1,
        },
        Direction::Down => Point {
            x: head.x,
            y: head.y + 1,
        },
        Direction::Left => Point {
            x: head.x - 1,
            y: head.y,
        },
        Direction::Right => Point {
            x: head.x + 1,
            y: head.y,
        },
    }
}

