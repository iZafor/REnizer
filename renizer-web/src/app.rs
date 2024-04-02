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
    
    view! {
        <Stylesheet id="leptos" href="/pkg/renizer-web.css"/>
        <Script src="https://cdn.tailwindcss.com"/>
        <TailwindConfig/>

        // sets the document title
        <Title text="REnizer"/>

        // logo
        <Link rel="icon" href="/images/logo/logo-no-background.png"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/login" view=Login/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! { <h1>"Welcome to REnizer!"</h1> }
}
