mod components;

use crate::{auth::{get_user, Logout}, error_template::{AppError, ErrorTemplate}};
use components::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::tables::user::User;

pub type UserCtx = Resource<(usize, usize, usize), Result<Option<User>, ServerFnError>>;
pub type LoginCtx = Action<Login, Result<Option<User>, ServerFnError>>;
pub type RegisterCtx =  Action<Registration, Result<(), ServerFnError>>;
pub type LogoutCtx = Action<Logout, Result<(), ServerFnError>>;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let login_action = Action::<Login, _>::server();
    let register_action = Action::<Registration, _>::server();
    let logout_action = Action::<Logout, _>::server();

    let user = create_resource(
        move || (
            login_action.version().get(),
            register_action.version().get(),
            logout_action.version().get()
        ),
        move |_| get_user() 
    );

    provide_context(user);
    provide_context(login_action);
    provide_context(logout_action);
    provide_context(register_action);

    view! {
        <Title text="REnizer"/>
        // logo
        <Link rel="icon" href="/images/logo/icon.png"/>
        <Stylesheet id="leptos" href="/pkg/renizer-web.css"/>

        <Router fallback=|| view! { <NotFound/> }>
            <main>
                <Routes>
                    <Route path="" view=HomePage>
                        <Route path="" view=move || view! { <Login action=login_action/> }/>
                        <Route path="/user-type" view=UserType/>
                        <Route
                            path="/register/:user-type"
                            view=move || view! { <Register action=register_action/> }
                        />
                        <Route path="/project/:project-id" view=ProjectView/>
                        <Route path="/profile" view=Profile/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[allow(unused)]
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Navbar/>
        <Outlet/>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    let mut outside_errors = Errors::default();
    outside_errors.insert_with_default_key(AppError::NotFound);

    view! { <ErrorTemplate outside_errors/> }
}