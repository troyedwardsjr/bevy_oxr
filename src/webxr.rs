#[cfg(feature = "webxr")]
mod webxr {
    use wasm_bindgen::prelude::*;
    use web_sys::{XrSession, WebGl2RenderingContext};
    use js_sys::try_iter;
    use wasm_bindgen::JsCast;
    use bevy::prelude::*;

    pub struct WebXrPlugin;

    impl Plugin for WebXrPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_system(handle_input.system())
                .add_system(render_frame.system())
                .add_system(add_event_listeners.system())
                .add_startup_system(initialize_webxr.system());
        }
    }

    #[wasm_bindgen]
    pub fn handle_input(session: &web_sys::XrSession, mut events: ResMut<Events<YourEvent>>) {
        let input_sources = session.input_sources().unwrap();
        for input_source in js_sys::try_iter(&input_sources).unwrap().unwrap() {
            let input_source = input_source.unwrap().dyn_into::<web_sys::XrInputSource>().unwrap();
            
            // Get the input source's handedness
            let handedness = input_source.handedness();
            
            // Get the input source's target ray mode
            let target_ray_mode = input_source.target_ray_mode();
            
            // Process the input source based on its handedness and target ray mode
            match (handedness.as_str(), target_ray_mode.as_str()) {
                ("left", "gaze") => {
                    // Dispatch an event when a left-handed gaze input is detected
                    events.send(YourEvent::LeftHandGaze);
                },
                // Add more cases as needed...
                _ => {
                    // Default case...
                },
            }
        }
    }

    #[wasm_bindgen]
    pub fn render_frame(session: &web_sys::XrSession, gl: &WebGl2RenderingContext) {
        let frame = session.request_animation_frame().unwrap();
        let pose = frame.get_viewer_pose().unwrap();

        for view in js_sys::try_iter(&pose.views()).unwrap().unwrap() {
            let view = view.unwrap().dyn_into::<web_sys::XrView>().unwrap();

            // Set up WebGL context for rendering the view
            gl.viewport(0, 0, view.width(), view.height());

            // TODO: Render the scene using the view's projection matrix and view transform
        }
    }

    #[wasm_bindgen]
    pub fn add_event_listeners(session: &web_sys::XrSession) {
        let onend_callback = Closure::wrap(Box::new(move || {
            // Handle session end event here...
        }) as Box<dyn FnMut()>);

        session.set_onend(Some(onend_callback.as_ref().unchecked_ref()));
        onend_callback.forget(); // Don't drop the closure while it's still in use

        let oninputsourceschange_callback = Closure::wrap(Box::new(move || {
            // Handle input sources change event here...
        }) as Box<dyn FnMut()>);

        session.set_oninputsourceschange(Some(oninputsourceschange_callback.as_ref().unchecked_ref()));
        oninputsourceschange_callback.forget(); // Don't drop the closure while it's still in use
    }

    pub async fn initialize_webxr() -> Result<XrSession, JsValue> {
        // Get the XR system
        let xr_system = web_sys::window().unwrap().navigator().xr().unwrap();

        // Check if XR is supported
        if xr_system.is_session_supported("immersive-vr").await.unwrap() {
            // Request an XR session
            let xr_session = xr_system.request_session("immersive-vr", &JsValue::NULL).await.unwrap();

            Ok(xr_session)
        } else {
            Err(JsValue::from_str("XR not supported"))
        }
    }
}