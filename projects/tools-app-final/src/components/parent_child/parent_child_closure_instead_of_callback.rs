use ev::MouseEvent;
use leptos::*;


#[component]
pub fn Child<IncrementButton, DecrementButton>(
    counter: ReadSignal<i32>,
    on_increment: IncrementButton,
    on_decrement: DecrementButton,
) -> impl IntoView
where
    IncrementButton: Fn(MouseEvent) + 'static,
    DecrementButton: Fn(MouseEvent) + 'static,
{
    let increment = on_increment;
    let decrement = on_decrement;

    view! {
        <div style="border: 1px solid black;margin: 4px;">
            <h1>Closure instead of Callback Child</h1>
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
                <h1>Closure instead of Callback Parent</h1>
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