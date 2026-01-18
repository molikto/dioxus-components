//! Defines the [`DropdownMenu`] component and its subcomponents.

use std::rc::Rc;

use crate::{
    focus::{use_focus_controlled_item, use_focus_provider, FocusState},
    merge_attributes, use_animated_open, use_controlled, use_id_or, use_unique_id,
};
use dioxus::prelude::*;
use dioxus_attributes::attributes;

#[derive(Clone, Copy)]
struct DropdownMenuContext {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,

    // Focus state
    focus: FocusState,

    // Unique ID for the trigger button
    trigger_id: Signal<String>,
}

/// The props for the [`DropdownMenu`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuProps {
    /// Whether the dropdown menu is open. If not provided, the component will be uncontrolled and use `default_open`.
    pub open: ReadSignal<Option<bool>>,

    /// Default open state if the component is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when the open state changes. This is called when the dropdown menu is opened or closed.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the dropdown menu is disabled. If true, the menu will not open and items will not be selectable.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the dropdown menu element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dropdown menu, which should include a [`DropdownMenuTrigger`] and a [`DropdownMenuContent`].
    pub children: Element,
}

/// # DropdownMenu
///
/// The `DropdownMenu` component is a container for a [`DropdownMenuContent`] component activated by a [`DropdownMenuTrigger`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenu`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
/// - `data-disabled`: Indicates if the dropdown menu is disabled. values are `true` or `false`.
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let disabled = props.disabled;
    let trigger_id = use_unique_id();
    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| DropdownMenuContext {
        open,
        set_open,
        disabled,
        focus,
        trigger_id,
    });

    use_effect(move || {
        let focused = focus.any_focused();
        if *ctx.open.peek() != focused {
            (ctx.set_open)(focused);
        }
    });

    // Handle escape key to close the menu
    let handle_keydown = move |event: Event<KeyboardData>| {
        if disabled() {
            return;
        }
        match event.key() {
            Key::Enter => {
                let new_open = !(ctx.open)();
                ctx.set_open.call(new_open);
            }
            Key::Escape => ctx.set_open.call(false),
            Key::ArrowDown => {
                ctx.focus.focus_next();
            }
            Key::ArrowUp => {
                if open() {
                    ctx.focus.focus_prev();
                }
            }
            Key::Home => ctx.focus.focus_first(),
            Key::End => ctx.focus.focus_last(),
            _ => return,
        }
        event.prevent_default();
    };

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            onkeydown: handle_keydown,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownMenuTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuTriggerProps {
    /// Render the trigger element as a custom component/element.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the trigger
    pub children: Element,
}

/// # DropdownMenuTrigger
///
/// The trigger button for the parent [`DropdownMenu`]. This button toggles the visibility of the [`DropdownMenuContent`].
///
/// This must be used inside a [`DropdownMenu`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenuTrigger`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
/// - `data-disabled`: Indicates if the dropdown menu is disabled. values are `true` or `false`.
#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let mut ctx: DropdownMenuContext = use_context();
    let mut element = use_signal(|| None::<Rc<MountedData>>);

    let open = ctx.open;
    let disabled = ctx.disabled;
    let data_state = if open() { "open" } else { "closed" };

    let base = attributes!(button {
        id: ctx.trigger_id,
        r#type: "button",
        "data-state": data_state,
        "data-disabled": disabled,
        disabled: disabled,
        aria_expanded: open,
        aria_haspopup: "listbox",
        onmounted: move |e: MountedEvent| {
            element.set(Some(e.data()));
        },
        onclick: move |_| {
            if disabled() {
                return;
            }

            let new_open = !open();
            ctx.set_open.call(new_open);

            // Focus the element on click. Safari does not do this automatically.
            // https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#clicking_and_focus
            if let Some(data) = element() {
                spawn(async move {
                    _ = data.set_focus(true).await;
                });
            }
        },
        onblur: move |_| {
            if !ctx.focus.any_focused() {
                ctx.focus.blur();
            }
        },
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged,{props.children} }
        }
    }
}

/// The props for the [`DropdownMenuContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuContentProps {
    /// The ID of the dropdown menu content element. If not provided, a unique ID will be generated.
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes to apply to the dropdown menu content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dropdown menu content, which should include one or more [`DropdownMenuItem`] components.
    pub children: Element,
}

/// # DropdownMenuTrigger
///
/// The contents of a [`DropdownMenu`]. The component will only be rendered when the parent [`DropdownMenu`] is open (as control by the [`DropdownMenuTrigger`]).
///
/// This must be used inside a [`DropdownMenu`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenuContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let ctx: DropdownMenuContext = use_context();

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);
    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                aria_labelledby: "{ctx.trigger_id}",
                "data-state": if (ctx.open)() { "open" } else { "closed" },
                onpointerdown: move |event| {
                    // The user is starting a click inside the dropdown menu.
                    // Prevent the blur event from occurring during pointerdown,
                    // to keep the dropdown menu open until pointerup happens,
                    // thus enabling onclick/onselect events to fire.
                    event.prevent_default();
                    event.stop_propagation();
                },
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`DropdownMenuItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuItemProps<T: Clone + PartialEq + 'static> {
    /// The value of the item, which will be passed to the `on_select` callback when clicked.
    pub value: ReadSignal<T>,
    /// The index of the item within the [`DropdownMenuContent`]. This is used to order the items for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the item is disabled. If true, the item will not be clickable and will not respond to keyboard events.
    /// Defaults to false.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The callback function that will be called when the item is selected. The value of the item will be passed as an argument.
    #[props(default)]
    pub on_select: Callback<T>,

    /// Additional attributes to apply to the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the item, which will be rendered inside the item element.
    pub children: Element,
}

/// # DropdownMenuTrigger
///
/// An item within a [`DropdownMenuContent`]. This component represents an individual selectable item in the dropdown menu.
///
/// This must be used inside a [`DropdownMenu`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         DropdownMenu { default_open: false,
///             DropdownMenuTrigger { "Open Menu" }
///             DropdownMenuContent {
///                 DropdownMenuItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownMenuItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownMenuItem`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates whether the item is disabled. Values are `true` or `false`.
#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element {
    let mut ctx: DropdownMenuContext = use_context();

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || ctx.focus.is_focused((props.index)());

    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        div {
            role: "option",
            "data-disabled": disabled(),
            tabindex: if focused() { "0" } else { "-1" },

            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                if !disabled() {
                    props.on_select.call((props.value)());
                    ctx.set_open.call(false);
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                    if !disabled() {
                        props.on_select.call((props.value)());
                        ctx.set_open.call(false);
                    }
                    event.prevent_default();
                    event.stop_propagation();
                }
            },

            onmounted,

            onblur: move |_| {
                if focused() {
                    ctx.focus.blur();
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, Copy)]
struct DropdownSubContext {
    open: Memo<bool>,
    set_open: Callback<bool>,
    trigger_id: Signal<String>,
}

/// Props for a submenu wrapper. Wrap a `DropdownMenuSubTrigger` and `DropdownMenuSubContent`.
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSubProps {
    /// Whether the submenu is open (controlled)
    pub open: ReadSignal<Option<bool>>,

    /// Default open state if uncontrolled
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Additional attributes applied to the submenu container
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (should include a `DropdownMenuSubTrigger` and `DropdownMenuSubContent`)
    pub children: Element,
}

/// A container for a submenu. Provides submenu open state to its children via context.
#[component]
pub fn DropdownMenuSub(props: DropdownMenuSubProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let trigger_id = use_unique_id();

    use_context_provider(|| DropdownSubContext { open, set_open, trigger_id });

    let set_open_clone = set_open.clone();

    rsx! {
        div {
            class: "dropdown-sub",
            "data-state": if (open)() { "open" } else { "closed" },
            onmouseenter: move |_| {
                set_open_clone.call(true);
            },
            onmouseleave: move |_| {
                set_open_clone.call(false);
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for the submenu trigger element
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSubTriggerProps {
    /// Render the trigger element as a custom component/element.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children of the trigger
    pub children: Element,
}

/// The trigger for a submenu. Toggles the submenu open state.
#[component]
pub fn DropdownMenuSubTrigger(props: DropdownMenuSubTriggerProps) -> Element {
    let ctx: DropdownSubContext = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;

    let base = attributes!(div {
        role: "menuitem",
        "data-state": if open() { "open" } else { "closed" },
        "aria-expanded": open,
        "aria-haspopup": "menu",
        onclick: move |e: Event<MouseData>| {
            e.stop_propagation();
            let new = !open();
            set_open.call(new);
        },
    });

    let merged = merge_attributes(vec![base, props.attributes]);

    if let Some(dynamic) = props.r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            div { ..merged,{props.children} }
        }
    }
}

/// Props for the submenu content element
#[derive(Props, Clone, PartialEq)]
pub struct DropdownMenuSubContentProps {
    /// The ID for the submenu content element
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes to apply to the submenu content element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children (menu items)
    pub children: Element,
}

/// The contents of a submenu. Rendered when the parent `DropdownMenuSub` is open.
#[component]
pub fn DropdownMenuSubContent(props: DropdownMenuSubContentProps) -> Element {
    let ctx: DropdownSubContext = use_context();

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);
    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                "data-state": if (ctx.open)() { "open" } else { "closed" },
                onpointerdown: move |event| {
                    event.prevent_default();
                    event.stop_propagation();
                },
                ..props.attributes,
                {props.children}
            }
        }
    }
}
