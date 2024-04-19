use leptos::*;
use leptos_router::FromFormData;
use crate::{app::components::Input, tables::{collaboration, collaboration_task}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct RoleFormData {
    pub role: String,
    pub task: String,
    pub start_date: String,
    pub expected_hour: String,
    pub expected_delivery_date: String
}

#[component]
pub fn AddRoleForm(
    p_user_id: String,
    project_id: String,
    set_show: WriteSignal<bool>
) -> impl IntoView {
    let (role, set_role) = create_signal("");
    let (task, set_task) = create_signal("");
    let (start_date, set_start_date) = create_signal("");
    let (expected_hour, set_expected_hour) = create_signal("");
    let (expected_delivery_date, set_expected_delivery_date) = create_signal("");

    let min_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let add_role_action = Action::<AddRole, _>::server();

    let _ = create_local_resource(add_role_action.value(), move|res| async move {
        if let Some(Ok(_)) = res {
            set_role.update(|v| *v = "");
            set_task.update(|v| *v = "");
            set_start_date.update(|v| *v = "");
            set_expected_hour.update(|v| *v = "");
            set_expected_delivery_date.update(|v| *v = "");
            set_show.update(|v| *v = false);
        }
    });

    view! {
        <section class="gradient-form h-screen bg-blueGray-50 py-1 fixed top-0 left-0 right-0 mx-auto z-50 backdrop-blur">
            <div class="container h-full p-10">
                <div class="flex h-full flex-wrap items-center justify-center text-neutral-800 dark:text-neutral-200">
                    <div class="w-1/3">
                        <div class="bg-dark border-2 border-gray-50 block rounded-lg shadow-lg">
                            <div
                                class="relative w-full max-w-full flex-1 flex-grow px-4 py-4 text-right"
                                on:click=move |_| { set_show.update(|v| *v = false) }
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
                                        <form
                                            class="space-y-4"
                                            on:submit=move |ev| {
                                                ev.prevent_default();
                                                let data = RoleFormData::from_event(&ev);
                                                if let Ok(role_form) = data {
                                                    add_role_action
                                                        .dispatch(AddRole {
                                                            p_user_id: p_user_id.clone(),
                                                            project_id: project_id.clone(),
                                                            role_form,
                                                        });
                                                }
                                            }
                                        >

                                            <Input label="Role".into() value=role name="role".into()/>

                                            <Input label="Task".into() value=task name="task".into()/>

                                            <Input
                                                label="Start Date".into()
                                                value=start_date
                                                name="start_date".into()
                                                type_="date".into()
                                                min=min_date.clone()
                                            />

                                            <Input
                                                label="Expected Delivery Date".into()
                                                value=expected_delivery_date
                                                name="expected_delivery_date".into()
                                                type_="date".into()
                                                min=min_date
                                            />

                                            <Input
                                                label="Expected Hour for Completion".into()
                                                value=expected_hour
                                                name="expected_hour".into()
                                            />

                                            <div class="mb-12 pb-1 pt-1 text-center">
                                                <button
                                                    class="shadow-dark-3 hover:shadow-dark-2 focus:shadow-dark-2 active:shadow-dark-2 dark:hover:shadow-dark-strong dark:focus:shadow-dark-strong dark:active:shadow-dark-strong mb-3 inline-block w-full rounded px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-white transition duration-150 ease-in-out focus:outline-none focus:ring-0 dark:shadow-black/30"
                                                    type="submit"
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

#[server(AddRole)]
pub async fn add_role(p_user_id: String, project_id: String, role_form: RoleFormData) -> Result<(collaboration::Collaboration, collaboration_task::CollaborationTask), ServerFnError> {    
    use chrono::{DateTime, NaiveDate, Utc};
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use std::str::FromStr;

    let str_to_datetime = |date_str| {
        let naive_dt = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?
            .and_hms_opt(0, 0, 0)
            .unwrap_or_default();
        Ok::<_, chrono::ParseError>(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc))
    };

    let collaboration = collaboration::Collaboration {
        p_user_id: p_user_id.clone(),
        project_id: project_id.clone(),
        start_date: str_to_datetime(&role_form.start_date)?,
        end_date: None,
        role: role_form.role
    };
    collaboration::ssr::insert_collaboration(collaboration.clone()).await?;

    let collaboration_task = collaboration_task::CollaborationTask {
        p_user_id,
        project_id,
        assigned_date: Utc::now(),
        task_name: role_form.task,
        start_date: str_to_datetime(&role_form.start_date)?,
        delivery_date: None,
        hour_taken: None,
        expected_day: Decimal::from_i64(str_to_datetime(&role_form.expected_delivery_date)?.signed_duration_since(Utc::now()).num_days()).unwrap_or_default(),
        expected_hour: Decimal::from_str(&role_form.expected_hour)?
    };
    collaboration_task::ssr::insert_collaboration_task(collaboration_task.clone()).await?;

    Ok((collaboration, collaboration_task))
}