use leptos::*;
use crate::app::components::{project_view::RoleRowData, TaskTable};

#[component]
pub fn RoleRow(rd: RoleRowData) -> impl IntoView {
    let (show_task_table, set_show_table) = create_signal(false);

    view! {
        <tr tabindex="0" class="h-16 rounded border border-gray-100 focus:outline-none text-light">
            <td class="">
                <div class="flex items-center pl-5 space-x-2">
                    <p class="text-light mr-2 text-base font-medium leading-none">{rd.role.clone()}</p>
                </div>
            </td>
            <td class="pl-5">
                <div class="flex items-center space-x-2">
                    <p>Project Status</p>
                    <p class="text-light ml-2 text-sm leading-none bg-gray-500 px-2 py-1 rounded-md">
                        {if let Some(_) = rd.end_date { "Completed" } else { "Pending" }}
                    </p>
                </div>
            </td>
            <td class="pl-5">
                <div class="flex items-center">
                    <p>Total Tasks</p>
                    <p class="text-light ml-2 text-sm leading-none bg-gray-500 px-2 py-1 rounded-md">{rd.n_tasks}</p>
                </div>
            </td>
            <td class="pl-5">
                <div class="flex items-center">
                    <p>Hourly Rate</p>
                    <p class="text-light ml-2 text-sm leading-none bg-gray-500 px-2 py-1 rounded-md">{rd.hourly_rate.to_string()}</p>
                </div>
            </td>
            <td class="pl-5">
                <div class="flex items-center">
                    <p>Start Date</p>
                    <p class="text-light ml-2 text-sm leading-none bg-gray-500 px-2 py-1 rounded-md">{rd.start_date.to_string()}</p>
                </div>
            </td>
            <td class="pl-5">
                <button class="bg-light text-dark rounded px-3 py-3 text-sm leading-none hover:bg-gray-200 focus:outline-none">
                    Due at {rd.expected_day.to_string()}
                </button>
            </td>
            <td class="pl-4">
                <button 
                    class="bg-light text-dark rounded px-5 py-3 text-sm leading-none hover:bg-gray-200 focus:outline-none"
                    on:click=move|_| { set_show_table(true) }
                >
                    View
                </button>
            </td>
        </tr>
        <tr class="h-3"></tr>
        <Show
            when=show_task_table
        >
            <TaskTable p_user_id=rd.p_user_id.clone() role=rd.role.clone() on_close=move|_| { set_show_table(false) } />
        </Show>
    }
}