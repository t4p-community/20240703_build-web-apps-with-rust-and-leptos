use ev::MouseEvent;
use leptos::*;

use crate::components::errors_fallback::error_fallback;
use crate::models::car::Car;

#[component]
fn CarViewRow(
    car: Car,
    #[prop(into)] on_edit_car: Callback<Option<String>>,
    #[prop(into)] on_delete_car: Callback<String>,
) -> impl IntoView {

    let car_id = car.id.to_string();
    let edit_car = move |_| {
        leptos::Callable::call(&on_edit_car, Some(car_id.to_string()));
    };
    
    let car_id = car.id.to_string();
    let delete_car = move |_| {
        leptos::Callable::call(&on_delete_car, car_id.to_string());
    };


    view! {
        <tr>
            <td>{car.id.to_string()}</td>
            <td>{car.make}</td>
            <td>{car.model}</td>
            <td>{car.year}</td>
            <td>{car.color}</td>
            <td>{car.price}</td>
            <td>
                <button type="button" on:click=edit_car>Edit</button>
                <button type="button" on:click=delete_car>Delete</button>
            </td>
        </tr>
    }
}

#[component]
fn CarEditRow(
    car: Car,
    #[prop(into)] on_save_car: Callback<Car>,
    #[prop(into)] on_cancel_car: Callback<MouseEvent>,
) -> impl IntoView {

    let (make, set_make) = create_signal(car.make.clone());
    let (model, set_model) = create_signal(car.model.clone());
    let (year, set_year) = create_signal(car.year.to_string());
    let (color, set_color) = create_signal(car.color.clone());
    let (price, set_price) = create_signal(car.price.to_string());

    let car_id = car.id.to_string();

    let save_car = move |_| {
        leptos::Callable::call(&on_save_car, Car {
            id: car_id.to_string(),
            make: make.get(),
            model: model.get(),
            year: year.get().parse().unwrap(),
            color: color.get(),
            price: price.get().parse().unwrap(),
        });
    };

    view! {
        <tr>
            <td>{car.id.to_string()}</td>
            <td><input type="text" name="make" prop:value=make
                on:input=move |ev| { set_make(event_target_value(&ev))} /></td>
            <td><input type="text" name="model" prop:value=model
                on:input=move |ev| { set_model(event_target_value(&ev))} /></td>
            <td><input type="number" name="year" prop:value=year
                on:input=move |ev| { set_year(event_target_value(&ev))} /></td>
            <td><input type="text" name="color" prop:value=color
                on:input=move |ev| { set_color(event_target_value(&ev))} /></td>
            <td><input type="number" name="price" prop:value=price
                on:input=move |ev| { set_price(event_target_value(&ev))} /></td>
            <td>
                <button type="button" on:click=save_car>Save</button>
                <button type="button" on:click=on_cancel_car>Cancel</button>
            </td>
        </tr>
    }
}


#[component]
pub fn CarTable(
    car_resource: Resource<usize, Result<Vec<Car>, ServerFnError>>,
    edit_car_id: ReadSignal<Option<String>>,
    #[prop(into)] on_edit_car: Callback<Option<String>>,
    #[prop(into)] on_delete_car: Callback<String>,
    #[prop(into)] on_save_car: Callback<Car>,
    #[prop(into)] on_cancel_car: Callback<MouseEvent>,
) -> impl IntoView {
    
    let car_rows = move || -> Option<Result<View, _>> {
        car_resource.and_then(|cars: &Vec<Car>| {
            cars
                .iter()
                .map(|car| {
                    if edit_car_id.get().map(|id| id == car.id.to_string()).unwrap_or(false) {
                        view! { <CarEditRow car=car.clone()
                            on_save_car=on_save_car
                            on_cancel_car=on_cancel_car/> }
                    } else {
                        view! { <CarViewRow car=car.clone()
                            on_edit_car=on_edit_car
                            on_delete_car=on_delete_car/> }
                    }
                })
                .collect_view()
        })
    };

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=error_fallback()>
                <div>Edit Car Id: {move || edit_car_id.get().unwrap_or_else(|| "None".to_string())}</div>
                <table>
                    <thead>
                        <tr>
                            <th>Id</th>
                            <th>Make</th>
                            <th>Model</th>
                            <th>Year</th>
                            <th>Color</th>
                            <th>Price</th>
                        </tr>
                    </thead>
                    <tbody>
                        {car_rows}
                    </tbody>
                </table>
            </ErrorBoundary>
        </Transition>
    }
}
