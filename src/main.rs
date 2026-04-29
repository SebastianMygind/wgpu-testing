mod app;
mod buffer;
mod state;

use winit::event_loop::EventLoop;

use crate::app::App;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn main() {
    println!("Running Program!");

    let res = run();

    match res {
        Ok(_) => {}
        Err(_e) => {
            eprintln!("Error running app, closing!")
        }
    }
}
