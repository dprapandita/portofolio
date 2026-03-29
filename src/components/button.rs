use leptos::prelude::*;

#[component]
pub fn PrimaryButton(
    #[prop(into)] label: String,
    // Add additional props like `href` or `on_click` if needed later
) -> impl IntoView {
    view! {
        <button class="bg-brand hover:bg-brand-hover text-white font-rubik uppercase tracking-wider text-sm font-medium py-4 px-10 rounded-full transition-colors duration-300 shadow-[0_0_20px_rgba(255,107,0,0.3)] hover:shadow-[0_0_30px_rgba(255,107,0,0.5)]">
            {label}
        </button>
    }
}
