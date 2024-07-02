use leptos::*;

// #[component] is a procedural macro that generates a component function.
// IntoView is a trait that allows a type to be converted into a view.
#[component]
pub fn ToolHeader(header_text: String) -> impl IntoView {
    // view! is a macro that generates a view from the given HTML-like syntax.
    view! {
        <header>
            <h1>{header_text}</h1>
        </header>
    }
}
