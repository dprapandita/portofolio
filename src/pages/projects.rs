use leptos::prelude::*;
use crate::models::get_projects;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects_resource = Resource::new(|| (), |_| async move { get_projects().await.unwrap_or_default() });
    let show_all = RwSignal::new(false);

    view! {
        <div class="flex flex-col gap-16 py-16">
            // Header
            <section class="text-center flex flex-col items-center gap-4">
                <h1 class="font-syne font-bold text-5xl md:text-6xl text-white">"My projects"</h1>
                <p class="text-text-secondary uppercase tracking-widest text-sm font-rubik mt-2">"Showcase About Projects"</p>
                <div class="w-16 h-1 mt-4 bg-brand rounded-full"></div>
            </section>

            <Transition fallback=|| view! { <div class="text-center text-white py-10 font-syne">"Loading Projects..."</div> }>
                {move || {
                    let projects = projects_resource.get().unwrap_or_default();
                    let total_projects = projects.len();
                    
                    view! {
                        <>
                            <section class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10">
                                {projects.clone().into_iter().enumerate().map(|(id, proj)| {
                                    let proj_id = proj.id;
                                    let img_url = proj.image_url.clone();
                                    let title = proj.title.clone();
                                    let cat = proj.category.clone();
                                    view! {
                                        <Show when={move || show_all.get() || id < 3} fallback=|| view!{}>
                                            <a href=format!("/projects/{}", proj_id) class="flex flex-col gap-4 group cursor-pointer block">
                                                <div class="w-full h-80 rounded-2xl overflow-hidden shadow-lg bg-bg-card relative">
                                                    <img src=img_url.clone() alt=title.clone()
                                                         class="w-full h-full object-cover grayscale group-hover:grayscale-0 group-hover:scale-105 transition-all duration-700" />
                                                         
                                                    // Overlay gradient
                                                    <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
                                                </div>
                                                
                                                <div class="flex flex-col gap-1 px-2">
                                                    <span class="text-brand font-rubik text-xs font-bold tracking-widest uppercase">{cat.clone()}</span>
                                                    <span class="text-white font-syne text-2xl font-bold group-hover:text-brand transition-colors">{title.clone()}</span>
                                                </div>
                                            </a>
                                        </Show>
                                    }
                                }).collect::<Vec<_>>()}
                            </section>

                            <Show when={move || total_projects >= 3 && !show_all.get()} fallback=|| view!{}>
                                <div class="flex justify-center mt-12 w-full">
                                    <button 
                                        on:click=move |_| show_all.set(true)
                                        class="border border-border text-white hover:text-brand hover:border-brand transition-colors uppercase tracking-widest font-rubik text-sm rounded-full py-3 px-8 cursor-pointer">
                                        "Load More"
                                    </button>
                                </div>
                            </Show>
                        </>
                    }
                }}
            </Transition>
        </div>
    }
}
