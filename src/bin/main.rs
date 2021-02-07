use easy_shell_os::{event, event::Events};
use std::{
    error::Error,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, sleep},
    time::{Duration, SystemTime},
};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tokio::{sync::mpsc, task};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders,
    },
    Terminal,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ball = Rectangle {
        x: 10.0,
        y: 30.0,
        width: 10.0,
        height: 10.0,
        color: Color::Yellow,
    };

    let mut events = Events::new(0, "hello".to_string(), 1 << 3);

    let sender = events.sender();
    task::spawn(async move {
        while let Some(Ok(key)) = io::stdin().keys().next() {
            sender.send(event::Event::Key(key)).await;
        }
    });

    let (tx, mut rx) = mpsc::channel(1);
    let frame_sender = events.sender();
    task::spawn(async move {
        while let Some(ball) = rx.recv().await {
            let start_time = SystemTime::now();
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size());
                let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("Pong"))
                    .paint(|ctx| {
                        ctx.draw(&ball);
                    })
                    .x_bounds([10.0, 110.0])
                    .y_bounds([10.0, 110.0]);
                f.render_widget(canvas, chunks[0]);
            });

            let next = 16u128
                - SystemTime::now()
                    .duration_since(start_time)
                    .unwrap_or(Duration::from_millis(16))
                    .as_millis()
                    .min(16u128);
            if next > 0 {
                sleep(Duration::from_millis(next as u64))
            }
            frame_sender.send(event::Event::Frame).await;
        }
    });

    tx.send(ball.clone()).await;

    while let Some(event) = events.receive().await {
        match event {
            event::Event::Key(key) => match key {
                Key::Esc => {
                    break;
                }
                Key::Up => {
                    ball.y += 1f64;
                }
                Key::Left => {
                    ball.x -= 1f64;
                }
                Key::Right => {
                    ball.x += 1f64;
                }
                Key::Down => {
                    ball.y -= 1f64;
                }
                _ => {}
            },
            event::Event::Frame => {
                tx.send(ball.clone()).await;
            }
        }
    }

    Ok(())
}
