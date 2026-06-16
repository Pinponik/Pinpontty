use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*;
    use mod.widgets.*;

    // Register the base widget type (replace `DesktopNewViewBase` with your own name)
    mod.widgets.ViewBase = #(View::register_widget(vm))

    // Define the default widget template (you can customize the UI here)
    mod.widgets.View = set_type_default() do mod.widgets.ViewBase {
        // Example: a simple rectangle view – replace with your actual UI
        rect := RectView {
            width: Fill
            height: Fill
            draw_bg +: {
                color: #fff
            }
        }
    }
}

// The widget struct – derives the necessary Makepad traits
#[derive(Script, ScriptHook, WidgetRef, WidgetSet, WidgetRegister)]
pub struct View {
    #[uid]
    uid: WidgetUid,
    // Add any `@live` fields you need here (e.g., colors, sizes, etc.)
    // #[live]
    // example_color: Color = #000,
}

impl WidgetNode for View {
    fn widget_uid(&self) -> WidgetUid {
        self.uid
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        // Delegate walking to your inner widget(s) – adjust as needed
        // For a simple RectView wrapper you could do:
        // self.rect.walk(cx)
        // For now we just return a default walk.
        Walk::default()
    }

    fn area(&self) -> Area {
        // Again, delegate to inner widget(s)
        Area::default()
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        // Delegate redraw to inner widget(s)
    }

    fn find_widgets_from_point(&self, _cx: &Cx, _point: DVec2, _found: &mut dyn FnMut(&WidgetRef)) {
        // Delegate hit‑testing to inner widget(s)
    }

    fn visible(&self) -> bool {
        // Delegate visibility check
        true
    }

    fn set_visible(&mut self, _cx: &mut Cx, _visible: bool) {
        // Delegate visibility setting
    }
}

impl Widget for View {
    fn draw_walk(&mut self, _cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        // Your drawing logic goes here.
        // For a simple pass‑through you could do:
        // self.rect.draw_walk(cx, scope, walk)
        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        // Handle widget events (e.g., button clicks, gestures, etc.)
        // Example:
        // if let Event::Actions(actions) = event {
        //     // Process actions...
        // }
    }
}

// Optional: add a Ref type for convenient external access (like other views do)
// impl ViewRef {
//     pub fn some_method(&self, cx: &mut Cx, ...) -> ... {
//         if let Some(inner) = self.borrow() {
//             inner.some_method(cx, ...);
//         }
//     }
// }
