//! Virtual list component that efficiently renders large lists by only rendering visible items.
//!
//! Supports dynamically sized items by measuring their heights as they are rendered.

use dioxus::prelude::*;
use std::collections::HashMap;

/// Props for the VirtualList component
#[derive(Props, Clone, PartialEq)]
pub struct VirtualListProps {
    /// Disable virtualization and render all items
    #[props(default = false)]
    pub disable_virtualization: bool,

    /// The number of items in the list
    pub data_len: ReadSignal<usize>,

    /// Function that renders each item given its index
    pub item_content: Callback<usize, Element>,

    /// Estimated item height for initial render (before measuring)
    #[props(default = 50.0)]
    pub estimated_item_height: f64,

    /// Number of items to render outside visible area (for smoother scrolling)
    #[props(default = 3)]
    pub overscan: usize,

    /// Additional attributes to apply to the container
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Copy, PartialEq)]
struct ItemMeasurement {
    height: f64,
    offset: f64,
}

/// # VirtualList
///
/// A virtual scrolling component that efficiently renders large lists by only rendering
/// visible items. Supports dynamically sized items.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::virtual_list::VirtualList;
///
/// #[derive(Clone, PartialEq)]
/// struct User {
///     name: String,
///     description: String,
///     size: f64,
/// }
///
/// #[component]
/// fn Demo() -> Element {
///     let users = use_signal(|| vec![
///         User {
///             name: "Alice".to_string(),
///             description: "Engineer".to_string(),
///             size: 80.0,
///         },
///         // ... more users
///     ]);
///
///     rsx! {
///         VirtualList {
///             data_len: users.len(),
///             style: "height: 100%;",
///             item_content: move |index: usize| rsx! {
///                 div {
///                     style: "padding: 0.5rem; border-bottom: 1px solid #ccc;",
///                     p { strong { "{users()[index].name}" } }
///                     div { "{users()[index].description}" }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn VirtualList(props: VirtualListProps) -> Element {
    if props.disable_virtualization {
        // Render all items without virtualization
        let items = (0..*props.data_len.read()).map(|i| {
            let item_content = props.item_content.clone();
            item_content.call(i)
        });

        return rsx! {
            div { ..props.attributes,{items} }
        };
    }
    let mut scroll_top = use_signal(|| 0.0);
    let mut container_height = use_signal(|| 400.0);
    let item_heights = use_signal(|| HashMap::<usize, f64>::new());
    let mut item_offsets = use_signal(|| HashMap::<usize, f64>::new());

    let item_count = props.data_len;

    // Calculate total height and item offsets
    let measurements = use_memo(move || {
        let count = item_count();
        let heights = item_heights.read();
        let mut offset = 0.0;
        let mut measurements = HashMap::new();

        for i in 0..count {
            let height = heights
                .get(&i)
                .copied()
                .unwrap_or(props.estimated_item_height);
            measurements.insert(i, ItemMeasurement { height, offset });
            offset += height;
        }

        measurements
    });

    let total_height = use_memo(move || {
        let measurements = measurements();
        measurements.values().map(|m| m.height).sum::<f64>()
    });

    // Update item offsets cache
    use_effect(move || {
        let measurements_val = measurements();
        let mut offsets = item_offsets.write();
        offsets.clear();
        for (idx, measurement) in measurements_val.iter() {
            offsets.insert(*idx, measurement.offset);
        }
    });

    // Calculate visible range
    let visible_range = use_memo(move || {
        let scroll = scroll_top();
        let viewport_height = container_height();
        let count = item_count();
        let measurements_val = measurements();

        if count == 0 {
            return (0, 0);
        }

        // Binary search for start index - find first item that overlaps with viewport
        // An item overlaps if its bottom edge (offset + height) > scroll_top
        let mut low = 0;
        let mut high = count;
        while low < high {
            let mid = (low + high) / 2;
            let m = measurements_val.get(&mid);
            let offset = m.map(|m| m.offset).unwrap_or(mid as f64 * props.estimated_item_height);
            let height = m.map(|m| m.height).unwrap_or(props.estimated_item_height);
            if offset + height <= scroll {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let start = low.saturating_sub(props.overscan);

        // Find end index - find first item whose top edge is below the viewport bottom
        // An item is below viewport if offset >= scroll + viewport_height
        let viewport_bottom = scroll + viewport_height;
        let mut end = start;
        for i in start..count {
            let m = measurements_val.get(&i);
            let offset = m.map(|m| m.offset).unwrap_or(i as f64 * props.estimated_item_height);
            if offset >= viewport_bottom {
                break;
            }
            end = i + 1;
        }
        let end = (end + props.overscan).min(count);

        (start, end)
    });

    let (start_idx, end_idx) = visible_range();
    let measurements_val = measurements();

    // Setup ResizeObserver for dynamic item measurement
    // We need a unique ID for this virtual list instance to find its elements
    let list_id = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    });

    // Listen for resize events from JavaScript
    use_effect(move || {
        let observer_code = format!(
            r#"
            const listId = '{list_id}';
            const observer = new ResizeObserver((entries) => {{
                for (const entry of entries) {{
                    const index = parseInt(entry.target.getAttribute('data-index'));
                    const height = entry.borderBoxSize?.[0]?.blockSize || entry.contentRect.height;
                    if (!isNaN(index) && height > 0) {{
                        dioxus.send({{ index, height }});
                    }}
                }}
            }});
            
            // Find the virtual list container by its data attribute
            const container = document.querySelector('[data-virtual-list-id="{list_id}"]');
            if (!container) {{
                console.warn('Virtual list container not found');
                return;
            }}
            
            const observeItems = () => {{
                const items = container.querySelectorAll('[data-virtual-item]');
                items.forEach(item => {{
                    if (!item._observed) {{
                        observer.observe(item);
                        item._observed = true;
                    }}
                }});
            }};
            
            observeItems();
            const mutationObserver = new MutationObserver(observeItems);
            mutationObserver.observe(container, {{ childList: true, subtree: true }});
            
            await dioxus.recv();
            observer.disconnect();
            mutationObserver.disconnect();
            "#
        );
        
        let mut eval = document::eval(&observer_code);
        let mut heights = item_heights;

        spawn(async move {
            loop {
                match eval.recv::<serde_json::Value>().await {
                    Ok(value) => {
                        if let (Some(index), Some(height)) = (
                            value.get("index").and_then(|v| v.as_u64()),
                            value.get("height").and_then(|v| v.as_f64()),
                        ) {
                            heights.write().insert(index as usize, height);
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    });

    // Generate visible items - each item positioned absolutely at its own offset
    let items = (start_idx..end_idx).map(|i| {
        let item_content = props.item_content.clone();
        let item_offset = measurements_val
            .get(&i)
            .map(|m| m.offset)
            .unwrap_or(i as f64 * props.estimated_item_height);

        rsx! {
            div {
                key: "{i}",
                "data-index": i,
                "data-virtual-item": "true",
                style: "position: absolute; top: {item_offset}px; left: 0; right: 0;",
                {item_content.call(i)}
            }
        }
    });

    rsx! {
        div {
            "data-virtual-list-id": list_id,
            style: "overflow-y: auto; position: relative;",
            onscroll: move |e: Event<ScrollData>| {
                scroll_top.set(e.data.scroll_top());
            },
            onmounted: move |e: MountedEvent| {
                spawn(async move {
                    if let Ok(rect) = e.get_client_rect().await {
                        container_height.set(rect.size.height);
                    }
                });
            },
            ..props.attributes,

            // Spacer to create scrollable area - items are positioned absolutely within this
            div { style: "height: {total_height}px; position: relative;", {items} }
        }
    }
}
