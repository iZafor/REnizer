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
                                                        <button
                                                            on:click=move |_| { set_show_add_role(true) }
                                                            class="text-dark hover:text-light bg-light mt-4 inline-flex items-start justify-start rounded px-6 py-3 hover:bg-gray-500 focus:outline-none focus:ring-2 sm:mt-0"
                                                        >
                                                            <p class="text-sm font-medium leading-none">Add Role</p>
                                                        </button>
                                                    </div>
                                                    <div class="mt-7 overflow-x-auto">
                                                        <table class="w-full whitespace-nowrap">
                                                            <tbody>
                                                                <For
                                                                    each=roles
                                                                    key=move |(id, _)| id.clone()
                                                                    children=move |(_, rd)| view! { <RoleRow rd/> }
                                                                />
                                                            </tbody>
                                                        </table>
                                                    </div>
                                                    <Show when=show_add_role>
                                                        <AddRoleForm on_close=move |_| set_show_add_role(false)/>
                                                    </Show>
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