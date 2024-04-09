use leptos::*;

#[component]
pub fn UserType() -> impl IntoView {
    let navigate_to = |path| {
        leptos_router::use_navigate()(&format!("/register/{path}"), Default::default())
    };

    view! {
        <section class="roboto-regular gradient-form h-screen bg-gray-700 flex flex-col items-center justify-center space-y-14">
            <p class="text-light text-6xl font-bold">Choose Your Role</p>
            <section class="flex justify-center items-center space-x-8">
                <div class="card" on:click=move |_| { navigate_to("manager") }>
                    <div class="flex flex-col space-y-1.5 p-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="w-6 h-6"
                        >
                            <rect width="20" height="14" x="2" y="7" rx="2" ry="2"></rect>
                            <path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"></path>
                        </svg>
                    </div>
                    <div class="p-4">
                        <h3 class="text-2xl font-bold text-light">Manager</h3>
                        <p class="text-sm text-gray-500 dark:text-neutral-800">
                            Access to all areas
                        </p>
                    </div>
                </div>
                <div class="card" on:click=move |_| { navigate_to("contributor") }>
                    <div class="flex flex-col space-y-1.5 p-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="w-6 h-6"
                        >
                            <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
                            <path d="M12 5 9.04 7.96a2.17 2.17 0 0 0 0 3.08v0c.82.82 2.13.85 3 .07l2.07-1.9a2.82 2.82 0 0 1 3.79 0l2.96 2.66"></path>
                            <path d="m18 15-2-2"></path>
                            <path d="m15 18-2-2"></path>
                        </svg>
                    </div>
                    <div class="p-4">
                        <h3 class="text-2xl font-bold text-light">Contributor</h3>
                        <p class="text-sm text-gray-500 dark:text-neutral-800">Limited access</p>
                    </div>
                </div>
                <div class="card" on:click=move |_| { navigate_to("investor") }>
                    <div class="flex flex-col space-y-1.5 p-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="w-6 h-6"
                        >
                            <line x1="12" x2="12" y1="2" y2="22"></line>
                            <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>
                        </svg>
                    </div>
                    <div class="p-4">
                        <h3 class="text-2xl font-bold text-light">Investor</h3>
                        <p class="text-sm text-gray-500 dark:text-neutral-800">
                            Financial information
                        </p>
                    </div>
                </div>
            </section>
        </section>
    }
}