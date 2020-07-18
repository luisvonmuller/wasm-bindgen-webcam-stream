use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::*;

#[wasm_bindgen(start)]
pub async fn start() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let _body = document.body().expect("document should have a body");

    /* Dom Reference */
    let video = document.get_element_by_id("webcam-stream").unwrap();

    /* Parsing it into a wasm sysm video track */
    let video: web_sys::HtmlVideoElement = video.dyn_into::<web_sys::HtmlVideoElement>().unwrap();

    /* later I'll need to implement a event listner here */
    video.set_width(1280);
    video.set_height(720);

    let navigator = web_sys::Window::navigator(&window);
    let media_devices = web_sys::Navigator::media_devices(&navigator);
    let permissions = web_sys::Navigator::permissions(&navigator);

    let user_media_promisse = match media_devices {
        Ok(user_media_content) => web_sys::MediaDevices::get_user_media_with_constraints(
            &user_media_content,
            web_sys::MediaStreamConstraints::video(
                &mut web_sys::MediaStreamConstraints::new(),
                &video.clone().into(),
            ),
        ),
        Err(err_msg) => Err(err_msg),
    };

    let user_media = match user_media_promisse {
        Ok(media_input) => wasm_bindgen_futures::JsFuture::from(media_input).await,
        Err(err_msg) => Err(err_msg),
    };

    let media_stream = match user_media {
        Ok(media_stream_input) => media_stream_input.dyn_into::<web_sys::MediaStream>(),
        Err(err_msg) => Err(err_msg),
    };  

    match media_stream {
        Ok(input) => { 
            video.set_src_object(Some(&input));
        },
        Err(err_msg) => {
            web_sys::console::log(&err_msg.into());
        },
    }

}
