use leptos::*;

use crate::models::car::Car;
use crate::repositories::car_repository::{all_cars, AppendCar, ReplaceCar, RemoveCar};
use crate::components::shared::tool_header::ToolHeader;
use crate::components::car_tool::car_table::CarTable;
use crate::components::car_tool::car_form::CarForm;

#[component]
pub fn CarHome() -> impl IntoView {

    let (edit_car_id, set_edit_car_id) = create_signal::<Option<String>>(None);

    let append_car = create_server_action::<AppendCar>();

    let car_resource = create_resource(
        move || append_car.version().get(),
        |_| all_cars(),
    );

    let edit_car = move |car_id: Option<String>| {
        set_edit_car_id.update(|edit_car_id| *edit_car_id = car_id);
    };

    let cancel_car = move |_| {
        set_edit_car_id.update(|edit_car_id| *edit_car_id = None);
    };

    let save_car =  move |car: Car| {
        let replace_car = create_server_action::<ReplaceCar>();
        replace_car.dispatch(ReplaceCar { car });
        set_edit_car_id.update(|edit_car_id| *edit_car_id = None);
        create_effect(move |_| {
            if replace_car.version().get() > 0 {
                car_resource.refetch();
            }
        });        
    };

    let delete_car = move |car_id: String| {
        let remove_car = create_server_action::<RemoveCar>();
        remove_car.dispatch(RemoveCar{ id: car_id });
        set_edit_car_id.update(|edit_car_id| *edit_car_id = None);
        create_effect(move |_| {
            if remove_car.version().get() > 0 {
                car_resource.refetch();
            }
        });       
    };


    view! {
        <ToolHeader header_text="Car Tool".to_string()/>
        <CarTable
            car_resource=car_resource
            edit_car_id=edit_car_id
            on_edit_car=edit_car
            on_delete_car=delete_car
            on_save_car=save_car
            on_cancel_car=cancel_car/>
        <CarForm append_car=append_car/>
    }

}
