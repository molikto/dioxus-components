# Virtual List Component

Efficiently renders large lists by only rendering visible items within the viewport. Supports dynamically sized items by measuring their heights as they render.

## Features

- **Virtual Scrolling**: Only renders items that are visible in the viewport
- **Dynamic Heights**: Automatically measures and adapts to varying item heights
- **Performance**: Handles tens of thousands of items smoothly
- **Simple API**: Similar to React Virtuoso

## Usage

```rust
use dioxus::prelude::*;
use dioxus_primitives::virtual_list::VirtualList;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    description: String,
}

#[component]
fn MyList() -> Element {
    let users = use_signal(|| vec![
        User { name: "Alice".to_string(), description: "Engineer".to_string() },
        // ... thousands more
    ]);

    rsx! {
        VirtualList {
            data: users,
            height: "100%".to_string(),
            item_content: move |(index, user): (usize, User)| rsx! {
                div {
                    style: "padding: 0.5rem; border-bottom: 1px solid #ccc;",
                    p { strong { "{user.name}" } }
                    div { "{user.description}" }
                }
            }
        }
    }
}
```

## Props

- `data`: ReadSignal containing the list of items
- `item_content`: Callback that renders each item (receives index and item data)
- `height`: Container height (default: "400px")
- `estimated_item_height`: Initial estimate for item heights before measurement (default: 50.0)
- `overscan`: Number of extra items to render outside viewport for smoother scrolling (default: 3)
