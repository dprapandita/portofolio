use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-bg-dark border-t border-border mt-32 relative w-full pt-16 pb-8">
            <div class="max-w-[1400px] mx-auto px-6 lg:px-12 grid grid-cols-1 md:grid-cols-12 gap-12 text-white">
                // Brand Column
                <div class="md:col-span-5 flex flex-col gap-6">
                    <div class="flex items-center gap-1">
                        <span class="text-3xl font-syne font-bold text-white tracking-widest uppercase">"MY"</span>
                        <span class="text-3xl font-syne font-bold text-transparent" style="-webkit-text-stroke: 1px #fff;">"SELF"</span>
                        <div class="w-1.5 h-1.5 rounded-full bg-brand ml-0.5 mt-2"></div>
                    </div>
                    <p class="text-text-secondary text-sm font-rubik leading-relaxed max-w-sm">
                        "Adaptive Logo Design & Brand Architecture. Elevating brand experiences through strategic visual storytelling since 2012."
                    </p>
                </div>

                // Address Column
                <div class="md:col-span-4 flex flex-col gap-4">
                    <h4 class="font-syne font-bold text-lg uppercase tracking-wider text-brand">"Information"</h4>
                    <address class="not-italic text-sm font-rubik text-text-secondary leading-relaxed flex flex-col gap-2">
                        <span>"123 Design Avenue, Suite 405"</span>
                        <span>"Creative District, NY 10001"</span>
                        <a href="tel:+13233342345" class="hover:text-brand transition-colors">"+1 (323) 334-2345"</a>
                        <a href="mailto:info@aaronn.com" class="hover:text-brand transition-colors">"info@aaronn.com"</a>
                    </address>
                </div>

                // Social Column
                <div class="md:col-span-3 flex flex-col gap-4 text-sm font-rubik">
                    <h4 class="font-syne font-bold text-lg uppercase tracking-wider text-brand">"Socials"</h4>
                    <div class="flex flex-col gap-3 text-text-secondary">
                        <a href="#" class="hover:text-white transition-colors">"Instagram"</a>
                        <a href="#" class="hover:text-white transition-colors">"Twitter / X"</a>
                        <a href="#" class="hover:text-white transition-colors">"LinkedIn"</a>
                        <a href="#" class="hover:text-white transition-colors">"Dribbble"</a>
                    </div>
                </div>
            </div>

            // Copyright
            <div class="mt-16 pt-8 border-t border-border text-center text-xs font-rubik text-text-muted">
                "© 2026 My Self. All rights reserved."
            </div>
        </footer>
    }
}
