use makepad_widgets::*;

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl AppMain for App {
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

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

makepad_widgets::app_main!(App);
