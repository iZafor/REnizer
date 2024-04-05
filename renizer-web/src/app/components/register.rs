use leptos::*;
use leptos_router::{use_params_map, Redirect};
use super::*;

#[component]
pub fn Register() -> impl IntoView {
    let user_type = move|| match use_params_map().with(|params| params.get("user-type").cloned()) {
        Some(s) => s,
        None => String::new()
    };

    let (first_name, _set_first_name) = create_signal("");
    let (last_name, _set_last_name) = create_signal("");
    let (email, _set_email) = create_signal("");
    let (contact, _set_contact) = create_signal("");
    let (password, _set_password) = create_signal("");
    let (confirm_password, _set_confirm_password) = create_signal("");
    let (hourly_rate, _set_hourly_rate) = create_signal("");
    let (working_experience, _set_working_experience) = create_signal("");
    let (working_department, _set_working_department) = create_signal("");
    let (organization, _set_organization) = create_signal("");
    let (organization_email, _set_organization_email) = create_signal("");
    let (organization_contact, _set_organization_contact) = create_signal("");
    let (organization_location, _set_organization_location) = create_signal("");

    let (org_registered, set_org_registered) = create_signal(false);
    let (individual_investor, set_individual_investor) = create_signal(false);

    view! {
        <Show
            when=move || !matches!(user_type().as_str(), "manager" | "contributor" | "investor")
            fallback=move || {
                view! {
                    <section class="gradient-form h-screen bg-neutral-200 dark:bg-neutral-700 grid place-items-center">
                        <div class="container h-full p-10">
                            <div class="flex h-full flex-wrap items-center justify-center text-neutral-800 dark:text-neutral-200">
                                <div class="w-full">
                                    <div class="block rounded-lg bg-white shadow-lg dark:bg-neutral-800">
                                        <div class="g-0 lg:flex lg:flex-wrap">
                                            <div class="px-4 md:px-0 lg:w-6/12">
                                                <div class="md:mx-6 md:p-12">
                                                    <div class="grid place-items-center">
                                                        <Logo/>
                                                    </div>

                                                    <form class="space-y-4">
                                                        <p class="mb-4">Please register an account</p>

                                                        <div class="flex flex-1 space-x-4">
                                                            <Input label=String::from("First Name") value=first_name/>
                                                            <Input label=String::from("Last Name") value=last_name/>
                                                        </div>

                                                        <Input
                                                            label=String::from("Email")
                                                            type_=String::from("email")
                                                            value=email
                                                        />

                                                        <Input label=String::from("Contact") value=contact/>

                                                        <Input
                                                            label=String::from("Password")
                                                            type_=String::from("password")
                                                            value=password
                                                        />

                                                        <Input
                                                            label=String::from("Confirm Password")
                                                            type_=String::from("password")
                                                            value=confirm_password
                                                        />
                                                    </form>
                                                </div>
                                            </div>

                                            <div
                                                class="rounded-b-lg lg:w-6/12 lg:rounded-e-lg lg:rounded-bl-none"
                                                style="background: linear-gradient(to right, #ee7724, #d8363a, #dd3675, #b44593)"
                                            >
                                                <div class="p-6 text-white md:mx-6 md:p-12 space-y-4">
                                                    <div class="grid grid-cols-1 space-y-4">
                                                        <Show
                                                            when=move || {
                                                                matches!(user_type().as_str(), "manager" | "contributor")
                                                            }

                                                            fallback=|| view! {}
                                                        >
                                                            <Input
                                                                label=String::from("Hourly Rate")
                                                                value=hourly_rate
                                                                on_input=move |ev| {
                                                                    let value = event_target_value(&ev);
                                                                    logging::log!("{value}");
                                                                }
                                                            />

                                                            <Input
                                                                label=String::from("Working Experience")
                                                                value=working_experience
                                                            />

                                                            <Show
                                                                when=move || matches!(user_type().as_str(), "contributor")
                                                                fallback=|| view! {}
                                                            >
                                                                <Input
                                                                    label=String::from("Working Department")
                                                                    value=working_department
                                                                />
                                                            </Show>
                                                        </Show>

                                                        <Show
                                                            when=move || matches!(user_type().as_str(), "investor")
                                                            fallback=|| view! {}
                                                        >
                                                            <CheckBox
                                                                label=String::from("Register as an individual investor?")
                                                                on_click=move |_| {
                                                                    set_individual_investor.update(|v| *v = !*v);
                                                                }
                                                            />

                                                        </Show>

                                                        <Show
                                                            when=move || { !individual_investor() }

                                                            fallback=|| view! {}
                                                        >

                                                            <Show
                                                                when=move || {
                                                                    !matches!(user_type().as_str(), "investor" | "contributor")
                                                                }

                                                                fallback=|| view! {}
                                                            >
                                                                <CheckBox
                                                                    label=String::from("Organization is already registered?")
                                                                    on_click=move |_| {
                                                                        set_org_registered.update(|v| *v = !*v);
                                                                    }
                                                                />

                                                            </Show>

                                                            <Input
                                                                label=String::from("Organization Name")
                                                                value=organization
                                                            />

                                                            <Show
                                                                when=move || {
                                                                    matches!(user_type().as_str(), "manager" | "investor")
                                                                        && !org_registered()
                                                                }

                                                                fallback=|| view! {}
                                                            >
                                                                <Input
                                                                    label=String::from("Organization Email")
                                                                    value=organization_email
                                                                />

                                                                <Input
                                                                    label=String::from("Organization Contact")
                                                                    value=organization_contact
                                                                />

                                                                <Input
                                                                    label=String::from("Organization Location")
                                                                    value=organization_location
                                                                />
                                                            </Show>
                                                        </Show>

                                                    </div>

                                                    <div class="mb-12 pb-1 pt-1 text-center">
                                                        <button
                                                            class="mb-3 border border-neutral-800 inline-block w-full rounded px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-white shadow-dark-3 transition duration-150 ease-in-out hover:shadow-dark-2 focus:shadow-dark-2 focus:outline-none focus:ring-0 active:shadow-dark-2 dark:shadow-black/30 dark:hover:shadow-dark-strong dark:focus:shadow-dark-strong dark:active:shadow-dark-strong bg-gradient-bgi-secondary"
                                                            type="button"
                                                            data-twe-ripple-init
                                                            data-twe-ripple-color="light"
                                                        >
                                                            Sign up
                                                        </button>
                                                    </div>

                                                    <div class="flex items-center justify-between pb-6">
                                                        <p class="mb-0 me-2">Have an account?</p>
                                                        <button
                                                            type="button"
                                                            class="inline-block rounded border-2 border-danger px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-danger transition duration-150 ease-in-out hover:border-danger-600 hover:bg-danger-50/50 hover:text-danger-600 focus:border-danger-600 focus:bg-danger-50/50 focus:text-danger-600 focus:outline-none focus:ring-0 active:border-danger-700 active:text-danger-700 dark:hover:bg-rose-950 dark:focus:bg-rose-950"
                                                            data-twe-ripple-init
                                                            data-twe-ripple-color="light"
                                                            on:click=move |_| {
                                                                leptos_router::use_navigate()("/", Default::default());
                                                            }
                                                        >

                                                            Login
                                                        </button>
                                                    </div>
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
        >

            <Redirect path="/user-type"/>
        </Show>
    }
}