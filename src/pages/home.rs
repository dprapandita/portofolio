use leptos::prelude::*;
use crate::components::button::PrimaryButton;
use crate::models::get_projects;

#[component]
pub fn HomePage() -> impl IntoView {
    let recent_projects = Resource::new(|| (), |_| async move { get_projects().await.unwrap_or_default() });

    view! {
        <div class="flex flex-col gap-24 py-16">
            // Hero section
            <section class="text-center flex flex-col items-center gap-6 mt-10">
                <h1 class="font-syne font-bold text-6xl md:text-8xl text-white tracking-tight leading-tight max-w-5xl">
                    "My journey life"
                </h1>
                
                <div class="mt-4">
                    <PrimaryButton
                     label="Explore Projects".to_string()
                      href="/projects"
                    />
                </div>
            </section>

            // About Me split section
            <section class="grid grid-cols-1 md:grid-cols-12 gap-12 mt-16 items-center">
                <div class="md:col-span-5 flex flex-col gap-6">
                    <h2 class="font-syne font-bold text-4xl">"Let's get know about me closer"</h2>
                    <p class="text-text-secondary leading-relaxed">
                        "My Self is a New York-based architectural designer with a passion for building minimal, adaptive visual identities. With over a decade of experience, we focus on clarity, precision, and purpose."
                    </p>
                    <div class="mt-4">
                        <PrimaryButton
                         label="Discover More About Me".to_string()
                          href="/about"
                        />
                    </div>
                </div>
                
                <div class="md:col-span-7 rounded-3xl overflow-hidden shadow-2xl bg-bg-card relative h-80">
                    <img src="https://images.unsplash.com/photo-1542744094-3a31f272c490?auto=format&fit=crop&q=80&w=1200" 
                         alt="About Workspace" class="w-full h-full object-cover grayscale opacity-70 transition-all hover:grayscale-0" />
                </div>
            </section>

            // Works Highlight
            <section class="mt-16 flex flex-col gap-12">
                <div class="text-center">
                    <h2 class="font-syne font-bold text-5xl md:text-6xl">"My Projects Highlight"</h2>
                    <div class="w-20 h-1 bg-brand mx-auto mt-6 rounded-full"></div>
                </div>
                
                <div class="flex flex-col gap-8 w-full max-w-5xl mx-auto border-t border-border">
                    <Transition fallback=|| view! { <div class="text-center py-10">"Loading..."</div> }>
                        {move || {
                            let projs = recent_projects.get().unwrap_or_default();
                            let top_projs = projs.into_iter().take(3).collect::<Vec<_>>();
                            
                            view! {
                                <>
                                {top_projs.into_iter().map(|proj| {
                                    view! {
                                        <a href=format!("/projects/{}", proj.id) class="flex items-center justify-between border-b border-border py-8 group hover:-translate-y-1 transition-transform cursor-pointer">
                                            <div class="flex items-center gap-10">
                                                <div class="w-32 h-20 rounded-xl overflow-hidden hidden sm:block shadow-lg bg-bg-card">
                                                    <img src=proj.image_url alt=proj.title.clone()
                                                         class="w-full h-full object-cover grayscale group-hover:grayscale-0 transition-all duration-500" />
                                                </div>
                                                <div class="flex flex-col gap-1">
                                                    <h3 class="font-syne text-2xl font-bold group-hover:text-brand transition-colors text-white">{proj.title}</h3>
                                                    <span class="text-text-secondary font-rubik text-sm tracking-widest uppercase text-brand font-bold">{proj.category}</span>
                                                </div>
                                            </div>
                                            <div class="w-12 h-12 rounded-full border border-border flex items-center justify-center group-hover:bg-brand group-hover:border-brand transition-colors">
                                                <span class="text-white">"→"</span>
                                            </div>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                                </>
                            }
                        }}
                    </Transition>
                    
                    <div class="flex justify-center mt-12">
                         <a href="/projects" class="text-white border border-border px-8 py-3 rounded-full hover:border-brand hover:text-brand transition-all font-syne uppercase tracking-wider text-sm">
                             "View All Projects"
                         </a>
                    </div>
                </div>
            </section>

            // Testimonial highlight
            <section class="bg-bg-card rounded-3xl p-16 flex flex-col items-center text-center gap-8 relative overflow-hidden">
                <div class="text-brand text-6xl opacity-30 absolute top-8 left-12 font-serif">"“"</div>
                <h3 class="font-syne font-bold text-3xl md:text-4xl max-w-3xl leading-relaxed z-10">
                    "My Self consistently delivers adaptive, stunning designs that scale perfectly with our business growth. A true master of digital craftsmanship."
                </h3>
                <div class="flex flex-col items-center gap-1 z-10">
                    <span class="font-syne font-bold text-xl text-brand uppercase tracking-wider">"Martin Woods"</span>
                    <span class="text-sm text-text-secondary">"CEO, TechFlow App"</span>
                </div>
            </section>

            // --- Contact Pre-Footer ---
            <section class="flex flex-col items-center text-center gap-8 mt-16 mb-8">
                <h2 class="font-syne font-bold text-5xl">"Get in Touch With Us"</h2>
                <a href="mailto:info@aaronn.com" class="text-2xl md:text-4xl font-rubik text-text-secondary hover:text-brand transition-colors border-b-2 border-transparent hover:border-brand pb-2">
                    "info@aaronn.com"
                </a>
            </section>
            
        </div>
    }
}
