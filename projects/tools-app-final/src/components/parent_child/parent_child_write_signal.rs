use leptos::*;


#[component]
pub fn Child(
    counter: ReadSignal<i32>,
    set_counter: WriteSignal<i32>,
) -> impl IntoView
{
    let increment = move |_| set_counter.update(|c| *c += 1);
    let decrement = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div style="border: 1px solid black;margin: 4px;">
            <h1>Write Signal Child</h1>
            <p>"Counter: " {counter}</p>
            <div>
                <button on:click=increment>"Child +1"</button>
                <button on:click=decrement>"Child -1"</button>
            </div>
        </div>
    }
}


#[component]
pub fn Parent() -> impl IntoView {

    let (counter, set_counter) = create_signal(0);

    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);

    view! {
        <div>
            <div style="border: 1px solid black;margin: 4px;">
                <h1>Write Signal Parent</h1>
                <p>"Counter: " {counter}</p>
                <div>
                    <button on:click=increment_counter>"Parent +1"</button>
                    <button on:click=decrement_counter>"Parent -1"</button>
                </div>
            </div>

            <Child counter=counter set_counter=set_counter />
        </div>
    }
}