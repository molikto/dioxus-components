use dioxus::prelude::*;
use dioxus_primitives::virtual_list::{self, VirtualListProps};

#[component]
pub fn VirtualList<T: Clone + PartialEq + 'static>(props: VirtualListProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        virtual_list::VirtualList {
            data: props.data,
            item_content: props.item_content,
            height: props.height,
            estimated_item_height: props.estimated_item_height,
            overscan: props.overscan,
            attributes: props.attributes,
        }
    }
}
