use super::super::VirtualList;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    description: String,
    size: f64,
}

#[component]
pub fn Demo() -> Element {
    let users = use_signal(|| {
        (0..10000)
            .map(|i| {
                let size = 60.0 + ((i * 37) % 100) as f64; // Vary heights between 60-160px
                User {
                    name: format!("User {}", i + 1),
                    description: format!("Description for user {}. This is item number {} in the list.", i + 1, i + 1),
                    size,
                }
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        div { style: "width: 100%; max-width: 600px; margin: 0 auto;",

            h2 { style: "margin-bottom: 1rem;", "Virtual List Demo (10,000 items)" }

            VirtualList {
                data_len: users().len(),
                height: "500px".to_string(),
                estimated_item_height: 80.0,
                overscan: 5,
                item_content: move |index: usize| {
                    let user = &users.read()[index];
                    rsx! {
                        div { class: "virtual-list-item", style: "height: {user.size}px;",
                            p {
                                strong { "{user.name}" }
                            }
                            div { "{user.description}" }
                        }
                    }
                },
            }
        }
    }
}
