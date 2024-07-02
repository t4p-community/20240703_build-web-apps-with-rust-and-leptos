use ev::MouseEvent;
use leptos::*;


#[component]
pub fn Child(
    counter: ReadSignal<i32>,
    #[prop(into)] on_increment: Callback<MouseEvent>,
    #[prop(into)] on_decrement: Callback<MouseEvent>,
) -> impl IntoView
{
    let increment = on_increment;
    let decrement = on_decrement;

    view! {
        <div style="border: 1px solid black;margin: 4px;">
            <h1>Callback Child</h1>
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
                <h1>Callback Parent</h1>
                <p>"Counter: " {counter}</p>
                <div>
                    <button on:click=increment_counter>"Parent +1"</button>
                    <button on:click=decrement_counter>"Parent -1"</button>
                </div>
            </div>

            <Child counter=counter 
                on_increment=increment_counter
                on_decrement=decrement_counter />
        </div>
    }
}