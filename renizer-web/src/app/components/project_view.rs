use leptos::*;
use leptos_router::{use_params_map, Redirect};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::{app::components::*, tables::collaboration_task::CollaborationTask};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProjectViewData {
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub energy_rate_kwh: Option<Decimal>,
    pub produced_energy_kwh: Option<Decimal>,
    pub energy_sold_kwh: Option<Decimal>,
    pub total_cost: Option<Decimal>,
    pub org_restricted: bool,
    pub m_p_user_id: Option<String>,
    pub creation_date: DateTime<Utc>,
    pub n_contributors: Option<i32>,
    pub n_investors: Option<i32>,
    pub n_tasks: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct RoleRowData {
    pub p_user_id: String,
    pub project_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub role: String,
    pub hourly_rate: Decimal,
    pub n_tasks: i32,
    pub expected_hour: Decimal,
    pub expected_day: DateTime<Utc>,
}

#[component]
pub fn ProjectView() -> impl IntoView {
    let project_id = move || use_params_map().get().get("project-id").cloned().unwrap_or_default();

    let (roles, set_roles) = create_signal(vec![]); 
    let (tasks, set_tasks) = create_signal(vec![]);
    let (show_add_role, set_show_add_role) = create_signal(false);
    let (on_focus, set_on_focus) = create_signal(false);
    let (s_text, set_s_text) = create_signal("".to_string());

    let filtered_signal = move || roles().iter().filter(|(_, val): &&(String, RoleRowData)| {
        val.role.to_lowercase().contains(&s_text().to_lowercase())
    }).map(|val| val.to_owned()).collect::<Vec<_>>();

    let project = create_resource(
        project_id,
        |project_id| get_project_view_data(project_id)
    );

    let _ = create_local_resource(
        project_id,
        move |project_id| async move {
            match get_role_row_data(project_id).await {
                Err(err) => logging::log!("{err:?}"),
                Ok(res) => set_roles.update(|vals| {
                    vals.clear();
                    for td in res {
                        vals.push((Uuid::new_v4().to_string(), td));
                    }
                })
            }
        }  
    );

    let _ = create_local_resource(
        project_id,
        move |project_id| async move {
            match get_tasks(project_id).await {
                Err(err) => logging::log!("{err:?}"),
                Ok(res) => set_tasks.update(|vals| {
                    vals.clear();
                    for td in res {
                        vals.push((Uuid::new_v4().to_string(), td));
                    }
                })
            }
        }  
    );

    provide_context(tasks);

    view! {
        <Show
            when=move || !project_id().is_empty()
            fallback=|| {
                view! { <Redirect path="/"/> }
            }
        >

            <Transition fallback=move || {
                view! { <p>Loading...</p> }
            }>
                {move || {
                    project
                        .get()
                        .map(|project| match project {
                            Err(_) => view! { <Redirect path="/"/> }.into_view(),
                            Ok(project) => {
                                match project {
                                    None => view! { <Redirect path="/"/> }.into_view(),
                                    Some(project) => {
                                        view! {
                                            <div class="w-full h-screen p-6 bg-gray-700">
                                                <div class="bg-dark rounded-md px-4 py-4 md:px-10 md:py-7 xl:px-10">
                                                    <div class="text-light mb-4 space-y-1.5">
                                                        <p class="text-4xl font-bold">{project.name}</p>
                                                        <p class="text-gray-400">{project.description}</p>
                                                        <div class="flex items-center space-x-4">
                                                            <span class="flex items-center space-x-2">
                                                                <p>Contributors</p>
                                                                <p class="text-sm bg-gray-500 px-2 rounded-md">
                                                                    {project.n_contributors}
                                                                </p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <p>Investors</p>
                                                                <p class="text-sm bg-gray-500 px-2 rounded-md">
                                                                    {project.n_investors}
                                                                </p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <p>In-total Tasks</p>
                                                                <p class="text-sm bg-gray-500 px-2 rounded-md">
                                                                    {project.n_tasks}
                                                                </p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <p>Project Cost</p>
                                                                <p class="text-sm bg-gray-500 px-2 rounded-md">
                                                                    {project.total_cost.unwrap_or_default().to_string()}
                                                                </p>
                                                            </span>
                                                        </div>
                                                        <div class="flex items-center space-x-4">
                                                            <span class="flex items-center space-x-2">
                                                                <p>Project Created at</p>
                                                                <p class="text-sm bg-gray-500 px-2 rounded-md">
                                                                    {project.creation_date.to_string()}
                                                                </p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <p>Project Started at</p>
                                                                <p class="text-sm bg-gray-500 px-2 rounded-md">
                                                                    {project.start_date.unwrap_or_default().to_string()}
                                                                </p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <p>Project Ended at</p>
                                                                <p class="text-sm bg-gray-500 px-2 rounded-md">
                                                                    {project.end_date.unwrap_or_default().to_string()}
                                                                </p>
                                                            </span>
                                                        </div>
                                                    </div>
                                                    <div class="items-center justify-between sm:flex">
                                                        <div class="flex items-center">
                                                            <a class="cursor-pointer rounded-full focus:outline-none focus:ring-2">
                                                                <div class="text-light rounded-full bg-gray-500 px-8 py-2">
                                                                    <p>All</p>
                                                                </div>
                                                            </a>
                                                            <a class="ml-4 cursor-pointer rounded-full focus:outline-none focus:ring-2 sm:ml-8">
                                                                <div class="text-light hover:text-light rounded-full px-8 py-2 hover:bg-gray-500">
                                                                    <p>Done</p>
                                                                </div>
                                                            </a>
                                                            <a class="ml-4 cursor-pointer rounded-full focus:bg-indigo-50 focus:outline-none focus:ring-2 sm:ml-8">
                                                                <div class="text-light hover:text-light rounded-full px-8 py-2 hover:bg-gray-500">
                                                                    <p>Pending</p>
                                                                </div>
                                                            </a>
                                                        </div>

                                                        <span class="space-x-4 flex">
                                                            <div
                                                                class="ml-auto w-[260px] flex items-center p-1 px-3 rounded-md"
                                                                class=("bg-gray-50", on_focus)
                                                                class=("bg-gray-700", move || on_focus() == false)
                                                            >
                                                                <svg
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    class=("fill-gray-700", on_focus)
                                                                    class=("fill-gray-50", move || on_focus() == false)
                                                                    height="24px"
                                                                    width="24px"
                                                                    viewBox="0 0 512 512"
                                                                >
                                                                    <path d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"></path>
                                                                </svg>
                                                                <input
                                                                    class="flex bg-gray-700 focus:bg-gray-50 rounded-md border-input bg-background px-3 py-3 ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground outline-none disabled:cursor-not-allowed disabled:opacity-50 w-full h-8 text-sm border-0 shadow-none resize-none pl-4"
                                                                    on:focus=move |_| set_on_focus.update(|v| *v = true)
                                                                    on:blur=move |_| set_on_focus.update(|v| *v = false)
                                                                    placeholder="Search Role..."
                                                                    type="search"
                                                                    on:input=move |ev| {
                                                                        let _ = gloo_timers::callback::Timeout::new(
                                                                            500,
                                                                            move || {},
                                                                        );
                                                                        set_s_text(event_target_value(&ev));
                                                                    }
                                                                />

                                                            </div>

                                                            <button
                                                                on:click=move |_| { set_show_add_role(true) }
                                                                class="text-dark hover:text-light bg-light mt-4 inline-flex items-start justify-start rounded px-6 py-3 hover:bg-gray-500 focus:outline-none focus:ring-2 sm:mt-0"
                                                            >
                                                                <p class="text-sm font-medium leading-none">Add Role</p>
                                                            </button>

                                                        </span>
                                                    </div>
                                                    <div class="mt-7 overflow-x-auto">
                                                        <table class="w-full whitespace-nowrap">
                                                            <tbody>
                                                                <For
                                                                    each=filtered_signal
                                                                    key=move |(id, _)| id.clone()
                                                                    children=move |(_, rd)| {
                                                                        view! {
                                                                            <RoleRow rd=rd.clone()/>
                                                                            <Show when=show_add_role>
                                                                                <AddRoleForm
                                                                                    p_user_id=rd.p_user_id.clone()
                                                                                    project_id=project_id()
                                                                                    set_show=set_show_add_role
                                                                                />
                                                                            </Show>
                                                                        }
                                                                    }
                                                                />

                                                            </tbody>
                                                        </table>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                            .into_view()
                                    }
                                }
                            }
                        })
                }}

            </Transition>
        </Show>
    }
}

#[server]
pub async fn get_project_view_data(project_id: String) -> Result<Option<ProjectViewData>, ServerFnError> {
    use crate::auth::ssr::pool;
    Ok(sqlx::query_as(&format!(r#"
    SELECT a.name, a.description, a.location, a.start_date, a.end_date, a.status, a.energy_rate_kwh, a.produced_energy_kwh, a.energy_sold_kwh, a.total_cost, a.m_p_user_id, a.creation_date, a.org_restricted, COUNT(b.p_user_id) AS n_contributors, COUNT(c.i_user_id) AS n_investors, COUNT(d.task_name) AS n_tasks
    FROM Project_T AS a INNER JOIN Collaboration_T AS b ON a.project_id = b.project_id
        INNER JOIN Investor_Invest_Project_T AS c ON a.project_id = c.project_id
        INNER JOIN Collaboration_Task_T AS d ON a.project_id = d.project_id
    WHERE a.project_id = "{project_id}"
    GROUP BY a.project_id 
    "#))
    .fetch_optional(&pool()?)
    .await?)
}

#[server]
pub async fn get_role_row_data(project_id: String) -> Result<Vec<RoleRowData>, ServerFnError> {
    use crate::auth::ssr::pool;

    Ok(sqlx::query_as(&format!(r#"
        SELECT a.project_id, a.start_date, a.end_date, a.role, a.p_user_id, SUM(b.expected_hour) AS expected_hour, DATE_ADD(a.start_date, INTERVAL MAX(b.expected_day) DAY) AS expected_day, COUNT(b.task_name) AS n_tasks, c.hourly_rate
        FROM Collaboration_T AS a INNER JOIN Collaboration_Task_T AS b ON a.p_user_id = b.p_user_id AND a.project_id = b.project_id INNER JOIN Project_Associate_T AS c ON a.p_user_id = c.p_user_id
        WHERE a.project_id = "{project_id}"
        GROUP BY a.project_id, a.start_date, a.end_date, a.role, a.p_user_id
        ORDER BY a.start_date DESC
    "#))
        .fetch_all(&pool()?)
        .await?)
}

#[server]
pub async fn get_tasks(project_id: String) -> Result<Vec<CollaborationTask>, ServerFnError> {
    crate::tables::collaboration_task::ssr::get_matching_collaboration_tasks(format!("project_id = \"{project_id}\"")).await   
}