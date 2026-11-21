//! Live examples for uploader.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn uploader() -> AnyView {
    let b = backend();
    let upload = move || {
        AsyncAction::new(move |request: UploadRequest| async move {
            for n in 1..=5 {
                sleep_ms(120).await;
                request.progress.run(n as f32 / 5.0);
            }
            b.call(format!("upload {}", request.file.name())).await
        })
    };
    view! {
        <Example title="Interactive" block=true>
            <Uploader on_upload=upload()/>
        </Example>
        <Example title="States" block=true>
            <Uploader on_upload=upload() enabled=false/>
        </Example>
        <Example title="Customization" block=true>
            <Uploader on_upload=upload() label=l("Attachments", "Вкладення") gap=3.0 padding=5.0 footer=|| view! { <span>"Select files, then upload each row; failed rows can be retried."</span> }/>
        </Example>
    }.into_any()
}
