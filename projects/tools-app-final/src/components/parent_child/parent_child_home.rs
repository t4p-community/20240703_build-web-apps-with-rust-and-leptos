use leptos::*;
use leptos_router::*;

#[component]
pub fn ParentChildHome() -> impl IntoView {
    view! {
        <div>
            <h1>"Parent Child Home"</h1>
            <ul>
                <li><a href="/parent-child/write-signal">"Write Signal"</a></li>
                <li><a href="/parent-child/callback">"Callback"</a></li>
                <li><a href="/parent-child/closure-instead-of-callback">"Closure instead of Callback"</a></li>
            </ul>
            <Outlet/>
        </div>
    }
}