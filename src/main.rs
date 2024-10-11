use fltk::{prelude::*, *, enums::*};
use livesplit_core::rendering::software::Renderer;
use livesplit_core::{Run, Segment, Timer, settings::ImageCache, layout::Layout};
use livesplit_core::layout::editor::Editor;
use livesplit_hotkey::{Hook, Hotkey, KeyCode, Modifiers};

fn main() {
    // Create a run object that we can use with at least one segment.
    let mut run = Run::new();
    run.set_game_name("Super Mario Odyssey");
    run.set_category_name("Any%");
    run.push_segment(Segment::new("Cap Kingdom"));
    run.push_segment(Segment::new("Kingdom"));
    run.push_segment(Segment::new("Cap"));
    run.push_segment(Segment::new("Capuing"));
    run.push_segment(Segment::new("💁👌🎍"));
    run.push_segment(Segment::new("本語組版処理の ..."));

    // Create the timer from the run.
    let timer = Timer::new(run).expect("Run with at least one segment provided");
    let shared_timer = timer.into_shared();

    shared_timer.write().unwrap().start();
    let layout = Layout::default_layout();
    let mut editor = Editor::new(layout).expect("Editor RIP");

    let st = shared_timer.clone();


    let hook = Hook::new().expect("Livesplit Hotkeys RIP");
    let _ = hook.register(Hotkey { key_code: KeyCode::F12, modifiers: Modifiers::empty() }, move || {
        st.write().unwrap().split();
    });
    let _ = hook.register(Hotkey { key_code: KeyCode::F11, modifiers: Modifiers::empty() }, move || {
        draw::push_clip(100, 100, 500, 200);
        draw::draw_box(
            enums::FrameType::ThinUpBox,
            100,
            100,
            500,
            200,
            enums::Color::FrameDefault,
        );
        draw::pop_clip();
    });

    let app = app::App::default();

    let mut wind = window::Window::default().with_size(400, 600);
    wind.make_resizable(true);
    wind.set_color(enums::Color::Black);
    wind.set_border(false);

    wind.handle({
        let mut x = 0;
        let mut y = 0;
        let mut can_resize: bool = false;
        let mut is_on_right_bottom_corner: bool = false;
        let st = shared_timer.clone();

        move |w, ev| match ev {
            enums::Event::Push => {
                match app::event_mouse_button() {
                    app::MouseButton::Right => {
                        let coords = app::event_coords();
                        let menu = menu::MenuButton::default();
                        menu.clone().with_pos(coords.0, coords.1);
                        menu.clone().add("Edit Splits...", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Open Splits/From File...", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("Open Splits/From URL...", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("_Open Splits/From Speedrun.com...", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("_Open Splits/Edit History...", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("Save Splits", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Save Splits As...", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("_Close Splits", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Control/Start", Shortcut::None, menu::MenuFlag::Normal, {
                        let st = st.clone();
                        move |_|{ 
                            st.write().unwrap().start();
                        }});
                        menu.clone().add("Control/Reset", Shortcut::None, menu::MenuFlag::Normal, {
                        let st = st.clone();
                        move |_|{ 
                            st.write().unwrap().reset(true);
                        }});
                        menu.clone().add("Control/Undo Split", Shortcut::None, menu::MenuFlag::Normal, {
                        let st = st.clone();
                        move |_|{ 
                            st.write().unwrap().undo_split();
                        }});
                        menu.clone().add("Control/Skip Split", Shortcut::None, menu::MenuFlag::Normal, {
                        let st = st.clone();
                        move |_|{ 
                            st.write().unwrap().skip_split();
                        }});
                        menu.clone().add("Control/Pause", Shortcut::None, menu::MenuFlag::Normal, {
                        let st = st.clone();
                        move |_|{ 
                            st.write().unwrap().pause();
                        }});
                        menu.clone().add("Control/Undo All Pauses", Shortcut::None, menu::MenuFlag::Normal, {
                        let st = st.clone();
                        move |_|{ 
                            st.write().unwrap().undo_all_pauses();
                        }});
                        menu.clone().add("Control/Global Hotkeys", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Compare Against/_Personal Best", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Compare Against/Best Segments", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Compare Against/_Average Segments", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Compare Against/Real Time", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("_Compare Against/Game Time", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Share...", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("racetime.gg Races/New Race...", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("_SRL Races/New Race...", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Edit Layout...", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Open Layout/From File...", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("Open Layout/From URL...", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("Open Layout/_Default", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("Open Layout/Edit History", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("Save Layout", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("_Save Layout As...", Shortcut::None, menu::MenuFlag::Normal, |_|{}); 
                        menu.clone().add("_Settings", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("About", Shortcut::None, menu::MenuFlag::Normal, |_|{});
                        menu.clone().add("Exit", Shortcut::None, menu::MenuFlag::Normal, |_|{});

                        menu.popup();
                    },
                    app::MouseButton::Left => {
                        let coords = app::event_coords();
                        x = coords.0;
                        y = coords.1;
                        can_resize = is_on_right_bottom_corner;
                    },
                    _ => ()
                }
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

    let mut frame = frame::Frame::default_fill();

    wind.end();
    wind.show();
    wind.set_on_top();


    let mut renderer = Renderer::default();
    let mut image_cache = ImageCache::new();

    app::add_idle3(move |_| {
        let layout_state = editor.layout_state(&mut image_cache, &shared_timer.clone().read().unwrap().snapshot());
        renderer.render(&layout_state, &image_cache, [wind.w().try_into().unwrap(), wind.h().try_into().unwrap()]);
        let fb = renderer.image_data();
        draw::draw_rgba(&mut frame, fb).unwrap(); 
        frame.redraw();
        app::sleep(0.016);
    });

    app.run().unwrap();
}

