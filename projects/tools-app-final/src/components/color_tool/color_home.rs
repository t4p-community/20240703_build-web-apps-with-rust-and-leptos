use leptos::*;

use crate::repositories::color_repository::{all_colors, AppendColor, RemoveColor};
use crate::components::shared::tool_header::ToolHeader;
use crate::components::color_tool::color_form::ColorForm;
use crate::components::color_tool::color_list::ColorList;

#[component]
pub fn ColorHome() -> impl IntoView {
    let append_color = create_server_action::<AppendColor>();
    let remove_color = create_server_action::<RemoveColor>();

    // move converts any variables captured by reference or mutable reference
    //   to variables captured by value.
    // if move is not used, then the closure borrows the variables,
    //   and they cannot be used again until the closure is destroyed.
    let color_resource = create_resource(
        move || (append_color.version().get(), remove_color.version().get()),
        |_| all_colors(),
    );

    view! {
        <ToolHeader header_text="Color Tool".to_string()/>
        <ColorList color_resource=color_resource remove_color=remove_color/>
        <ColorForm append_color=append_color/>
    }
}
