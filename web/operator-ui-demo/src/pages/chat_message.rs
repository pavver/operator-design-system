//! Live examples for ChatMessage, delivery statuses, TTL indicators, and reactions.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

const SVG_PHOTO_1: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='300' height='200' viewBox='0 0 300 200'><rect width='300' height='200' fill='%231e293b'/><circle cx='150' cy='100' r='50' fill='%2300a0ff' opacity='0.4'/><text x='150' y='105' fill='%23ffffff' font-family='monospace' font-size='14' text-anchor='middle'>SATELLITE OPTICS</text></svg>";
const SVG_PHOTO_2: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='300' height='200' viewBox='0 0 300 200'><rect width='300' height='200' fill='%230f172a'/><rect x='40' y='40' width='220' height='120' fill='%2322c55e' opacity='0.25'/><text x='150' y='105' fill='%2322c55e' font-family='monospace' font-size='14' text-anchor='middle'>RADAR SWEEP</text></svg>";
const SVG_STICKER: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='140' height='140' viewBox='0 0 140 140'><rect width='140' height='140' fill='%23000000' opacity='0'/><polygon points='70,10 90,50 135,55 100,90 110,135 70,110 30,135 40,90 5,55 50,50' fill='%23eab308'/><text x='70' y='80' fill='%23000000' font-family='monospace' font-size='20' font-weight='bold' text-anchor='middle'>ODS</text></svg>";
const SVG_AVATAR_OP: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='32' height='32' viewBox='0 0 32 32'><rect width='32' height='32' fill='%2300a0ff'/><text x='16' y='21' fill='%23ffffff' font-family='sans-serif' font-size='14' font-weight='bold' text-anchor='middle'>OP</text></svg>";

pub fn chat_message() -> AnyView {
    // Interactive signal for voting in demo poll
    let poll_options = RwSignal::new(vec![
        PollOption {
            id: "opt1".to_string(),
            text: "Sector A (Hydro)".to_string(),
            vote_count: 14,
            is_selected: false,
        },
        PollOption {
            id: "opt2".to_string(),
            text: "Sector B (Telemetry)".to_string(),
            vote_count: 28,
            is_selected: true,
        },
        PollOption {
            id: "opt3".to_string(),
            text: "Sector C (Perimeter)".to_string(),
            vote_count: 7,
            is_selected: false,
        },
    ]);

    let on_poll_vote = Callback::new(move |selected_id: String| {
        poll_options.update(|opts| {
            for opt in opts.iter_mut() {
                if opt.id == selected_id {
                    opt.is_selected = !opt.is_selected;
                    if opt.is_selected {
                        opt.vote_count += 1;
                    } else if opt.vote_count > 0 {
                        opt.vote_count -= 1;
                    }
                }
            }
        });
    });

    // Interactive reactions signal
    let reactions_sig = RwSignal::new(vec![
        MessageReaction::new("👍", 5, true),
        MessageReaction::new("🔥", 12, false),
        MessageReaction::new("⚡", 3, false),
    ]);

    let on_reaction_click = Callback::new(move |emoji: String| {
        reactions_sig.update(|rxs| {
            for rx in rxs.iter_mut() {
                if rx.emoji == emoji {
                    rx.is_selected_by_me = !rx.is_selected_by_me;
                    if rx.is_selected_by_me {
                        rx.count += 1;
                    } else if rx.count > 0 {
                        rx.count -= 1;
                    }
                }
            }
        });
    });

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 160);">
            // Example 1: Conversation thread with consecutive message grouping and delivery statuses
            <Example
                title="Conversation Thread & Grouping"
                note="Consecutive message grouping, sender roles, timestamps, and delivery verification."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 1);width:100%;">
                    <ChatMessage
                        sent=false
                        name="Operator Dispatch"
                        role="DISPATCHER"
                        avatar=SVG_AVATAR_OP
                        grouping=MessageGrouping::First
                        stamp="10:14 AM"
                        text="Sector 4 radar telemetry anomalous. Initiating sensor handshake."
                    />
                    <ChatMessage
                        sent=false
                        avatar=SVG_AVATAR_OP
                        grouping=MessageGrouping::Middle
                        stamp="10:14 AM"
                        text="Secondary frequency responding on beacon 0x4F."
                    />
                    <ChatMessage
                        sent=false
                        avatar=SVG_AVATAR_OP
                        grouping=MessageGrouping::Last
                        stamp="10:15 AM"
                        text="Awaiting local field acknowledgement."
                    />
                    <ChatMessage
                        sent=true
                        name="You"
                        stamp="10:16 AM"
                        delivery_status=DeliveryStatus::Read
                        text="Acknowledge. Diagnostic routine underway. Signal confirmed."
                    />
                </div>
            </Example>

            // Example 2: Rich Content & Media Types
            <Example
                title="Rich Content Payloads"
                note="Voice waveforms, code blocks, blockquotes, media grids with spoilers, polls, and cards."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2.5);width:100%;">
                    // Voice Memo
                    <ChatMessage
                        sent=false
                        name="Comms Lead"
                        stamp="10:20 AM"
                        content=MessageContent::Voice(VoiceAttachment {
                            url: None,
                            duration_seconds: 18,
                            waveform: vec![4, 8, 14, 22, 28, 18, 12, 16, 24, 20, 15, 9, 13, 21, 27, 19, 11, 7, 14, 23, 17, 8],
                        })
                    />

                    // Code Block with Copy
                    <ChatMessage
                        sent=true
                        name="You"
                        stamp="10:22 AM"
                        delivery_status=DeliveryStatus::Read
                        content=MessageContent::CodeBlock {
                            language: "rust".to_string(),
                            code: "let status = DeliveryStatus::Read;\nprintln!(\"Telemetry sync nominal: {:?}\", status);".to_string(),
                        }
                    />

                    // Blockquote
                    <ChatMessage
                        sent=false
                        name="HQ Directive"
                        stamp="10:25 AM"
                        content=MessageContent::Blockquote {
                            text: "Field units must maintain passive RF listening mode during sector calibration.".to_string(),
                            is_collapsible: true,
                        }
                    />

                    // Media Grid with spoiler
                    <ChatMessage
                        sent=true
                        name="You"
                        stamp="10:28 AM"
                        delivery_status=DeliveryStatus::Delivered
                        content=MessageContent::MediaGrid(vec![
                            MediaItem::photo(SVG_PHOTO_1).with_spoiler(true),
                            MediaItem::video(SVG_PHOTO_2, 45),
                        ])
                    />

                    // Interactive Voting Poll
                    {move || {
                        let opts = poll_options.get();
                        let total: u32 = opts.iter().map(|o| o.vote_count).sum();
                        view! {
                            <ChatMessage
                                sent=false
                                name="Shift Commander"
                                stamp="10:30 AM"
                                on_poll_vote=on_poll_vote
                                content=MessageContent::Poll(PollAttachment {
                                    question: "Which sector requires emergency backup routing?".to_string(),
                                    options: opts,
                                    total_votes: total,
                                    is_closed: false,
                                    is_anonymous: false,
                                })
                            />
                        }
                    }}

                    // Web Page Preview Card
                    <ChatMessage
                        sent=false
                        name="Intel Bot"
                        stamp="10:32 AM"
                        content=MessageContent::WebPage(WebPagePreview {
                            url: "https://operator-design.org".to_string(),
                            site_name: Some("Operator System".to_string()),
                            title: Some("Standard Communication Protocols v2.4".to_string()),
                            description: Some("Unified schema documentation for tactical telemetry messages and sensor channels.".to_string()),
                            preview_image_url: Some(SVG_PHOTO_2.to_string()),
                        })
                    />

                    // Invoice Card
                    <ChatMessage
                        sent=false
                        name="Logistics Depot"
                        stamp="10:35 AM"
                        content=MessageContent::Invoice(InvoiceAttachment {
                            title: "Hardware Telemetry Module Kit".to_string(),
                            description: "Tactical mesh transceiver and antenna array.".to_string(),
                            currency: "⭐️".to_string(),
                            total_amount: "350".to_string(),
                            photo_url: Some(SVG_PHOTO_1.to_string()),
                            is_paid: false,
                            invoice_id: "INV-9041".to_string(),
                        })
                    />

                    // Story Card
                    <ChatMessage
                        sent=false
                        name="Field Unit 7"
                        stamp="10:40 AM"
                        content=MessageContent::Story(StoryAttachment {
                            author_name: "Field Team Bravo".to_string(),
                            author_avatar_url: None,
                            media_url: SVG_PHOTO_1.to_string(),
                            is_video: true,
                            story_id: "STORY-12".to_string(),
                        })
                    />
                </div>
            </Example>

            // Example 3: Markdown, TTL Self-Destruct, Replies, Reactions & Inline Buttons
            <Example
                title="Attribution, Reactions, TTL & Actions"
                note="Reply quote, forward attribution, self-destruct countdown, and bot keyboard."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;">
                    <ChatMessage
                        sent=true
                        name="You"
                        stamp="10:42 AM"
                        delivery_status=DeliveryStatus::Read
                        is_edited=true
                        ttl=MessageTtlInfo::new(60, 8, true)
                        forward_info=ForwardHeader::new("General Command")
                        reply_to=MessageReply::new("msg-1", "Operator Dispatch", "Sector 4 radar telemetry anomalous.")
                        text="Confirmed with **high priority**. Standby for ~~abort~~ *execution*."
                        reactions=reactions_sig.get()
                        on_reaction_click=on_reaction_click
                        inline_keyboard=vec![
                            vec![
                                InlineKeyboardButton::callback("Acknowledge", "ack"),
                                InlineKeyboardButton::callback("Escalate", "esc"),
                            ],
                            vec![
                                InlineKeyboardButton::link("Open Radar", "https://operator-design.org"),
                            ]
                        ]
                    />
                </div>
            </Example>

            // Example 4: Standalone Stickers, Video Notes & Isolated Big Emojis
            <Example
                title="Bubble-less Elements: Stickers, Video Notes & Big Emojis"
                note="Pure 1-3 emojis render big without a bubble; circular video note and standalone stickers."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;">
                    // Standalone Video Note (40 DU circular viewfinder)
                    <ChatMessage
                        sent=false
                        stamp="10:45 AM"
                        content=MessageContent::VideoNote(VideoNoteAttachment {
                            url: "".to_string(),
                            duration_seconds: 14,
                            thumbnail_url: Some(SVG_PHOTO_2.to_string()),
                        })
                    />

                    // Standalone Sticker (35 DU)
                    <ChatMessage
                        sent=true
                        stamp="10:46 AM"
                        delivery_status=DeliveryStatus::Read
                        content=MessageContent::Sticker(StickerAttachment {
                            url: SVG_STICKER.to_string(),
                            format: StickerFormat::VectorSvg,
                            emoji: Some("⭐".to_string()),
                            pack_id: Some("ods_tactical".to_string()),
                            sticker_id: Some("star_1".to_string()),
                        })
                    />

                    // Isolated Big Emojis (1-3 emojis render large without bubble!)
                    <ChatMessage
                        sent=false
                        stamp="10:48 AM"
                        text="🔥 ⚡"
                    />

                    <ChatMessage
                        sent=true
                        stamp="10:49 AM"
                        delivery_status=DeliveryStatus::Read
                        text="🚀"
                    />
                </div>
            </Example>

            // Example 5: Status Badges and Delivery Status Indicators
            <Example
                title="Chat Status Badges & Delivery Indicators"
                note="Tactical delivery checkmarks, Stop sign failure, unread counts, and syncing indicators."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;">
                    <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 3);flex-wrap:wrap;">
                        <span style="font-weight:700;">"Delivery Statuses:"</span>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Sending:"</span>
                            <DeliveryStatusView status=DeliveryStatus::Sending />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Sent:"</span>
                            <DeliveryStatusView status=DeliveryStatus::Sent />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Delivered:"</span>
                            <DeliveryStatusView status=DeliveryStatus::Delivered />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Read:"</span>
                            <DeliveryStatusView status=DeliveryStatus::Read />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Failed:"</span>
                            <DeliveryStatusView status=DeliveryStatus::Failed />
                        </div>
                    </div>

                    <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 3);flex-wrap:wrap;">
                        <span style="font-weight:700;">"Chat Status Badges:"</span>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Unread:"</span>
                            <ChatStatusBadge status=ChatStatus::Unread {
                                count: 12,
                                is_muted: false,
                                is_mentioned: true,
                                mention_count: 2,
                                is_critical: false,
                            } />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Critical:"</span>
                            <ChatStatusBadge status=ChatStatus::Unread {
                                count: 104,
                                is_muted: false,
                                is_mentioned: false,
                                mention_count: 0,
                                is_critical: true,
                            } />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Syncing:"</span>
                            <ChatStatusBadge status=ChatStatus::Syncing />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Pinned:"</span>
                            <ChatStatusBadge status=ChatStatus::Pinned />
                        </div>
                        <div style="display:flex;align-items:center;gap:calc(var(--op-du) * 1);">
                            <span>"Muted:"</span>
                            <ChatStatusBadge status=ChatStatus::Muted />
                        </div>
                    </div>
                </div>
            </Example>
        </div>
    }
    .into_any()
}
