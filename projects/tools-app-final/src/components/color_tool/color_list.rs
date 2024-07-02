use leptos::*;
use leptos_router::ActionForm;

use crate::components::errors_fallback::error_fallback;
use crate::models::color::Color;
use crate::repositories::color_repository::RemoveColor;

#[component]
fn ColorListItem(
    color: Color,
    remove_color: Action<RemoveColor, Result<(), ServerFnError>>,
) -> impl IntoView {
    // ActionForm is used to create a form that sends an action to the server
    view! {
        <li>
            <ActionForm action=remove_color>
                <input type="hidden" name="id" prop:value=color.id/>
                {color.name}
                -
                {color.hex_code}
                <button type="submit">X</button>
            </ActionForm>
        </li>
    }
}

#[component]
pub fn ColorList(
    color_resource: Resource<(usize, usize), Result<Vec<Color>, ServerFnError>>,
    remove_color: Action<RemoveColor, Result<(), ServerFnError>>,
) -> impl IntoView {
    let color_list_view = move || -> Option<Result<View, _>> {
        color_resource.and_then(|colors: &Vec<Color>| {
            colors
                .iter()
                .map(|color| {
                    view! { <ColorListItem color=color.clone() remove_color=remove_color/> }
                })
                // collect_view is used to convert an iterator of views into a single view
                .collect_view()
        })
    };

    // Transition is used to show a loading view while the resource is loading
    // ErrorBoundary is used to show a fallback view when an error occurs
    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=error_fallback()>
                <ul>{color_list_view}</ul>
            </ErrorBoundary>
        </Transition>
    }
}
