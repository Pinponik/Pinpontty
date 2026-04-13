use winit::error::EventLoopError;
use xilem::masonry::parley::swash::text;
use xilem::view::{Axis, flex, label, text_button};
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

use anyhow::Error;
use portable_pty::{CommandBuilder, PtySize, PtySystem, native_pty_system};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use strip_ansi::strip_ansi;

struct Pinpontty {
    rx: Receiver<String>,
    last_output: String,
}

fn app_logic(data: &mut Pinpontty) -> impl WidgetView<Pinpontty> + use<> {
    while let Ok(output) = data.rx.try_recv() {
        data.last_output = output;
    }
    flex(Axis::Vertical, (label(format!("{}", data.last_output)),))
}

fn main() -> Result<(), EventLoopError> {
    let pty_system = native_pty_system();

    let mut pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            // Not all systems support pixel_width, pixel_height,
            // but it is good practice to set it to something
            // that matches the size of the selected font.  That
            // is more complex than can be shown here in this
            // brief example though!
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();
    let cmd = CommandBuilder::new("nu");
    let child = pair.slave.spawn_command(cmd).unwrap();
    let mut reader = pair.master.try_clone_reader().unwrap();
    let writer = pair.master.take_writer().unwrap();

    let (tx, rx) = channel();

    ::std::thread::spawn(move || {
        use std::io::Read;
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf) {
                Ok(n) => {
                    let s = String::from_utf8_lossy(&buf[..n]);
                    println!("Read: {}", s);
                    tx.send(strip_ansi(s.as_ref())).ok();
                }
                Err(e) => {
                    eprintln!("Error reading from pty: {}", e);
                    break;
                }
            }
        }
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
