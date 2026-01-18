use super::super::component::{
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
    DropdownMenuSub,
    DropdownMenuSubTrigger,
    DropdownMenuSubContent,
};
use dioxus::prelude::*;

#[derive(Clone, Copy, strum::Display, strum::EnumIter, PartialEq)]
enum Operation {
    Edit,
    Undo,
    Duplicate,
    Delete,
}

#[component]
pub fn Demo() -> Element {
    let mut selected_operation = use_signal(|| None);

    rsx! {
        DropdownMenu { class: "dropdown-menu", default_open: false,
            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }
            DropdownMenuContent { class: "dropdown-menu-content",
                DropdownMenuItem::<Operation> {
                    class: "dropdown-menu-item",
                    value: Operation::Edit,
                    index: 0usize,
                    on_select: move |value| {
                        selected_operation.set(Some(value));
                    },
                    "Edit"
                }

                DropdownMenuItem::<Operation> {
                    class: "dropdown-menu-item",
                    value: Operation::Undo,
                    index: 1usize,
                    disabled: true,
                    on_select: move |value| {
                        selected_operation.set(Some(value));
                    },
                    "Undo"
                }

                DropdownMenuSub { default_open: false,
                    DropdownMenuSubTrigger { class: "dropdown-menu-sub-trigger", "More" }
                    DropdownMenuSubContent { class: "dropdown-menu-sub-content",
                        DropdownMenuItem::<Operation> {
                            class: "dropdown-menu-item",
                            value: Operation::Duplicate,
                            index: 2usize,
                            on_select: move |value| {
                                selected_operation.set(Some(value));
                            },
                            "Duplicate"
                        }

                        DropdownMenuItem::<Operation> {
                            class: "dropdown-menu-item",
                            value: Operation::Delete,
                            index: 3usize,
                            on_select: move |value| {
                                selected_operation.set(Some(value));
                            },
                            "Delete"
                        }
                    }
                }
            }
        }

        if let Some(op) = selected_operation() {
            p { "Selected: {op}" }
        }
    }
}
