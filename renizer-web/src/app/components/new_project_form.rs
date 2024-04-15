use leptos::*;
use leptos_router::FromFormData;
use crate::app::components::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct NewProjectData {
    pub name: String,
    pub description: String,
    pub location: String,
    pub start_date: String,
    #[serde(default)]
    pub org_restricted: bool
}

#[component]
pub fn NewProjectForm(
    m_p_user_id: String,
    #[prop(into)]
    on_close: Callback<ev::MouseEvent, ()>,
    set_on_close: WriteSignal<bool>
) -> impl IntoView {
    let (name, set_name) = create_signal("");
    let (description, set_description) = create_signal("");
    let (location, set_location) = create_signal("");
    let (start_date, set_start_date) = create_signal("");
    let (org_restricted, set_org_restricted) = create_signal(false);

    let new_form_action = Action::<AddNewProject, _>::server();

    let _ = create_local_resource(new_form_action.value(), move |res| async move {
        if let Some(Ok(_)) = res {
            set_name.update(|v| *v = "");
            set_description.update(|v| *v = "");
            set_location.update(|v| *v = "");
            set_start_date.update(|v| *v = "");
            set_org_restricted.update(|v| *v = false);

            set_on_close(true);
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
                                        <form 
                                            class="space-y-4"
                                            on:submit=move|ev| {
                                                ev.prevent_default();
                                                if let Ok(mut data) = NewProjectData::from_event(&ev) {
                                                    data.org_restricted = org_restricted();

                                                    new_form_action.dispatch(AddNewProject {
                                                        m_p_user_id: m_p_user_id.clone(),
                                                        p_data: data 
                                                    });
                                                }
                                            }    
                                        >

                                            <Input
                                                label="Name".into()
                                                value=name
                                                name="name".into()
                                            />

                                            <Input
                                                label="Description".into()
                                                value=description
                                                name="description".into()
                                            />

                                            <Input
                                                label="Location".into()
                                                value=location
                                                name="location".into()
                                            />

                                            <Input
                                                label="Start Date".into()
                                                value=start_date
                                                name="start_date".into()
                                                type_="date".into()
                                                min=chrono::Utc::now().format("%Y-%m-%d").to_string()
                                            />

                                            <CheckBox on_click=move|_| { set_org_restricted.update(|v| *v = !*v) }/>

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
                                                    Add New Project
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

#[server(AddNewProject)] 
pub async fn add_new_project(m_p_user_id: String, p_data: NewProjectData) -> Result<crate::tables::project::Project, ServerFnError> {
    use crate::tables::project;
    use chrono::*;

    let project = project::Project {
        project_id: uuid::Uuid::new_v4().to_string(),
        name: p_data.name,
        description: Some(p_data.description),
        start_date: Some(DateTime::from_naive_utc_and_offset(
            NaiveDate::parse_from_str(&p_data.start_date, "%Y-%m-%d")?.and_hms_opt(0, 0, 0).unwrap_or_default(), 
            Utc
        )),
        location: (!p_data.location.is_empty()).then_some(p_data.location),
        end_date: None,
        status: Some("On going".into()),
        energy_rate_kwh: None,
        produced_energy_kwh: None,
        energy_sold_kwh: None,
        org_restricted: p_data.org_restricted,
        total_cost: None,
        creation_date: Utc::now(),
        m_p_user_id: Some(m_p_user_id)
    };

    project::ssr::insert_project(project.clone()).await?;

    Ok(project)
}