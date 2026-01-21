use super::super::VirtualList;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    description: String,
    size: f64,
    color: String,
}

#[component]
pub fn Demo() -> Element {
    let mut users = use_signal(|| {
        (0..10000)
            .map(|i| {
                let size = 60.0 + ((i * 37) % 100) as f64; // Vary heights between 60-160px
                let random_color = format!(
                    "hsl({}, 70%, 80%)",
                    (i * 137) % 360 // Generate a color based on index
                );
                let mut description = format!(
                    "Description for user {}. This is item number {} in the list.",
                    i + 1,
                    i + 1
                );
                // concat description multiple times to increase text length
                for _ in 0..(1 + (i % 5)) {
                    // repeat 1 to 5 times
                    description.push_str(&description.clone());
                }
                User {
                    name: format!("User {}", i + 1),
                    description,
                    size: size * 3.0,
                    color: random_color,
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
                estimated_item_height: 20.0,
                overscan: 5,
                item_content: move |index: usize| {
                    let user = &users.read()[index];
                    rsx! {
                        div { class: "virtual-list-item",
                            div { style: "height: {user.size}px; background-color: {user.color}; padding: 0.5rem; border-bottom: 1px solid #ccc;",
                                button {
                                    style: "z-index: 1; margin: 0.5rem;",
                                    onclick: move |_| {
                                        let new_size = 60.0 + ((index * 53) % 100) as f64;
                                        if let Some(u) = users.write().get_mut(index) {
                                            u.size = new_size * 3.0;
                                        }
                                    },
                                    "Click Me"
                                }
                                p {
                                    strong { "{user.name}" }
                                }
                                div { "{user.description}" }
                            }

        
                        }
                    }
                },
            }
        }
    }
}
