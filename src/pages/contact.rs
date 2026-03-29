use leptos::prelude::*;
use crate::components::button::PrimaryButton;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 py-16 text-white min-h-[70vh]">
            // Header
            <section class="text-center flex flex-col items-center gap-4">
                <h1 class="font-syne font-bold text-5xl md:text-6xl text-white">"Contact Me"</h1>
                <p class="text-text-secondary uppercase tracking-widest text-sm font-rubik mt-2">"For Any Project Knock Us"</p>
                <div class="w-16 h-1 mt-4 bg-brand rounded-full"></div>
            </section>

            // Main Layout
            <section class="grid grid-cols-1 lg:grid-cols-2 gap-16 mt-12">
                // Form Section
                <div class="flex flex-col gap-10 bg-bg-card p-12 rounded-3xl border border-border">
                    <h3 class="font-syne text-3xl font-bold">"Get In Touch"</h3>
                    
                    <form class="flex flex-col gap-8" on:submit=|e| e.prevent_default()>
                        <div class="flex flex-col gap-2">
                            <label class="font-syne uppercase tracking-wider text-sm font-bold text-text-secondary">"Name"</label>
                            <input type="text" 
                                   class="bg-transparent border-b border-border outline-none py-3 text-white focus:border-brand transition-colors font-rubik" 
                                   placeholder="Enter your name" />
                        </div>
                        
                        <div class="flex flex-col gap-2">
                            <label class="font-syne uppercase tracking-wider text-sm font-bold text-text-secondary">"Subject"</label>
                            <input type="text" 
                                   class="bg-transparent border-b border-border outline-none py-3 text-white focus:border-brand transition-colors font-rubik" 
                                   placeholder="What's this about?" />
                        </div>
                        
                        <div class="flex flex-col gap-2">
                            <label class="font-syne uppercase tracking-wider text-sm font-bold text-text-secondary">"Message"</label>
                            <textarea rows="4"
                                      class="bg-transparent border-b border-border outline-none py-3 text-white focus:border-brand transition-colors font-rubik resize-none" 
                                      placeholder="Tell me about your project..."></textarea>
                        </div>
                        
                        <div class="mt-4">
                            <PrimaryButton label="Send Message".to_string() />
                        </div>
                    </form>
                </div>
                
                // Image Section
                <div class="hidden lg:flex w-full h-full min-h-[500px] rounded-3xl overflow-hidden shadow-xl bg-bg-card relative">
                    <img src="https://images.unsplash.com/photo-1542744173-8e7e53415bb0?auto=format&fit=crop&q=80&w=800" 
                         alt="Work setup" class="w-full h-full object-cover grayscale opacity-60" />
                     <div class="absolute inset-0 bg-gradient-to-t from-bg-dark/80 to-transparent"></div>
                     <div class="absolute bottom-12 left-12">
                         <h4 class="font-syne text-2xl font-bold mb-2">"Let's Build Something Great"</h4>
                         <p class="text-text-secondary">"We respond within 24 hours."</p>
                     </div>
                </div>
            </section>
        </div>
    }
}
