use winit::error::EventLoopError;
use xilem::masonry::parley::swash::text;
use xilem::view::{Axis, flex, label, text_button};
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

#[derive(Default)]
struct Counter {
    num: i32,
}

fn app_logic(data: &mut Counter) -> impl WidgetView<Counter> + use<> {
    flex(
        Axis::Vertical,
        (
            label(format!("{}", data.num)),
            flex(
                Axis::Horizontal,
                (
                    text_button("+", |data: &mut Counter| data.num += 1),
                    text_button("-", |data: &mut Counter| data.num -= 1),
                ),
            ),
        ),
    )
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        Counter::default(),
        app_logic,
        WindowOptions::new("Counter app"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
