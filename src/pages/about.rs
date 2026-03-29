use leptos::prelude::*;
use crate::components::button::PrimaryButton;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-16 py-16 text-white">
            // Header
            <section class="text-center flex flex-col items-center gap-4 border-b border-border pb-10">
                <h1 class="font-syne font-bold text-5xl md:text-6xl text-white">"About Me"</h1>
                <p class="text-text-secondary uppercase tracking-widest text-sm font-rubik mt-2">"Little Brief About Myself"</p>
                <div class="w-16 h-1 mt-4 bg-brand rounded-full"></div>
            </section>

            // Main Content Split
            <section class="grid grid-cols-1 lg:grid-cols-12 gap-16 pt-8">
                // Left text side
                <div class="lg:col-span-6 flex flex-col gap-8">
                    <h2 class="font-syne font-bold text-4xl leading-tight text-white max-w-lg">
                        "My mission is to make design easier."
                    </h2>
                    
                    <p class="text-text-secondary leading-relaxed font-rubik text-lg">
                        "Create custom Designs with Aaronn that converts more visitors than any website. With the Aaronn portfolio template, you have the ideal foundation to construct a spectacular portfolio."
                    </p>
                    <p class="text-text-secondary leading-relaxed font-rubik text-lg">
                        "Your best design is a reflection of your best self. We focus on elevating brand narratives with immersive, high-quality aesthetics that seamlessly bridge the gap between user experience and visual art."
                    </p>
                    
                    <div class="mt-8 flex items-center gap-6">
                        <PrimaryButton label="Follow Me on Instagram".to_string() />
                    </div>
                </div>
                
                // Right images side
                <div class="lg:col-span-6 relative h-[500px] w-full flex gap-4">
                    <div class="w-1/2 h-[80%] mt-auto rounded-3xl overflow-hidden shadow-xl bg-bg-card">
                        <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?auto=format&fit=crop&q=80&w=600" 
                             alt="Portrait 1" class="w-full h-full object-cover grayscale opacity-80" />
                    </div>
                    <div class="w-1/2 h-[90%] rounded-3xl overflow-hidden shadow-xl bg-bg-card">
                        <img src="https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?auto=format&fit=crop&q=80&w=600" 
                             alt="Portrait 2" class="w-full h-full object-cover grayscale opacity-80" />
                    </div>
                </div>
            </section>
        </div>
    }
}
