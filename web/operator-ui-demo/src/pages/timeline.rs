//! Live examples for Timeline.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn timeline() -> AnyView {
    view! {
        <Example title="Standard timeline" note="Milestones with tones and icons." block=true>
            <div class="demo-stack demo-narrow">
                <Timeline>
                    <TimelineEntry
                        title="Deployment started"
                        date="Today, 14:32"
                        tone="pending"
                        icon=icons::REFRESH
                    >
                        "Container build initiated on runner cluster-west."
                    </TimelineEntry>
                    <TimelineEntry
                        title="Unit tests passed"
                        date="Today, 14:35"
                        tone="success"
                        icon=icons::CHECK
                    >
                        "342 unit tests completed with 0 failures."
                    </TimelineEntry>
                    <TimelineEntry
                        title="Release published"
                        date="Today, 14:40"
                        tone="accent"
                        icon=icons::CHECK_CIRCLE
                    >
                        "Version 2.4.0 is now live in production."
                    </TimelineEntry>
                </Timeline>
            </div>
        </Example>
        <Example title="Dense mode" note="Reduced vertical spacing." block=true>
            <div class="demo-stack demo-narrow">
                <Timeline dense=true>
                    <TimelineEntry title="Event 1" date="10:00" tone="success" icon=icons::CHECK />
                    <TimelineEntry title="Event 2" date="11:30" tone="warning" icon=icons::WARNING />
                    <TimelineEntry title="Event 3" date="12:15" tone="error" icon=icons::ERROR />
                </Timeline>
            </div>
        </Example>
    }
    .into_any()
}
