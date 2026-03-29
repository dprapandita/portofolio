use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::models::get_project;

#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let project_id = move || {
        params.get().get("id").unwrap_or_default()
    };

    let project_resource = Resource::new(project_id, |id| async move {
        if !id.is_empty() {
            get_project(id).await.unwrap_or(None)
        } else {
            None
        }
    });

    view! {
        <Transition fallback=|| view! { <div class="text-white text-center py-20 font-syne">"Loading Case Study..."</div> }>
            {move || {
                match project_resource.get().flatten() {
                    Some(proj) => view! {
                        <div class="flex flex-col gap-16 py-16 text-white max-w-5xl mx-auto pb-32">
                            // Header
                            <section class="flex flex-col gap-6 text-center items-center">
                                <h1 class="font-syne font-bold text-5xl md:text-7xl">
                                    {proj.title.clone()}
                                </h1>
                                <p class="text-text-secondary uppercase tracking-widest text-sm font-rubik mt-2">
                                    {proj.category.clone()}
                                </p>
                            </section>

                            // Hero Image Container
                            <section class="w-full h-[500px] md:h-[600px] rounded-3xl overflow-hidden shadow-2xl bg-bg-card border border-border">
                                <img src=proj.image_url.clone() alt=proj.title.clone()
                                     class="w-full h-full object-cover grayscale hover:grayscale-0 transition-all duration-700" />
                            </section>

                            // Meta Information
                            <section class="grid grid-cols-2 md:grid-cols-4 border-y border-border py-10 gap-8">
                                <div class="flex flex-col gap-2">
                                    <span class="text-brand font-syne font-bold">"Client:"</span>
                                    <span class="text-text-secondary font-rubik">{proj.client.clone()}</span>
                                </div>
                                <div class="flex flex-col gap-2">
                                    <span class="text-brand font-syne font-bold">"Work:"</span>
                                    <span class="text-text-secondary font-rubik">{proj.work_category.clone()}</span>
                                </div>
                                <div class="flex flex-col gap-2">
                                    <span class="text-brand font-syne font-bold">"Date:"</span>
                                    <span class="text-text-secondary font-rubik">{proj.date_text.clone()}</span>
                                </div>
                                <div class="flex flex-col gap-2">
                                    <span class="text-brand font-syne font-bold">"Tools:"</span>
                                    <span class="text-text-secondary font-rubik">{proj.tools.clone()}</span>
                                </div>
                            </section>

                            // Project Story Content
                            <section class="grid grid-cols-1 md:grid-cols-12 gap-12 mt-8">
                                <div class="md:col-span-4">
                                    <h2 class="font-syne font-bold text-3xl">"The Story"</h2>
                                </div>
                                <div class="md:col-span-8 flex flex-col gap-6 font-rubik text-text-secondary leading-relaxed text-lg">
                                    {proj.story.split('\n').map(|p| view! { <p>{p.to_string()}</p> }).collect::<Vec<_>>()}
                                </div>
                            </section>

                            // Project Outcome Content
                            <section class="grid grid-cols-1 md:grid-cols-12 gap-12 mt-8 border-t border-border pt-16">
                                <div class="md:col-span-4">
                                    <h2 class="font-syne font-bold text-3xl">"The Outcome"</h2>
                                </div>
                                <div class="md:col-span-8 flex flex-col gap-6 font-rubik text-text-secondary leading-relaxed text-lg">
                                    {proj.outcome.split('\n').map(|p| view! { <p>{p.to_string()}</p> }).collect::<Vec<_>>()}
                                </div>
                            </section>

                            // Call to Action
                            <section class="mt-20 text-center">
                                <h3 class="font-syne font-bold text-4xl mb-8">"Ready to elevate your brand?"</h3>
                                <a href="/contact" class="bg-brand text-white px-10 py-4 text-wide rounded-full font-syne font-bold uppercase tracking-widest hover:bg-white hover:text-black transition-colors duration-300">
                                    "Work with me"
                                </a>
                            </section>
                        </div>
                    }.into_any(),
                    None => view! {
                        <div class="text-center py-32 text-red-500 font-syne font-bold text-3xl">
                            "Case Study Not Found"
                        </div>
                    }.into_any()
                }
            }}
        </Transition>
    }
}
