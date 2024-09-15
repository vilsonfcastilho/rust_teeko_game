use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

mod view;
use view::board_view::Renderer;

mod model;
use model::game::GameState;

fn main() -> Result<(), String> {
    let window_width: u32 = 800;
    let window_height: u32 = 600;

    let sdl_context: Sdl = sdl2::init()?;

    let video_subsystem: VideoSubsystem = sdl_context.video()?;
    let window: Window = video_subsystem
        .window("Rust!", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let texture_loader: TextureCreator<WindowContext> = canvas.texture_creator();

    let board_view: Renderer = Renderer::new(window_width, window_height, &texture_loader);

    let mut game_state: GameState = GameState::new();

    let mut running: bool = true;
    let mut event_queue: EventPump = sdl_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let row: usize = (5 * y / board_view.screen_area.h).try_into().unwrap();
                    let col: usize = (5 * x / board_view.screen_area.w).try_into().unwrap();
                    game_state.handle_click(row, col);
                }

                Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap() == sdl2::keyboard::Keycode::U {
                        game_state.undo_action();
                    }

                    if keycode.unwrap() == sdl2::keyboard::Keycode::R {
                        game_state.redo_action();
                    }
                }

                _ => {}
            }
        }

        board_view.render(&mut canvas, &game_state.board);
        canvas.present();
    }

    return Ok(());
}
