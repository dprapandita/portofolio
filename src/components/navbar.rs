use leptos::prelude::*;
 

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="flex items-center justify-between py-8 px-6 lg:px-12 max-w-[1400px] mx-auto w-full z-50 relative">
            // Logo
            <a href="/">
                <div class="flex items-center gap-1">
                    <span class="text-3xl font-syne font-bold text-white tracking-widest uppercase">"MY"</span>
                    <span class="text-3xl font-syne font-bold outline-text text-transparent" style="-webkit-text-stroke: 1px #fff;">"SELF"</span>
                    <div class="w-1.5 h-1.5 rounded-full bg-brand ml-0.5 mt-2"></div>
                </div>
            </a>

            // Links (Desktop)
            <div class="hidden md:flex items-center gap-10">
                <a href="/" class="text-sm font-rubik text-text-secondary hover:text-brand uppercase tracking-wider transition-colors">"Home"</a>
                <a href="/about" class="text-sm font-rubik text-text-secondary hover:text-brand uppercase tracking-wider transition-colors">"About Me"</a>
                <a href="/projects" class="text-sm font-rubik text-text-secondary hover:text-brand uppercase tracking-wider transition-colors">"My Projects"</a>
                
                // Contact acting as a distinct CTA link
                <a href="/contact" class="text-sm font-rubik text-white border border-border px-6 py-2.5 rounded-full hover:border-brand hover:text-brand transition-all">
                    "Contact"
                </a>
            </div>
            
            // Mobile Menu Toggle (Placeholder)
            <button class="md:hidden text-white focus:outline-none">
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
                </svg>
            </button>
        </nav>
    }
}
