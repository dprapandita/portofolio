use leptos::prelude::*;
use leptos_router::components::Outlet;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-bg-dark text-text-primary font-rubik flex flex-col selection:bg-brand selection:text-white">
            <Navbar />
            
            <main class="flex-grow w-full max-w-[1400px] mx-auto px-6 lg:px-12">
                // The Outlet renders the matching child route
                <Outlet />
            </main>

            <Footer />
        </div>
    }
}
