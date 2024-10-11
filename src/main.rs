use fltk::{prelude::*, *};
use livesplit_core::rendering::software::Renderer;
use livesplit_core::{Run, Segment, Timer, layout::Layout};
use livesplit_core::layout::editor::Editor;

fn main() {
    // Create a run object that we can use with at least one segment.
    let mut run = Run::new();
    run.set_game_name("Super Mario Odyssey");
    run.set_category_name("Any%");
    run.push_segment(Segment::new("Cap Kingdom"));

    // Create the timer from the run.
    let mut timer = Timer::new(run).expect("Run with at least one segment provided");
    timer.start();
    let layout = Layout::default_layout();
    let mut editor = Editor::new(layout).expect("EDITOR");


    let app = app::App::default();
    let mut wind = window::DoubleWindow::default().with_size(400, 600);
    wind.make_resizable(true);

    wind.set_color(enums::Color::Black);
    wind.set_border(false);
    wind.handle({
        let mut x = 0;
        let mut y = 0;
        let mut can_resize: bool = false;
        let mut is_on_right_bottom_corner: bool = false;

        move |w, ev| match ev {
            enums::Event::Push => {
                let coords = app::event_coords();
                x = coords.0;
                y = coords.1;
                can_resize = is_on_right_bottom_corner;
                true
            }
            enums::Event::Released => {
                can_resize = false;
                true
            }
            enums::Event::Move => {
                let event_x = app::event_x();
                let event_y = app::event_y();
                let x = w.width() - 10;
                let y = w.height() - 10;
                let dist_right_border = x - event_x;
                let dist_bottom_border = y - event_y;
                is_on_right_bottom_corner = dist_right_border < 10 && dist_bottom_border < 10;
                match is_on_right_bottom_corner {
                    true => w.set_cursor(enums::Cursor::SE),
                    false => w.set_cursor(enums::Cursor::Default)
                }
                true
            }
            enums::Event::Leave => {
                w.set_cursor(enums::Cursor::Default);
                true
            }
            enums::Event::Drag => {
                if can_resize {
                    let x = app::event_x();
                    let y = app::event_y();
                    let width = match x > (10) + 1 {
                        true => x - (10),
                        false => 10
                    };
                    let height = match y > (10) + 1 {
                        true => y - (10),
                        false => 10
                    };
                    w.set_size(width, height);
                } else {
                    w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                }
                true
            }
            _ => false,
        }
    });


    wind.draw(|w| {
    // draw::draw_box(enums::FrameType::FlatBox, 0, 0, 100, 100, enums::Color::White);
        let x = w.width() - 10;
        let y = w.height() - 10;
        let w = 10;
        let h = 10;
        draw::push_clip(x, y, w, h);
        // draw::draw_rectf(x, y, w, h);
        draw::set_draw_color(enums::Color::White);
        // draw::draw_rect(x, y, w, h);
        draw::pop_clip();

    });
    let mut frame = frame::Frame::default_fill();


    wind.end();
    wind.show();
    wind.set_on_top();

    let mut renderer = Renderer::new();

    app::add_idle3(move |_| {
        let layout_state = editor.layout_state(&timer.snapshot());
        renderer.render(&layout_state, [wind.w().try_into().unwrap(), wind.h().try_into().unwrap()]);
        let fb = renderer.image_data();
        draw::draw_rgba(&mut frame, fb).unwrap(); 
        wind.redraw();
        app::sleep(0.016);
    });

    app.run().unwrap();
}

