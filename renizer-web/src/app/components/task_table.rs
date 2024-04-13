use leptos::*;
use crate::tables::collaboration_task::CollaborationTask;

#[component]
pub fn TaskTable(
    #[prop(into)]
    on_close: Callback<ev::MouseEvent>,
    p_user_id: String,
    role: String
) -> impl IntoView {
    let tasks = use_context::<ReadSignal<Vec<(String, CollaborationTask)>>>().unwrap();

    let tasks = move || tasks
        .with(|tasks| 
            tasks
            .into_iter()
            .filter(|task| task.1.p_user_id == p_user_id)
            .map(|task| task.to_owned())
            .collect::<Vec<_>>()
        );

    view! {
        <section class="bg-blueGray-50 py-1 fixed top-0 left-0 right-0 mx-auto z-50 backdrop-blur h-screen">
            <div class="mx-auto mt-24 w-full px-[200px]">
                <div class="bg-dark text-light relative mb-6 flex w-full min-w-0 flex-col break-words rounded shadow-lg border-2 border-gray-50">
                <div class="mb-0 rounded-t border-0 px-4 py-6">
                    <div class="flex flex-wrap items-center">
                    <div class="relative w-full max-w-full flex-1 flex-grow px-2">
                        <h3 class="text-blueGray-700 text-base font-semibold">{role}</h3>
                    </div>
                    <div 
                        class="relative w-full max-w-full flex-1 flex-grow px-4 text-right"
                        on:click=on_close
                    >
                        <button class="mb-1 mr-1 rounded bg-red-500 px-3 py-1 text-xs font-bold uppercase outline-none transition-all duration-150 ease-linear focus:outline-none" type="button">X</button>
                    </div>
                    </div>
                </div>

                <div class="block w-full overflow-x-auto">
                    <table class="text-blueGray-700 w-full border-collapse items-center">
                    <thead class="thead-light">
                        <tr>
                        <th class="bg-blueGray-50 text-blueGray-500 border-blueGray-100 whitespace-nowrap border border-l-0 border-r-0 border-solid px-6 py-3 text-center align-middle text-xs font-semibold uppercase">Task</th>
                        <th class="bg-blueGray-50 text-blueGray-500 border-blueGray-100 whitespace-nowrap border border-l-0 border-r-0 border-solid px-6 py-3 text-center align-middle text-xs font-semibold uppercase">Assigned at</th>
                        <th class="bg-blueGray-50 text-blueGray-700 border-blueGray-100 min-w-140-px whitespace-nowrap border border-l-0 border-r-0 border-solid px-6 py-3 text-center align-middle text-xs font-semibold uppercase">Strat Date</th>
                        <th class="bg-blueGray-50 text-blueGray-700 border-blueGray-100 min-w-140-px whitespace-nowrap border border-l-0 border-r-0 border-solid px-6 py-3 text-center align-middle text-xs font-semibold uppercase">Delivery Date</th>
                        <th class="bg-blueGray-50 text-blueGray-700 border-blueGray-100 min-w-140-px whitespace-nowrap border border-l-0 border-r-0 border-solid px-6 py-3 text-center align-middle text-xs font-semibold uppercase">Worked on (Hours)</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=tasks
                            key=|(id, _)| id.clone()
                            children=move|(_, task)| view! {
                                <tr>
                                    <th class="whitespace-nowrap border-l-0 border-r-0 border-t-0 p-4 px-6 text-center align-middle text-xs">{task.task_name}</th>
                                    <td class="whitespace-nowrap border-l-0 border-r-0 border-t-0 p-4 px-6 text-center align-middle text-xs">{task.assigned_date.to_string()}</td>
                                    <td class="whitespace-nowrap border-l-0 border-r-0 border-t-0 p-4 px-6 text-center align-middle text-xs">{task.start_date.to_string()}</td>
                                    <td class="whitespace-nowrap border-l-0 border-r-0 border-t-0 p-4 px-6 text-center align-middle text-xs">{task.delivery_date.unwrap_or_default().to_string()}</td>
                                    <td class="whitespace-nowrap border-l-0 border-r-0 border-t-0 p-4 px-6 text-center align-middle text-xs">{task.hour_taken.unwrap_or_default().to_string()}</td>
                                </tr>
                            }
                        />
                    </tbody>
                    </table>
                </div>
                </div>
            </div>
        </section>
    }
}