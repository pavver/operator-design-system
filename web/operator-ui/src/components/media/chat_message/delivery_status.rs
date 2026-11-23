//! Message delivery status indicator component.

use crate::components::chat_message::types::DeliveryStatus as StatusEnum;
use crate::components::primitives::{Icon, ProgressIndicator};
use crate::core::{Metric, root_class, root_style};
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::icons;

/// Delivery and reception status badge.
#[component]
pub fn DeliveryStatus(
    /// The delivery status to display.
    status: StatusEnum,
    /// Size in DU (`--op-delivery-status-size`). Default is 4 DU.
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <span
            class=root_class("op-delivery-status", class)
            data-status=match status {
                StatusEnum::Sending => "sending",
                StatusEnum::Sent => "sent",
                StatusEnum::Delivered => "delivered",
                StatusEnum::Read => "read",
                StatusEnum::Failed => "failed",
            }
            style=root_style(vec![Metric::du("--op-delivery-status-size", size)], style)
            role="status"
            aria-label=match status {
                StatusEnum::Sending => "Sending",
                StatusEnum::Sent => "Sent",
                StatusEnum::Delivered => "Delivered",
                StatusEnum::Read => "Read",
                StatusEnum::Failed => "Failed",
            }
        >
            {match status {
                StatusEnum::Sending => view! {
                    <ProgressIndicator size=4.0 />
                }.into_any(),
                StatusEnum::Sent => view! {
                    <span class="op-delivery-single-check">
                        <Icon icon=icons::CHECK size=dim::ICON_XSMALL />
                    </span>
                }.into_any(),
                StatusEnum::Delivered => view! {
                    <span class="op-delivery-double-check">
                        <Icon icon=icons::CHECK size=dim::ICON_XSMALL />
                        <Icon icon=icons::CHECK size=dim::ICON_XSMALL class="op-delivery-check-overlap" />
                    </span>
                }.into_any(),
                StatusEnum::Read => view! {
                    <span class="op-delivery-double-check op-delivery-read">
                        <Icon icon=icons::CHECK size=dim::ICON_XSMALL />
                        <Icon icon=icons::CHECK size=dim::ICON_XSMALL class="op-delivery-check-overlap" />
                    </span>
                }.into_any(),
                StatusEnum::Failed => view! {
                    <span class="op-delivery-stop-sign" aria-hidden="true">
                        <span class="op-delivery-stop-octagon">
                            <span class="op-delivery-exclamation">"!"</span>
                        </span>
                    </span>
                }.into_any(),
            }}
        </span>
    }
}

/// Convenience alias for the delivery status component.
pub use DeliveryStatus as DeliveryStatusView;
