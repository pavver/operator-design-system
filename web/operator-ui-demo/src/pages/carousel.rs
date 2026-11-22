//! Live examples for carousel.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn carousel() -> AnyView {
    let current = RwSignal::new(0usize);

    view! {
        <Example title="Interactive Carousel" note="Slides with arrows, dots, keyboard navigation (Left/Right), swipe, and autoplay." block=true>
            <div style="max-width: calc(var(--op-du) * 150); width: 100%;">
                <Carousel
                    current=current
                    count=3
                    height=50.0
                    autoplay=false
                >
                    <CarouselSlide>
                        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; background: #1a2233; color: #fff; padding: calc(var(--op-du) * 6); text-align: center;">
                            <span style="font-weight: 700; font-size: var(--op-type-section-title-size);">"Slide 1: Telemetry Hub"</span>
                            <p style="margin-top: calc(var(--op-du) * 2); font-size: var(--op-type-body-size); opacity: 0.8;">
                                "Real-time metrics streaming with zero latency."
                            </p>
                        </div>
                    </CarouselSlide>
                    <CarouselSlide>
                        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; background: #22331a; color: #fff; padding: calc(var(--op-du) * 6); text-align: center;">
                            <span style="font-weight: 700; font-size: var(--op-type-section-title-size);">"Slide 2: Security Perimeter"</span>
                            <p style="margin-top: calc(var(--op-du) * 2); font-size: var(--op-type-body-size); opacity: 0.8;">
                                "Hardware-enforced encryption and strict identity verification."
                            </p>
                        </div>
                    </CarouselSlide>
                    <CarouselSlide>
                        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; background: #33221a; color: #fff; padding: calc(var(--op-du) * 6); text-align: center;">
                            <span style="font-weight: 700; font-size: var(--op-type-section-title-size);">"Slide 3: Automated Actions"</span>
                            <p style="margin-top: calc(var(--op-du) * 2); font-size: var(--op-type-body-size); opacity: 0.8;">
                                "Declarative workflows triggered on critical state transitions."
                            </p>
                        </div>
                    </CarouselSlide>
                </Carousel>
            </div>
        </Example>
    }
    .into_any()
}
