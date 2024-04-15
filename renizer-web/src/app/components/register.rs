use leptos::*;
use leptos_router::{use_params_map, Redirect, FromFormData};
use super::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct RegistrationData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact: String,
    pub password: String,
    pub confirm_password: String,
    #[serde(default)]
    pub hourly_rate: String,
    #[serde(default)]
    pub working_experience: String,
    #[serde(default)]
    pub working_department: String,
    #[serde(default)]
    pub organization: String,
    #[serde(default)]
    pub organization_email: String,
    #[serde(default)]
    pub organization_contact: String,
    #[serde(default)]
    pub organization_location: String,
}

#[component]
pub fn Register(action: Action<Registration, Result<(), ServerFnError>>) -> impl IntoView {
    let user_type = move|| match use_params_map().with(|params| params.get("user-type").cloned()) {
        Some(s) => s,
        None => String::new()
    };

    let (first_name, set_first_name) = create_signal("");
    let (last_name, set_last_name) = create_signal("");
    let (email, set_email) = create_signal("");
    let (contact, set_contact) = create_signal("");
    let (password, set_password) = create_signal("");
    let (confirm_password, set_confirm_password) = create_signal("");
    let (hourly_rate, set_hourly_rate) = create_signal("");
    let (working_experience, set_working_experience) = create_signal("");
    let (working_department, set_working_department) = create_signal("");
    let (organization, set_organization) = create_signal("");
    let (organization_email, set_organization_email) = create_signal("");
    let (organization_contact, set_organization_contact) = create_signal("");
    let (organization_location, set_organization_location) = create_signal("");

    let (org_registered, set_org_registered) = create_signal(false);
    let (individual_investor, set_individual_investor) = create_signal(false);

    let _ = create_local_resource(action.value(), move|res| async move {
        if let Some(Ok(_)) = res {
            set_first_name("");
            set_last_name("");
            set_email("");
            set_password("");
            set_confirm_password("");
            set_contact("");
            set_hourly_rate("");
            set_working_department("");
            set_organization("");
            set_working_experience("");
            set_organization_contact("");
            set_organization_email("");
            set_organization_location("");
            set_org_registered(false);
            set_individual_investor(false);
            leptos_router::use_navigate()("/", Default::default());
        }
    });

    view! {
        <Show
            when=move || !matches!(user_type().as_str(), "manager" | "contributor" | "investor")
            fallback=move || {
                view! {
                    <section class="gradient-form h-screen bg-gray-700 grid place-items-center">
                        <div class="container h-full p-10">
                            <div class="flex h-full flex-wrap items-center justify-center text-light">
                                <div class="w-full">
                                    <div class="block rounded-lg bg-white shadow-lg dark:bg-neutral-800">
                                        <form class="g-0 lg:flex lg:flex-wrap" on:submit=move|ev| {
                                            ev.prevent_default();
                                            let data = RegistrationData::from_event(&ev);
                                            if let Ok(re_data) = data {
                                                action.dispatch(Registration { re_data });
                                            }
                                        }
                                            >
                                            <div class="px-4 md:px-0 lg:w-6/12">
                                                <div class="md:mx-6 md:p-12">
                                                    <div class="grid place-items-center">
                                                        <Logo/>
                                                    </div>

                                                    <section class="space-y-4">
                                                        <p class="mb-4">Please register an account</p>

                                                        <div class="flex flex-1 space-x-4">
                                                            <Input
                                                                label="First Name".into()
                                                                value=first_name
                                                                name="first_name".into()
                                                            />
                                                            <Input
                                                                label="Last Name".into()
                                                                value=last_name
                                                                name="last_name".into()
                                                            />
                                                        </div>

                                                        <Input
                                                            label="Email".into()
                                                            type_="email".into()
                                                            value=email
                                                            name="email".into()
                                                        />

                                                        <Input
                                                            label="Contact".into()
                                                            value=contact
                                                            name="contact".into()
                                                        />

                                                        <Input
                                                            label="Password".into()
                                                            type_="password".into()
                                                            value=password
                                                            name="password".into()
                                                        />

                                                        <Input
                                                            label="Confirm Password".into()
                                                            type_="password".into()
                                                            value=confirm_password
                                                            name="confirm_password".into()
                                                        />
                                                    </section>
                                                </div>
                                            </div>

                                            <div
                                                class="rounded-b-lg lg:w-6/12 lg:rounded-e-lg lg:rounded-bl-none"
                                                style="background: linear-gradient(to right, #ee7724, #d8363a, #dd3675, #b44593)"
                                            >
                                                <div class="p-6 text-light md:mx-6 md:p-12 space-y-4">
                                                    <div class="grid grid-cols-1 space-y-4">
                                                        <Show
                                                            when=move || {
                                                                matches!(user_type().as_str(), "manager" | "contributor")
                                                            }

                                                            fallback=|| view! {}
                                                        >
                                                            <Input
                                                                label="Hourly Rate".into()
                                                                value=hourly_rate
                                                                name="hourly_rate".into()
                                                            />

                                                            <Input
                                                                label="Working Experience".into()
                                                                value=working_experience
                                                                name="working_experience".into()
                                                            />

                                                            <Show
                                                                when=move || matches!(user_type().as_str(), "contributor")
                                                                fallback=|| view! {}
                                                            >
                                                                <Input
                                                                    label="Working Department".into()
                                                                    value=working_department
                                                                    name="working_department".into()
                                                                />
                                                            </Show>
                                                        </Show>

                                                        <Show
                                                            when=move || matches!(user_type().as_str(), "investor")
                                                            fallback=|| view! {}
                                                        >
                                                            <CheckBox
                                                                label="Register as an individual investor?".into()
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
                                                                    label="Organization is already registered?".into()
                                                                    on_click=move |_| {
                                                                        set_org_registered.update(|v| *v = !*v);
                                                                    }
                                                                />

                                                            </Show>

                                                            <Input
                                                                label="Organization Name".into()
                                                                value=organization
                                                                name="organization".into()
                                                            />

                                                            <Show
                                                                when=move || {
                                                                    matches!(user_type().as_str(), "manager" | "investor")
                                                                        && !org_registered()
                                                                }

                                                                fallback=|| view! {}
                                                            >
                                                                <Input
                                                                    label="Organization Email".into()
                                                                    value=organization_email
                                                                    name="organization_email".into()
                                                                />

                                                                <Input
                                                                    label="Organization Contact".into()
                                                                    value=organization_contact
                                                                    name="organization_contact".into()
                                                                />

                                                                <Input
                                                                    label="Organization Location".into()
                                                                    value=organization_location
                                                                    name="organization_location".into()
                                                                />
                                                            </Show>
                                                        </Show>

                                                    </div>

                                                    <div class="mb-12 pb-1 pt-1 text-center">
                                                        <button
                                                            class="mb-3 border border-neutral-800 inline-block w-full rounded px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-light shadow-dark-3 transition duration-150 ease-in-out hover:shadow-dark-2 focus:shadow-dark-2 focus:outline-none focus:ring-0 active:shadow-dark-2 dark:shadow-black/30 dark:hover:shadow-dark-strong dark:focus:shadow-dark-strong dark:active:shadow-dark-strong bg-gradient-bgi-secondary"
                                                            type="submit"
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
                                                        >
                                                            Login
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        </form>
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

#[server(Registration)]
pub async fn registration(re_data: RegistrationData) -> Result<(), ServerFnError> {
    use crate::{ex::ReqInfo, 
        tables::{user, investor, project_manager, project_contributor, project_associate, organization},
        auth::ssr::pool
    };
    use leptos::use_context;
    use bcrypt::{hash, DEFAULT_COST};
    use uuid::Uuid;

    let req_url = use_context::<ReqInfo>().unwrap().url;
    let user_type = req_url.rsplit_once("/").unwrap_or_default().1;

    if !matches!(user_type, "investor" | "manager" | "contributor") {
        return Err(ServerFnError::ServerError("Invalid user type!".into()))
    }

    let user_t = if user_type == "investor" { "I" } else { "P" };
    let mut user_id = Uuid::new_v4().to_string();
    user_id.truncate(8);

    let mut user = user::User {
        user_id: user_id.clone(),
        first_name: re_data.first_name.clone(),
        last_name: (!re_data.last_name.is_empty()).then_some(re_data.last_name.clone()),
        password: hash(re_data.password.clone(), DEFAULT_COST)?,
        email: re_data.email.clone(),
        contact_number: (!re_data.contact.is_empty()).then_some(re_data.contact.clone()),
        user_type: user_t.into(),
        // organization_email remains empty as long as the organization doesn't exist already 
        org_id: re_data.organization_email.is_empty().then_some(
            sqlx::query_as::<_, organization::Organization>("SELECT * FROM Organization_T WHERE name = ")
                .bind(re_data.organization.clone())
                .fetch_one(&pool()?)
                .await?
                .org_id
        )
    };

    if !re_data.organization_email.is_empty() {
        let mut org_id = Uuid::new_v4().to_string();
        org_id.truncate(8);
        let org = organization::Organization {
            org_id: org_id.clone(),
            name: re_data.organization.clone(),
            email: Some(re_data.organization_email.clone()),
            location: (!re_data.organization_location.is_empty()).then_some(re_data.organization_location.clone()) 
        };
        user.org_id = Some(org_id.clone());
        organization::ssr::insert_organization(org).await?;
    }

    // insert user
    user::ssr::insert_user(user).await?;

    if user_type == "investor" {
        let mut investor = investor::Investor {
            i_user_id: user_id,
            investor_type: "".into()
        };

        if !re_data.organization.is_empty() {
            investor.investor_type = "Corporate".into(); 
        }

        investor::ssr::insert_investor(investor).await?;
    } else {
        let mut associate = project_associate::ProjectAssociate {
            p_user_id: user_id.clone(),
            working_experience: re_data.working_experience.parse::<u32>().unwrap_or_default(),   
            hourly_rate: re_data.hourly_rate.parse::<f32>().unwrap_or_default(),
            associate_type: "".into(),
        };

        if user_type == "manager" {
            let manager = project_manager::ProjectManager {
                m_p_user_id: user_id.clone()
            };
            associate.associate_type = "M".into();
            project_associate::ssr::insert_project_associate(associate).await?;
            project_manager::ssr::insert_project_manager(manager).await?;
        }else {
            let contributor = project_contributor::ProjectContributor {
                c_p_user_id: user_id.clone(),
                working_department: re_data.working_department
            };
            associate.associate_type = "C".into();
            project_associate::ssr::insert_project_associate(associate).await?;
            project_contributor::ssr::insert_project_contributor(contributor).await?;
        }
    }

    Ok(())
}
