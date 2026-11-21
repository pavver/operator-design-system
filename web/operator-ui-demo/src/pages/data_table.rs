//! Live examples for data table.

use crate::kit::{Example, backend, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

fn camera_rows() -> Vec<TableRow> {
    [
        ("c1", "Gate North", "Online", "12", "H.265", "4K"),
        ("c2", "Gate South", "Online", "18", "H.265", "1080p"),
        ("c3", "Yard East", "Offline", "—", "H.264", "1080p"),
        ("c4", "Parking A", "Online", "40", "H.264", "720p"),
        ("c5", "Warehouse dock", "Alarm", "9", "H.265", "4K"),
        ("c6", "Server room", "Online", "3", "MJPEG", "720p"),
        ("c7", "Reception", "Online", "22", "H.265", "1080p"),
        ("c8", "Roof", "Offline", "—", "H.264", "4K"),
    ]
    .into_iter()
    .map(|(id, a, b, c, d, e)| TableRow::new(id, [a, b, c, d, e]))
    .collect()
}

pub fn data_table() -> AnyView {
    let b = backend();
    let rows = RwSignal::new(camera_rows());
    let selected = RwSignal::new(Some("c5".to_string()));
    let loading = RwSignal::new(false);
    let columns = || {
        vec![
            Column::new("Camera").sortable(),
            Column::new("Status").sortable().width(24.0),
            Column::new("RTT, ms").numeric().sortable().width(22.0),
            Column::new("Codec").width(20.0),
            Column::new("Resolution").width(24.0),
        ]
    };
    let no_rows: Vec<TableRow> = Vec::new();
    let reload = AsyncAction::new(move |()| async move {
        loading.set(true);
        let r = b.call("reload cameras".into()).await;
        loading.set(false);
        r
    });
    view! {
        <Example title="Sortable, selectable" note="Click a header to sort ascending, descending, then back to the original order." block=true>
            <DataTable
                columns=columns()
                rows=rows
                selected=selected
                loading=loading
                max_height=70.0
                label="Cameras"
                on_row_click=Callback::new(move |id: String| b.log(format!("[UI] row {id}")))
            />
            <div class="demo-inline">
                <ActionButton text="Reload" icon=icons::REFRESH on_click=reload/>
                <ActionButton text="Clear rows" on_click=instant(move || rows.set(Vec::new()))/>
                <ActionButton text="Restore rows" on_click=instant(move || rows.set(camera_rows()))/>
            </div>
        </Example>
        <Example title="Dense, empty" block=true>
            <DataTable
                columns=columns()
                rows=no_rows
                dense=true
                empty=ViewFn::from(|| view! { <EmptyState title="No cameras" description="Add a camera to see it here." compact=true/> })
            />
        </Example>
    }
    .into_any()
}
