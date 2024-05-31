#![allow(clippy::needless_pass_by_value)]

use std::time::{Duration, Instant};

use nannou::{
    app, color, event::{Key, Update, WindowEvent}, rand::random_range, App, Draw, Frame
};

fn main() {
    app(model).update(update).run();
}

struct Model {
    switch_time: Instant,
    tabbed: Option<Instant>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(256, 256)
        .resizable(false)
        .title("tab")
        .view(view)
        .event(event)
        .build()
        .unwrap();
    Model {
        switch_time: Instant::now() + Duration::from_secs_f32(random_range(4., 12.)),
        tabbed: None,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    if matches!(event, WindowEvent::KeyPressed(Key::R)) {
        model.switch_time = Instant::now() + Duration::from_secs_f32(random_range(4., 12.));
        model.tabbed = None;
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if [Key::Q, Key::C]
        .iter()
        .all(|key| app.keys.down.contains(key))
        && model.tabbed.is_none()
    {
        model.tabbed = Some(Instant::now());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(
        if Instant::now() > model.switch_time && model.tabbed.is_none() {
            color::RED
        } else {
            color::BLUE
        },
    );
    let draw = Draw::new();
    if let Some(tabbed) = model.tabbed {
        if tabbed > model.switch_time {
            let delta = tabbed - model.switch_time;
            draw.text(&format!("tabbed in {}ms", delta.as_millis()))
                .color(color::WHITE)
                .font_size(32)
                .x_y(0., 0.);
        } else {
            draw.text(&format!(
                "failed, -{}ms",
                (model.switch_time - tabbed).as_millis()
            ))
            .color(color::WHITE)
            .font_size(32)
            .x_y(0., 0.);
        }
    } else {
        draw.text("press Q and C when the screen turns red")
            .color(color::WHITE)
            .font_size(32)
            .x_y(0., 0.);
    }

    draw.to_frame(app, &frame).unwrap();
}
