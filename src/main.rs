use makepad_widgets::*;

pub struct App {
    ui: WidgetRef,
}

impl App {
    fn new() -> Self {
        Self {
            ui: WidgetRef::default(),
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        script_mod! {
            pub app := Window {
                caption: "Pinpontty Terminal",
                window_size: vec2(1024.0, 768.0),

                body := View {
                    width: fill,
                    height: fill,
                    background: #000000
                }
            }
        }
        script_mod(vm)
    }
}

impl ScriptApply for App {}
impl ScriptHook for App {}
impl ScriptNew for App {
    fn script_new(vm: &mut ScriptVm) -> ScriptValue {
        ScriptValue::new(vm, App::new())
    }
}

makepad_widgets::app_main!(App);
