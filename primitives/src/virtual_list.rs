//! Virtual list component that efficiently renders large lists by only rendering visible items.
//!
//! Supports dynamically sized items by measuring their heights as they are rendered.

use dioxus::prelude::*;
use std::collections::HashMap;

/// Props for the VirtualList component
#[derive(Props, Clone, PartialEq)]
pub struct VirtualListProps<T: Clone + PartialEq + 'static> {
    /// The data items to render
    pub data: ReadSignal<Vec<T>>,

    /// Function that renders each item given its index and data
    pub item_content: Callback<(usize, T), Element>,

    /// Height of the container (e.g., "400px", "100%")
    #[props(default = "400px".to_string())]
    pub height: String,

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
///             data: users,
///             height: "100%".to_string(),
///             item_content: move |(index, user): (usize, User)| rsx! {
///                 div {
///                     style: "padding: 0.5rem; border-bottom: 1px solid #ccc;",
///                     p { strong { "{user.name}" } }
///                     div { "{user.description}" }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn VirtualList<T: Clone + PartialEq + 'static>(props: VirtualListProps<T>) -> Element {
    let mut scroll_top = use_signal(|| 0.0);
    let mut container_height = use_signal(|| 400.0);
    let item_heights = use_signal(|| HashMap::<usize, f64>::new());
    let mut item_offsets = use_signal(|| HashMap::<usize, f64>::new());

    let data = props.data;
    let item_count = use_memo(move || data().len());

    // Calculate total height and item offsets
    let measurements = use_memo(move || {
        let count = item_count();
        let heights = item_heights.read();
        let mut offset = 0.0;
        let mut measurements = HashMap::new();

        for i in 0..count {
            let height = heights.get(&i).copied().unwrap_or(props.estimated_item_height);
            measurements.insert(
                i,
                ItemMeasurement {
                    height,
                    offset,
                },
            );
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

        // Binary search for start index
        let mut start = 0;
        let mut end = count;
        while start < end {
            let mid = (start + end) / 2;
            let offset = measurements_val.get(&mid).map(|m| m.offset).unwrap_or(0.0);
            if offset < scroll {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        let start = start.saturating_sub(props.overscan);

        // Find end index
        let mut visible_end = start;
        let mut accumulated_height = 0.0;
        while visible_end < count && accumulated_height < viewport_height + (props.estimated_item_height * props.overscan as f64) {
            let height = measurements_val.get(&visible_end).map(|m| m.height).unwrap_or(props.estimated_item_height);
            accumulated_height += height;
            visible_end += 1;
        }
        let end = visible_end.min(count);

        (start, end)
    });

    let (start_idx, end_idx) = visible_range();
    let measurements_val = measurements();
    let offset_top = measurements_val.get(&start_idx).map(|m| m.offset).unwrap_or(0.0);

    // Generate visible items
    let items = (start_idx..end_idx).map(|i| {
        let item_data = data().get(i).cloned();
        
        if let Some(item) = item_data {
            let item_content = props.item_content.clone();
            let mut heights_signal = item_heights;
            
            rsx! {
                div {
                    key: "{i}",
                    "data-index": i,
                    onmounted: move |e: MountedEvent| {
                        spawn(async move {
                            if let Ok(rect) = e.get_client_rect().await {
                                heights_signal.write().insert(i, rect.size.height);
                            }
                        });
                    },
                    {item_content.call((i, item))}
                }
            }
        } else {
            rsx! {
                div { key: "{i}" }
            }
        }
    });

    rsx! {
        div {
            style: "height: {props.height}; overflow-y: auto; position: relative;",
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

            // Spacer to create scrollable area
            div { style: "height: {total_height}px; position: relative;",

                // Visible items container
                div { style: "position: absolute; top: {offset_top}px; left: 0; right: 0;",
                    {items}
                }
            }
        }
    }
}
