use leptos::*;
use leptos_router::*;

#[component]
pub fn ParentChildHome() -> impl IntoView {
    view! {
        <div>
            <h2>"Parent Child Home"</h2>
            <ul>
                <li>
                    <a href="/parent-child/write-signal">"Write Signal"</a>
                </li>
                <li>
                    <a href="/parent-child/callback">"Callback"</a>
                </li>
            </ul>
            <Outlet/>
        </div>
    }
}
