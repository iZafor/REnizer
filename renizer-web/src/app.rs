mod components;

use crate::error_template::{AppError, ErrorTemplate};
use components::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let login_action = Action::<Login, _>::server();
    let register_action = Action::<Registration, _>::server();
    
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