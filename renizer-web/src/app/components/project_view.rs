use leptos::*;
use leptos_router::{use_params_map, Redirect};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::app::components::TaskRow;

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
pub struct TaskRowData {
    pub p_user_id: String,
    pub project_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub role: String,
    pub n_tasks: i32,
    pub expected_hour: Decimal,
    pub expected_day: DateTime<Utc>,
}

#[component]
pub fn ProjectView() -> impl IntoView {
    let project_id = move || use_params_map().get().get("project-id").cloned().unwrap_or_default();

    let (task_data_vec, set_task_data_vec) = create_signal(vec![]); 

    let project = create_resource(
        project_id,
        |project_id| get_project_view_data(project_id)
    );

    let _ = create_local_resource(
        project_id,
        move |project_id| async move {
            match get_task_row_data(project_id).await {
                Err(err) => logging::log!("{err:?}"),
                Ok(res) => set_task_data_vec.update(|vals| {
                    vals.clear();
                    for td in res {
                        vals.push((Uuid::new_v4().to_string(), td));
                    }
                })
            }
        }  
    );

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
                                                                <svg
                                                                    width="20px"
                                                                    height="20px"
                                                                    class="svg-icon fill-gray-50"
                                                                    style="vertical-align: middle;fill: currentColor;overflow: hidden;"
                                                                    viewBox="0 0 1024 1024"
                                                                    version="1.1"
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                >
                                                                    <path d="M774.4 742.4c0-44.8-12.8-89.6-38.4-121.6C691.2 569.6 640 544 576 544c-38.4 0-76.8 0-115.2 0-44.8 0-83.2 12.8-121.6 38.4-44.8 32-70.4 76.8-76.8 128-6.4 57.6 0 121.6-6.4 185.6 6.4 0 19.2 6.4 25.6 6.4 76.8 19.2 147.2 38.4 224 38.4 44.8 0 96 0 140.8-6.4 38.4-6.4 76.8-19.2 115.2-32 6.4 0 6.4-6.4 6.4-12.8C774.4 838.4 774.4 787.2 774.4 742.4z"></path>
                                                                    <path d="M358.4 531.2c12.8-6.4 19.2-6.4 32-12.8-38.4-38.4-57.6-76.8-57.6-128-44.8 0-89.6 0-134.4 0C172.8 390.4 147.2 396.8 128 403.2 57.6 428.8 0 499.2 0 576c0 51.2 0 102.4 0 160 0 6.4 0 6.4 6.4 12.8 51.2 12.8 102.4 25.6 153.6 32 19.2 0 44.8 6.4 64 6.4 0-12.8 0-25.6 0-38.4 0-19.2 0-38.4 6.4-57.6C249.6 614.4 294.4 563.2 358.4 531.2z"></path>
                                                                    <path d="M1024 588.8c0-25.6-6.4-57.6-19.2-83.2-38.4-76.8-96-115.2-179.2-115.2-121.6 0 0 0-121.6 0 0 51.2-19.2 96-57.6 128 6.4 0 12.8 6.4 12.8 6.4 32 12.8 64 32 89.6 57.6 38.4 44.8 57.6 96 57.6 153.6 0 12.8 0 32 0 44.8 32-6.4 57.6-6.4 89.6-12.8 38.4-6.4 83.2-19.2 121.6-38.4 6.4 0 6.4-6.4 6.4-12.8C1024 684.8 1024 633.6 1024 588.8z"></path>
                                                                    <path d="M518.4 537.6c83.2 0 153.6-70.4 147.2-153.6 0-83.2-70.4-147.2-147.2-147.2-83.2 0-153.6 64-153.6 147.2C371.2 467.2 435.2 537.6 518.4 537.6z"></path>
                                                                    <path d="M704 371.2C723.2 377.6 742.4 384 768 384c83.2 0 153.6-70.4 147.2-153.6 0-83.2-70.4-147.2-147.2-147.2-83.2 0-153.6 64-153.6 147.2C665.6 256 697.6 307.2 704 371.2z"></path>
                                                                    <path d="M256 384c25.6 0 57.6-6.4 76.8-19.2 6.4-51.2 32-96 70.4-121.6 0 0 0-6.4 0-6.4 0-83.2-70.4-147.2-147.2-147.2-83.2 0-153.6 64-153.6 147.2C102.4 313.6 172.8 384 256 384z"></path>
                                                                </svg>
                                                                <p class="text-sm">{project.n_contributors}</p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <svg
                                                                    class="fill-gray-50"
                                                                    version="1.1"
                                                                    id="Layer_1"
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    xmlns:xlink="http://www.w3.org/1999/xlink"
                                                                    width="20px"
                                                                    height="20px"
                                                                    viewBox="0 0 256 252"
                                                                    xml:space="preserve"
                                                                >
                                                                    <path d="M63.3,10.8C79,0.5,91.9-0.5,106.1,7.2c18.9,10.3,25.8,34,15.5,52.9S88,85,68.6,75.7C43.7,63.8,31.4,38.2,30.7,19.3l0,0
                                                                    C42.3,22.1,53.3,17.3,63.3,10.8z M254,159.3c0-12.6-4.8-20.4-9-27.3c-9.3-15.2-29.1-40.1-38.9-52.7L232,41.1h-71.7l25.8,38.1
                                                                    c-9.8,12.6-29.7,37.6-39,52.8c-3.8,6.1-7.9,13-8.8,23.4h-22.5c0,0-22.1-51.3-22.5-52c-6.4-12.6-16.7-20.1-30.4-20.1
                                                                    c-20.2,0-35.4,17.5-42.4,36.5C7.9,153.3,1.3,215.4,2.1,249.8h102.3v-45.3h-1.1c-13.8,0-26.2-8.2-31.6-20.8l-14-32.4
                                                                    c-1.1-2.5,0.1-5.4,2.6-6.5c2.5-1.1,5.4,0.1,6.5,2.6l14,32.4c3.9,9,12.8,14.9,22.6,14.9H225C241.3,192.4,254,177.4,254,159.3z
                                                                    M198.9,164v7.1H191v-0.9h0v-6.5c-4.1-0.9-7.8-2.9-11.3-6.1l5.8-6.5c0,0,4.3,3.7,8.3,4.3c4.6,0.7,7.4-1.5,7.7-4.4
                                                                    c0.6-5.4-10-7.5-13.8-9.5c-4.6-2.4-9.1-5.8-9.1-12.8c0-7.2,5-12.4,12.5-13.1V109h0v-0.9h7.8v7.8c4.8,1.1,8.1,3.6,10.7,5.6l-5.3,6.9
                                                                    c-1.7-1.5-4.7-3.4-7.6-4.1c-3.7-0.9-7.7,0.1-8,4.2c-0.1,1.7,0.6,4.9,8.6,7.4c4.3,1.3,14.2,4.3,14.2,14.6
                                                                    C211.6,157.5,206.2,163.1,198.9,164z"></path>
                                                                </svg>
                                                                <p class="text-sm">{project.n_investors}</p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <svg
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    width="20"
                                                                    height="20"
                                                                    viewBox="0 0 20 20"
                                                                >
                                                                    <path
                                                                        class="stroke-gray-50"
                                                                        d="M7.5 5H16.6667"
                                                                        stroke="#52525B"
                                                                        stroke-width="1.65"
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                    ></path>
                                                                    <path
                                                                        class="stroke-gray-50"
                                                                        d="M7.5 10H16.6667"
                                                                        stroke="#52525B"
                                                                        stroke-width="1.65"
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                    ></path>
                                                                    <path
                                                                        class="stroke-gray-50"
                                                                        d="M7.5 15H16.6667"
                                                                        stroke="#52525B"
                                                                        stroke-width="1.65"
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                    ></path>
                                                                    <path
                                                                        class="stroke-gray-50"
                                                                        d="M4.16669 5V5.00667"
                                                                        stroke="#52525B"
                                                                        stroke-width="1.65"
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                    ></path>
                                                                    <path
                                                                        class="stroke-gray-50"
                                                                        d="M4.16669 10V10.0067"
                                                                        stroke="#52525B"
                                                                        stroke-width="1.65"
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                    ></path>
                                                                    <path
                                                                        class="stroke-gray-50"
                                                                        d="M4.16669 15V15.0067"
                                                                        stroke="#52525B"
                                                                        stroke-width="1.65"
                                                                        stroke-linecap="round"
                                                                        stroke-linejoin="round"
                                                                    ></path>
                                                                </svg>
                                                                <p class="text-sm">{project.n_tasks}</p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <svg
                                                                    width="20px"
                                                                    height="2opx"
                                                                    class="fill-gray-50"
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    data-name="Layer 1"
                                                                    viewBox="0 0 30 37.5"
                                                                    x="0px"
                                                                    y="0px"
                                                                >
                                                                    <path d="M19.357,7.093l1.592-4.776A1,1,0,0,0,20,1H10a1,1,0,0,0-.949,1.317l1.592,4.776A11.01,11.01,0,0,0,1,18v6a5.006,5.006,0,0,0,5,5H24a5.006,5.006,0,0,0,5-5V18A11.01,11.01,0,0,0,19.357,7.093ZM14,17h2a3,3,0,0,1,3,3v1a3,3,0,0,1-3,3v1a1,1,0,0,1-2,0V24a3,3,0,0,1-3-3,1,1,0,0,1,2,0,1,1,0,0,0,1,1h2a1,1,0,0,0,1-1V20a1,1,0,0,0-1-1H14a3,3,0,0,1-3-3V15a3,3,0,0,1,3-3V11a1,1,0,0,1,2,0v1a3,3,0,0,1,3,3,1,1,0,0,1-2,0,1,1,0,0,0-1-1H14a1,1,0,0,0-1,1v1A1,1,0,0,0,14,17Z"></path>
                                                                </svg>
                                                                <p class="text-sm">
                                                                    {project.total_cost.unwrap_or_default().to_string()}
                                                                </p>
                                                            </span>
                                                        </div>
                                                        <div class="flex items-center space-x-4">
                                                            <span class="flex items-center space-x-2">
                                                                <svg
                                                                    width="20px"
                                                                    height="20px"
                                                                    class="fill-gray-50"
                                                                    version="1.1"
                                                                    id="Layer_1"
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    xmlns:xlink="http://www.w3.org/1999/xlink"
                                                                    x="0px"
                                                                    y="0px"
                                                                    viewBox="0 0 110.01 122.88"
                                                                    style="enable-background:new 0 0 110.01 122.88"
                                                                    xml:space="preserve"
                                                                >
                                                                    <g>
                                                                        <path
                                                                            class="st0"
                                                                            d="M1.87,14.69h22.66L24.5,14.3V4.13C24.5,1.86,26.86,0,29.76,0c2.89,0,5.26,1.87,5.26,4.13V14.3l-0.03,0.39 h38.59l-0.03-0.39V4.13C73.55,1.86,75.91,0,78.8,0c2.89,0,5.26,1.87,5.26,4.13V14.3l-0.03,0.39h24.11c1.03,0,1.87,0.84,1.87,1.87 v19.46c0,1.03-0.84,1.87-1.87,1.87H1.87C0.84,37.88,0,37.04,0,36.01V16.55C0,15.52,0.84,14.69,1.87,14.69L1.87,14.69z M71.6,74.59 c2.68-0.02,4.85,2.14,4.85,4.82c-0.01,2.68-2.19,4.87-4.87,4.89l-11.76,0.08l-0.08,11.77c-0.02,2.66-2.21,4.81-4.89,4.81 c-2.68-0.01-4.84-2.17-4.81-4.83l0.08-11.69L38.4,84.54c-2.68,0.02-4.85-2.14-4.85-4.82c0.01-2.68,2.19-4.88,4.87-4.9l11.76-0.08 l0.08-11.77c0.02-2.66,2.21-4.82,4.89-4.81c2.68,0,4.83,2.16,4.81,4.82l-0.08,11.69L71.6,74.59L71.6,74.59L71.6,74.59z M0.47,42.19 h109.08c0.26,0,0.46,0.21,0.46,0.46l0,0v79.76c0,0.25-0.21,0.46-0.46,0.46l-109.08,0c-0.25,0-0.46-0.21-0.46-0.46V42.66 C0,42.4,0.21,42.19,0.47,42.19L0.47,42.19L0.47,42.19z M8.84,50.58h93.84c0.52,0,0.94,0.45,0.94,0.94v62.85 c0,0.49-0.45,0.94-0.94,0.94H8.39c-0.49,0-0.94-0.42-0.94-0.94v-62.4c0-1.03,0.84-1.86,1.86-1.86L8.84,50.58L8.84,50.58z M78.34,29.87c2.89,0,5.26-1.87,5.26-4.13V15.11l-0.03-0.41H73.11l-0.03,0.41v10.16c0,2.27,2.36,4.13,5.25,4.13L78.34,29.87 L78.34,29.87z M29.29,29.87c2.89,0,5.26-1.87,5.26-4.13V15.11l-0.03-0.41H24.06l-0.03,0.41v10.16c0,2.27,2.36,4.13,5.25,4.13V29.87 L29.29,29.87z"
                                                                        ></path>
                                                                    </g>
                                                                </svg>
                                                                <p class="text-sm">{project.creation_date.to_string()}</p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <svg
                                                                    width="20px"
                                                                    height="20px"
                                                                    class="fill-gray-50"
                                                                    version="1.1"
                                                                    id="Layer_1"
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    xmlns:xlink="http://www.w3.org/1999/xlink"
                                                                    x="0px"
                                                                    y="0px"
                                                                    viewBox="0 0 122.88 119.97"
                                                                    style="enable-background:new 0 0 122.88 119.97"
                                                                    xml:space="preserve"
                                                                >
                                                                    <g>
                                                                        <path d="M69.76,4.06c0-2.23,2.2-4.06,4.95-4.06s4.95,1.81,4.95,4.06V21.8c0,2.23-2.2,4.06-4.95,4.06s-4.95-1.81-4.95-4.06V4.06 L69.76,4.06L69.76,4.06z M14.37,78.05h11.34c0.72,0,1.31,0.59,1.31,1.31v8.38c0,0.72-0.59,1.31-1.31,1.31H14.37 c-0.72,0-1.31-0.59-1.31-1.31v-8.38C13.06,78.63,13.65,78.05,14.37,78.05L14.37,78.05z M57.79,54.17h11.34 c0.35,0,0.66,0.14,0.9,0.36c-4.45,2.78-8.31,6.4-11.37,10.65h-0.87c-0.72,0-1.31-0.59-1.31-1.31v-8.38 C56.48,54.76,57.07,54.17,57.79,54.17L57.79,54.17z M36.08,54.17h11.34c0.72,0,1.31,0.59,1.31,1.31v8.38 c0,0.72-0.59,1.31-1.31,1.31H36.08c-0.72,0-1.31-0.59-1.31-1.31v-8.38C34.77,54.76,35.36,54.17,36.08,54.17L36.08,54.17z M14.37,54.17h11.34c0.72,0,1.31,0.59,1.31,1.31v8.38c0,0.72-0.59,1.31-1.31,1.31H14.37c-0.72,0-1.31-0.59-1.31-1.31v-8.38 C13.06,54.76,13.65,54.17,14.37,54.17L14.37,54.17z M36.08,78.05h11.34c0.72,0,1.31,0.59,1.31,1.31v8.38 c0,0.72-0.59,1.31-1.31,1.31H36.08c-0.72,0-1.31-0.59-1.31-1.31v-8.38C34.77,78.63,35.36,78.05,36.08,78.05L36.08,78.05z M103.49,59.54c11.71,4.85,19.39,16.28,19.39,29.02c0,8.67-3.52,16.53-9.2,22.21c-5.68,5.68-13.53,9.2-22.21,9.2 c-8.67,0-16.52-3.52-22.21-9.2c-5.68-5.69-9.2-13.54-9.2-22.21c0-12.64,7.86-24.43,19.55-29.23 C86.86,56.37,96.29,56.55,103.49,59.54L103.49,59.54L103.49,59.54z M86.64,87.72c0.39-0.43,0.87-0.8,1.39-1.08V72.98 c0-1.39,1.13-2.52,2.52-2.52c1.39,0,2.53,1.13,2.53,2.52v13.66c0.92,0.5,1.68,1.25,2.17,2.17h9.76c1.4,0,2.52,1.13,2.52,2.52 s-1.13,2.52-2.52,2.52h-9.76c-0.9,1.68-2.66,2.82-4.7,2.82c-1.6,0-3.03-0.71-4.01-1.82C84.73,92.78,84.82,89.69,86.64,87.72 L86.64,87.72L86.64,87.72z M110.12,70.41c-13.01-13.01-34.95-9.33-42.56,7.05c-1.56,3.37-2.44,7.13-2.44,11.09 c0,7.28,2.95,13.87,7.72,18.64c4.77,4.77,11.36,7.72,18.64,7.72c7.28,0,13.87-2.96,18.64-7.72c4.77-4.77,7.72-11.36,7.72-18.64 c0-4.13-0.95-8.04-2.64-11.52C113.91,74.4,112.19,72.48,110.12,70.41L110.12,70.41L110.12,70.41z M25.33,4.06 c0-2.23,2.2-4.06,4.95-4.06c2.74,0,4.95,1.81,4.95,4.06V21.8c0,2.23-2.21,4.06-4.95,4.06c-2.74,0-4.95-1.81-4.95-4.06V4.06 L25.33,4.06L25.33,4.06z M5.45,38.79h94.21V18.37c0-0.7-0.28-1.31-0.73-1.76c-0.45-0.45-1.09-0.73-1.76-0.73h-9.03 c-1.51,0-2.74-1.23-2.74-2.74c0-1.51,1.23-2.74,2.74-2.74h9.03c2.21,0,4.2,0.89,5.65,2.34c1.45,1.45,2.34,3.44,2.34,5.65v32.43 c-1.8-0.62-3.65-1.12-5.56-1.49v-5.07h0.06H5.45v52.91c0,0.7,0.28,1.31,0.73,1.76c0.45,0.45,1.09,0.73,1.76,0.73h44.77 c0.51,1.9,1.15,3.76,1.92,5.54H7.99c-2.2,0-4.2-0.89-5.65-2.34C0.89,101.4,0,99.42,0,97.21V18.39c0-2.2,0.89-4.19,2.34-5.65 c1.45-1.45,3.44-2.34,5.65-2.34h9.64c1.51,0,2.74,1.23,2.74,2.74c0,1.51-1.23,2.74-2.74,2.74H7.99c-0.7,0-1.31,0.28-1.76,0.73 c-0.45,0.45-0.73,1.09-0.73,1.76v20.43H5.45V38.79L5.45,38.79z M43.13,15.87c-1.51,0-2.74-1.23-2.74-2.74 c0-1.51,1.23-2.74,2.74-2.74h18.39c1.51,0,2.74,1.23,2.74,2.74c0,1.51-1.23,2.74-2.74,2.74H43.13L43.13,15.87L43.13,15.87z"></path>
                                                                    </g>
                                                                </svg>
                                                                <p class="text-sm">
                                                                    {project.start_date.unwrap_or_default().to_string()}
                                                                </p>
                                                            </span>
                                                            <span class="flex items-center space-x-2">
                                                                <svg
                                                                    width="20px"
                                                                    height="18px"
                                                                    class="fill-gray-50"
                                                                    version="1.1"
                                                                    id="Layer_1"
                                                                    xmlns="http://www.w3.org/2000/svg"
                                                                    xmlns:xlink="http://www.w3.org/1999/xlink"
                                                                    x="0px"
                                                                    y="0px"
                                                                    viewBox="0 0 110.01 122.88"
                                                                    style="enable-background:new 0 0 110.01 122.88"
                                                                    xml:space="preserve"
                                                                >
                                                                    <g>
                                                                        <path
                                                                            class="st0"
                                                                            d="M1.87,14.69h22.66L24.5,14.3V4.13C24.5,1.86,26.86,0,29.76,0c2.89,0,5.26,1.87,5.26,4.13V14.3l-0.03,0.39 h38.59l-0.03-0.39V4.13C73.55,1.86,75.91,0,78.8,0c2.89,0,5.26,1.87,5.26,4.13V14.3l-0.03,0.39h24.11c1.03,0,1.87,0.84,1.87,1.87 v19.46c0,1.03-0.84,1.87-1.87,1.87H1.87C0.84,37.88,0,37.04,0,36.01V16.55C0,15.52,0.84,14.69,1.87,14.69L1.87,14.69z M31.35,83.53 c-2.27-1.97-2.52-5.41-0.55-7.69c1.97-2.28,5.41-2.53,7.69-0.56l12.45,10.8l20.31-20.04c2.13-2.12,5.59-2.11,7.71,0.02 c2.12,2.13,2.11,5.59-0.02,7.71L55.02,97.37c-2,1.99-5.24,2.14-7.41,0.26L31.35,83.53L31.35,83.53L31.35,83.53z M0.47,42.19h109.08 c0.26,0,0.46,0.21,0.46,0.47l0,0v79.76c0,0.25-0.21,0.46-0.46,0.46l-109.08,0c-0.25,0-0.46-0.21-0.46-0.46V42.66 C0,42.4,0.21,42.19,0.47,42.19L0.47,42.19L0.47,42.19z M8.84,50.58h93.84c0.52,0,0.94,0.45,0.94,0.94v62.85 c0,0.49-0.45,0.94-0.94,0.94H8.39c-0.49,0-0.94-0.42-0.94-0.94v-62.4c0-1.03,0.84-1.86,1.86-1.86L8.84,50.58L8.84,50.58z M78.34,29.87c2.89,0,5.26-1.87,5.26-4.13V15.11l-0.03-0.41l-10.45,0l-0.03,0.41v10.16c0,2.27,2.36,4.13,5.25,4.13L78.34,29.87 L78.34,29.87z M29.29,29.87c2.89,0,5.26-1.87,5.26-4.13V15.11l-0.03-0.41l-10.46,0l-0.03,0.41v10.16c0,2.27,2.36,4.13,5.25,4.13 V29.87L29.29,29.87z"
                                                                        ></path>
                                                                    </g>
                                                                </svg>
                                                                <p class="text-sm">
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
                                                        <button class="text-dark hover:text-light bg-light mt-4 inline-flex items-start justify-start rounded px-6 py-3 hover:bg-gray-500 focus:outline-none focus:ring-2 sm:mt-0">
                                                            <p class="text-sm font-medium leading-none">Add Task</p>
                                                        </button>
                                                    </div>
                                                    <div class="mt-7 overflow-x-auto">
                                                        <table class="w-full whitespace-nowrap">
                                                            <tbody>
                                                                <For
                                                                    each=task_data_vec
                                                                    key=move |(id, _)| id.clone()
                                                                    children=move |(_, td)| view! { <TaskRow td/> }
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
pub async fn get_task_row_data(project_id: String) -> Result<Vec<TaskRowData>, ServerFnError> {
    use crate::auth::ssr::pool;

    Ok(sqlx::query_as(&format!(r#"
        SELECT a.project_id, a.start_date, a.end_date, a.role, a.p_user_id, SUM(b.expected_hour) AS expected_hour, DATE_ADD(a.start_date, INTERVAL MAX(b.expected_day) DAY) AS expected_day, COUNT(b.task_name) AS n_tasks
        FROM Collaboration_T AS a INNER JOIN Collaboration_Task_T AS b ON a.p_user_id = b.p_user_id AND a.project_id = b.project_id
        WHERE a.project_id = "{project_id}"
        GROUP BY a.project_id, a.start_date, a.end_date, a.role, a.p_user_id
        ORDER BY a.start_date DESC
    "#))
        .fetch_all(&pool()?)
        .await?)
}