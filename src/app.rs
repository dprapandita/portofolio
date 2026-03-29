use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, ParentRoute, Router, Routes},
    path
};

use crate::components::layout::Layout;
use crate::pages::home::HomePage;
use crate::pages::about::AboutPage;
use crate::pages::projects::ProjectsPage;
use crate::pages::contact::ContactPage;
use crate::pages::project_detail::ProjectDetailPage;
use crate::pages::style_guide::StyleGuidePage;
use crate::pages::admin::login::LoginPage;
use crate::pages::admin::dashboard::DashboardPage;
use crate::pages::admin::project_form::ProjectFormPage;

#[cfg(feature = "ssr")]
pub mod state {
    use sqlx::SqlitePool;
    use leptos::prelude::LeptosOptions;

    #[derive(Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub pool: SqlitePool,
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                // Google Fonts: Syne & Rubik
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
                <link href="https://fonts.googleapis.com/css2?family=Rubik:ital,wght@0,300;0,400;0,500;0,600;0,700;0,800;1,400&family=Syne:wght@400;500;600;700;800&display=swap" rel="stylesheet"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-bg-dark text-text-primary antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/myself.css"/>

        // sets the document title
        <Title text="My Self — Adaptive Logo Design"/>

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                // Layout acts as a shell providing Navbar and Footer to its children Route
                <ParentRoute path=path!("") view=Layout>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("about") view=AboutPage/>
                    <Route path=path!("projects") view=ProjectsPage/>
                    // Dynamic route for individual project cases
                    <Route path=path!("projects/:id") view=ProjectDetailPage/>
                    <Route path=path!("contact") view=ContactPage/>
                </ParentRoute>
                // Admin Routes (Independent without standard navbar to disguise them, or maybe use Layout? The user asked for "all page follow the style from main css" but usually admin dashboard has no public footer)
                <Route path=path!("portal-admin-login") view=LoginPage/>
                <Route path=path!("portal-admin") view=DashboardPage/>
                <Route path=path!("portal-admin/new") view=ProjectFormPage/>
                <Route path=path!("portal-admin/edit/:id") view=ProjectFormPage/>
                
                // Independent routes if needed
                <Route path=path!("style-guide") view=StyleGuidePage/>
            </Routes>
        </Router>
    }
}
