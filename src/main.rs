use winit::error::EventLoopError;
use xilem::view::{flex, label, Axis};
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use strip_ansi::strip_ansi;

struct Pinpontty {
    rx: Receiver<String>,
    last_output: String,
}

fn app_logic(data: &mut Pinpontty) -> impl WidgetView<Pinpontty> + use<> {
    while let Ok(output) = data.rx.try_recv() {
        data.last_output = output;
    }
    let display_text = if data.last_output.is_empty() {
        "Waiting for output...".to_string()
    } else {
        data.last_output.clone()
    };
    flex(Axis::Vertical, (label(display_text),))
}

fn main() -> Result<(), EventLoopError> {
    let (tx, rx) = channel();

    ::std::thread::spawn(move || {
        use std::io::Read;
        use std::process::Command;

        let mut child = Command::new("nu")
            .arg("-c")
            .arg("ls")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to execute nu");

        let mut stdout = child.stdout.take().expect("Failed to open stdout");
        let mut output = String::new();
        stdout.read_to_string(&mut output).ok();

        let stripped = strip_ansi(&output);
        if !stripped.trim().is_empty() {
            println!("Output from nu ls:\n{}", stripped);
            tx.send(stripped).ok();
        }

        child.wait().ok();
    });

    let app = Xilem::new_simple(
        Pinpontty {
            rx: rx,
            last_output: String::new(),
        },
        app_logic,
        WindowOptions::new("Pinpontty"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
