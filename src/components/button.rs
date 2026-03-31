use leptos::prelude::*;
use leptos::*;
use leptos_router::components::A;
use web_sys::MouseEvent;

#[component]
pub fn PrimaryButton(
    // Text button
    #[prop(into)] label: String,
    // link
    #[prop(optional, into)] href: Option<String>,
    // Fungsi callback custom
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    // Memungkinkan penyisipan class Tailwind tambahan dari luar
    #[prop(optional, into)] class: Option<String>,
    // Signal reaktif untuk menonaktifkan tombol (contoh: saat loading)
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let combined_class = {
        let base = "bg-brand hover:bg-brand-hover text-white font-rubik uppercase tracking-wider text-sm font-medium py-4 px-10 rounded-full transition-colors duration-300 shadow-[0_0_20px_rgba(255,107,0,0.3)] hover:shadow-[0_0_30px_rgba(255,107,0,0.5)] active:scale-95 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100 disabled:hover:shadow-[0_0_20px_rgba(255,107,0,0.3)] inline-block text-center";

        if let Some(custom) = &class {
            format!("{} {}", base, custom)
        } else {
            base.to_string()
        }
    };

    let handle_click = move |e: MouseEvent| {
        // Cegah klik jika sedang disabled
        if disabled.get() {
            e.prevent_default();
            return;
        }

        if let Some(cb) = on_click {
            cb.run(e);
        }
    };

    // Logika Percabangan Render (Smart Rendering)
    if let Some(url) = href {
        if url.starts_with("http") {
            // Render sebagai Tautan Eksternal (Mendukung "Open in new tab" & SEO)
            view! {
                <a href=url class=combined_class target="_blank" rel="noopener noreferrer">
                    {label}
                </a>
            }.into_any()
        } else {
            // Render sebagai Tautan Internal SPA (Mulus, tanpa reload halaman)
            view! {
                <A href=url attr:class=combined_class>
                    {label.clone()}
                </A>
            }.into_any()
        }
    } else {
        // Render sebagai Tombol Aksi Murni
        view! {
            <button
                class=combined_class
                on:click=handle_click
                disabled=move || disabled.get()
            >
                {label}
            </button>
        }.into_any()
    }
}
