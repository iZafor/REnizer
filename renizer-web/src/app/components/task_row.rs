use leptos::*;
use crate::app::components::project_view::TaskRowData;

#[component]
pub fn TaskRow(td: TaskRowData) -> impl IntoView {
    view! {
        <tr tabindex="0" class="h-16 rounded border border-gray-100 focus:outline-none">
            <td>
                <div class="ml-5">
                    <div class="relative flex h-5 w-5 flex-shrink-0 items-center justify-center rounded-sm bg-gray-200">
                        <input
                            placeholder="checkbox"
                            type="checkbox"
                            class="checkbox absolute h-full w-full cursor-pointer opacity-0 focus:opacity-100"
                        />
                        <div class="check-icon hidden rounded-sm bg-indigo-700 text-white">
                            <svg
                                class="icon icon-tabler icon-tabler-check"
                                xmlns="http://www.w3.org/2000/svg"
                                width="20"
                                height="20"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                fill="none"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path stroke="none" d="M0 0h24v24H0z"></path>
                                <path d="M5 12l5 5l10 -10"></path>
                            </svg>
                        </div>
                    </div>
                </div>
            </td>
            <td class="">
                <div class="flex items-center pl-5">
                    <p class="text-light mr-2 text-base font-medium leading-none">{td.role}</p>
                </div>
            </td>
            <td class="pl-24">
                <div class="flex items-center">
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
                        viewBox="0 0 122.88 122.88"
                        style="enable-background:new 0 0 122.88 122.88"
                        xml:space="preserve"
                    >
                        <g>
                            <path
                                class="st0"
                                d="M61.44,0c33.93,0,61.44,27.51,61.44,61.44c0,33.93-27.51,61.44-61.44,61.44C27.51,122.88,0,95.37,0,61.44 C0,27.51,27.51,0,61.44,0L61.44,0z M52.92,30.52h7.51c1.37,0,2.5,1.13,2.5,2.5v28.94h26.41c1.38,0,2.5,1.13,2.5,2.5v7.51 c0,1.38-1.13,2.5-2.5,2.5H50.41V33.02C50.41,31.64,51.54,30.52,52.92,30.52L52.92,30.52z M61.44,13.95 c26.23,0,47.49,21.26,47.49,47.49c0,26.23-21.26,47.49-47.49,47.49c-26.23,0-47.49-21.26-47.49-47.49 C13.95,35.22,35.21,13.95,61.44,13.95L61.44,13.95z"
                            ></path>
                        </g>
                    </svg>
                    <p class="text-light ml-2 text-sm leading-none">
                        {if let Some(_) = td.end_date { "Completed" } else { "Pending" }}

                    </p>
                </div>
            </td>
            <td class="pl-5">
                <div class="flex items-center">
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
                    <p class="text-light ml-2 text-sm leading-none">{td.n_tasks}</p>
                </div>
            </td>
            <td class="pl-5">
                <div class="flex items-center">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 20 20"
                        class="fill-gray-50"
                    >
                        <path
                            d="M3.33331 17.4998V6.6665C3.33331 6.00346 3.59671 5.36758 4.06555 4.89874C4.53439 4.4299 5.17027 4.1665 5.83331 4.1665H14.1666C14.8297 4.1665 15.4656 4.4299 15.9344 4.89874C16.4033 5.36758 16.6666 6.00346 16.6666 6.6665V11.6665C16.6666 12.3295 16.4033 12.9654 15.9344 13.4343C15.4656 13.9031 14.8297 14.1665 14.1666 14.1665H6.66665L3.33331 17.4998Z"
                            stroke="#52525B"
                            stroke-width="1.25"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        ></path>
                        <path
                            d="M10 9.1665V9.17484"
                            stroke="#52525B"
                            stroke-width="1.25"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        ></path>
                        <path
                            d="M6.66669 9.1665V9.17484"
                            stroke="#52525B"
                            stroke-width="1.25"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        ></path>
                        <path
                            d="M13.3333 9.1665V9.17484"
                            stroke="#52525B"
                            stroke-width="1.25"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        ></path>
                    </svg>
                    <p class="text-light ml-2 text-sm leading-none">23</p>
                </div>
            </td>
            <td class="pl-5">
                <div class="flex items-center">
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
                    <p class="text-light ml-2 text-sm leading-none">{td.start_date.to_string()}</p>
                </div>
            </td>
            <td class="pl-5">
                <button class="bg-light text-dark rounded px-3 py-3 text-sm leading-none hover:bg-gray-200 focus:outline-none">
                    Due at {td.expected_day.to_string()}
                </button>
            </td>
            <td class="pl-4">
                <button class="bg-light text-dark rounded px-5 py-3 text-sm leading-none hover:bg-gray-200 focus:outline-none">
                    View
                </button>
            </td>
            <td>
                <div class="relative px-5 pt-2">
                    <button
                        class="rounded-md focus:outline-none focus:ring-2"
                        onclick="dropdownFunction(this)"
                        role="button"
                        aria-label="option"
                    >
                        <svg
                            class="dropbtn"
                            onclick="dropdownFunction(this)"
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewBox="0 0 20 20"
                            fill="none"
                        >
                            <path
                                d="M4.16667 10.8332C4.62691 10.8332 5 10.4601 5 9.99984C5 9.5396 4.62691 9.1665 4.16667 9.1665C3.70643 9.1665 3.33334 9.5396 3.33334 9.99984C3.33334 10.4601 3.70643 10.8332 4.16667 10.8332Z"
                                stroke="#9CA3AF"
                                stroke-width="1.25"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></path>
                            <path
                                d="M10 10.8332C10.4602 10.8332 10.8333 10.4601 10.8333 9.99984C10.8333 9.5396 10.4602 9.1665 10 9.1665C9.53976 9.1665 9.16666 9.5396 9.16666 9.99984C9.16666 10.4601 9.53976 10.8332 10 10.8332Z"
                                stroke="#9CA3AF"
                                stroke-width="1.25"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></path>
                            <path
                                d="M15.8333 10.8332C16.2936 10.8332 16.6667 10.4601 16.6667 9.99984C16.6667 9.5396 16.2936 9.1665 15.8333 9.1665C15.3731 9.1665 15 9.5396 15 9.99984C15 10.4601 15.3731 10.8332 15.8333 10.8332Z"
                                stroke="#9CA3AF"
                                stroke-width="1.25"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></path>
                        </svg>
                    </button>
                    <div class="dropdown-content absolute right-0 z-30 mr-6 hidden w-24 bg-white shadow">
                        <div
                            tabindex="0"
                            class="w-full cursor-pointer px-4 py-4 text-xs hover:bg-indigo-700 hover:text-white focus:text-indigo-600 focus:outline-none"
                        >
                            <p>Edit</p>
                        </div>
                        <div
                            tabindex="0"
                            class="w-full cursor-pointer px-4 py-4 text-xs hover:bg-indigo-700 hover:text-white focus:text-indigo-600 focus:outline-none"
                        >
                            <p>Delete</p>
                        </div>
                    </div>
                </div>
            </td>
        </tr>
        <tr class="h-3"></tr>
    }
}