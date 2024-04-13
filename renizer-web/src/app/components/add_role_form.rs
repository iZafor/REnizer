use leptos::*;
use crate::{app::components::Input, tables::collaboration::Collaboration};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct RoleFormData {
    pub role: String,
    pub task: String,
    pub start_date: String,
    pub end_date: String,
    pub expected_hour: String,
    pub expected_delivery_date: String
}

#[component]
pub fn AddRoleForm(
    #[prop(into)]
    on_close: Callback<ev::MouseEvent>
) -> impl IntoView {
    let (role, _) = create_signal("");
    let (task, _) = create_signal("");
    let (start_date, _) = create_signal("");
    let (expected_hour, _) = create_signal("");
    let (expected_delivery_date, _) = create_signal("");

    view! {
        <section class="gradient-form h-screen bg-blueGray-50 py-1 fixed top-0 left-0 right-0 mx-auto z-50 backdrop-blur">
            <div class="container h-full p-10">
                <div class="flex h-full flex-wrap items-center justify-center text-neutral-800 dark:text-neutral-200">
                    <div class="w-1/3">
                        <div class="bg-dark border-2 border-gray-50 block rounded-lg shadow-lg">
                            <div
                                class="relative w-full max-w-full flex-1 flex-grow px-4 py-4 text-right"
                                on:click=on_close
                            >
                                <button
                                    class="mb-1 mr-1 rounded bg-red-500 px-3 py-1 text-xs font-bold uppercase outline-none transition-all duration-150 ease-linear focus:outline-none"
                                    type="button"
                                >
                                    X
                                </button>
                            </div>
                            <div class="g-0 lg:flex lg:flex-wrap">
                                <div class="grid w-full md:px-0">
                                    <div class="md:mx-6 md:p-12">
                                        <form class="space-y-4">

                                            <Input
                                                label="Role".into()
                                                value=role
                                                name="role_form[role]".into()
                                            />

                                            <Input
                                                label="Task".into()
                                                value=task
                                                name="role_form[task]".into()
                                            />

                                            <Input
                                                label="Start Date".into()
                                                value=start_date
                                                name="role_form[start_date]".into()
                                                type_="date".into()
                                            />

                                            <Input
                                                label="Expected Delivery Date".into()
                                                value=expected_delivery_date
                                                name="role_form[expected_delivery_date]".into()
                                                type_="date".into()
                                            />

                                            <Input
                                                label="Expected Hour for Completion".into()
                                                value=expected_hour
                                                name="role_form[expected_hour]".into()
                                            />

                                            <div class="mb-12 pb-1 pt-1 text-center">
                                                <button
                                                    class="shadow-dark-3 hover:shadow-dark-2 focus:shadow-dark-2 active:shadow-dark-2 dark:hover:shadow-dark-strong dark:focus:shadow-dark-strong dark:active:shadow-dark-strong mb-3 inline-block w-full rounded px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-white transition duration-150 ease-in-out focus:outline-none focus:ring-0 dark:shadow-black/30"
                                                    type="button"
                                                    data-twe-ripple-init
                                                    data-twe-ripple-color="light"
                                                    style="
                                                    background: linear-gradient(to right, #ee7724, #d8363a, #dd3675, #b44593);
                                                    "
                                                >
                                                    Add Role
                                                </button>

                                            </div>

                                        </form>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[server]
pub async fn add_role(_role_form: RoleFormData) -> Result<(), ServerFnError> {    
    Ok(())
}