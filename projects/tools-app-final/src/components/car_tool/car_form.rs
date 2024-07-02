use leptos::*;
use leptos_router::ActionForm;

use crate::models::car::Car;
use crate::repositories::car_repository::AppendCar;

#[component]
pub fn CarForm(append_car: Action<AppendCar, Result<Car, ServerFnError>>) -> impl IntoView {

    view! {
        <ActionForm action={append_car}>
            <label>
                Make:
                <input type="text" name="make" />
            </label>
            <label>
                Model:
                <input type="text" name="model" />
            </label>
            <label>
                Year:
                <input type="number" name="year" />
            </label>
            <label>
                Color:
                <input type="text" name="color" />
            </label>
            <label>
                Price:
                <input type="number" name="price" />
            </label>
            <button type="submit">Add Car</button>
        </ActionForm>
    }
}
