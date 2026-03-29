use leptos::prelude::*;

/// A single color swatch card
#[component]
fn ColorSwatch(
    #[prop(into)] hex: String,
    #[prop(into)] bg_class: String,
    #[prop(default = false)] large: bool,
    #[prop(default = false)] bordered: bool,
) -> impl IntoView {
    let size_class = if large {
        "w-36 h-28"
    } else {
        "w-28 h-20"
    };
    let border_class = if bordered {
        "border border-grey-200"
    } else {
        ""
    };

    view! {
        <div class="flex flex-col gap-1">
            <span class="text-xs text-grey-500 uppercase tracking-wide font-rubik">{hex.clone()}</span>
            <div class={format!("{} {} {} rounded-xl shadow-sm", size_class, bg_class, border_class)}></div>
        </div>
    }
}

/// A single greyscale swatch (smaller, in a row)
#[component]
fn GreySwatch(
    #[prop(into)] hex: String,
    #[prop(into)] bg_class: String,
    #[prop(default = false)] bordered: bool,
) -> impl IntoView {
    let border_class = if bordered {
        "border border-grey-200"
    } else {
        ""
    };

    view! {
        <div class="flex flex-col gap-1 flex-1">
            <span class="text-[10px] text-grey-500 uppercase tracking-wide font-rubik">{hex.clone()}</span>
            <div class={format!("w-full h-14 rounded-lg shadow-sm {} {}", bg_class, border_class)}></div>
        </div>
    }
}

/// Typography style row
#[component]
fn TypographyRow(
    #[prop(into)] label: String,
    #[prop(into)] font_info: String,
    #[prop(into)] size_info: String,
    #[prop(into)] sample_class: String,
    #[prop(into)] sample_text: String,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-12 items-center py-8 border-t border-grey-200">
            <div class="col-span-3">
                <p class="text-sm font-bold text-secondary-2 font-rubik">{label}</p>
                <p class="text-xs text-grey-500 font-rubik mt-1">{font_info}</p>
                <p class="text-xs text-grey-400 font-rubik">{size_info}</p>
            </div>
            <div class="col-span-9 text-right">
                <span class={sample_class}>{sample_text}</span>
            </div>
        </div>
    }
}

/// Section badge/tag
#[component]
fn SectionBadge(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div class="inline-block bg-secondary-2 text-white text-sm font-rubik font-medium px-6 py-2.5 rounded-full shadow-lg">
            {text}
        </div>
    }
}

/// Main Style Guide Page
#[component]
pub fn StyleGuidePage() -> impl IntoView {
    view! {
        // Background grid lines overlay
        <div class="relative min-h-screen bg-white overflow-hidden">
            // Pink column guides
            <div class="absolute inset-0 pointer-events-none z-0">
                <div class="max-w-5xl mx-auto h-full px-8">
                    <div class="grid grid-cols-12 gap-4 h-full">
                        {(0..12).map(|_| view! {
                            <div class="bg-red-50/40 h-full"></div>
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>

            // Content
            <div class="relative z-10 max-w-5xl mx-auto px-8 py-16">

                // ─── TYPOGRAPHY SECTION ───
                <section id="typography-section" class="mb-24">
                    <SectionBadge text="Typography" />

                    // Typeface showcase
                    <div class="mt-12 mb-16">
                        <h3 class="text-lg font-rubik text-secondary-2 mb-6 font-medium">"Typeface"</h3>
                        <div class="flex items-stretch gap-0 rounded-2xl overflow-hidden shadow-md max-w-xl">
                            // Dark side with big Aa
                            <div class="bg-secondary-2 text-white flex items-center gap-4 px-8 py-6 min-w-[200px]">
                                <span class="text-5xl font-syne font-bold leading-none">"Aa"</span>
                                <div class="flex flex-col">
                                    <span class="text-sm font-rubik font-medium">"Syne &"</span>
                                    <span class="text-sm font-rubik font-medium">"Rubik"</span>
                                </div>
                            </div>
                            // Light side with character set
                            <div class="bg-white border border-grey-200 flex-1 px-8 py-6 flex flex-col justify-center">
                                <p class="text-base font-syne text-secondary-2 tracking-wide mb-3">
                                    "AaBbCcDdEeFfGg  •  0123456789"
                                </p>
                                <div class="flex gap-6 text-sm font-rubik text-grey-500">
                                    <span>"Regular"</span>
                                    <span>"Medium"</span>
                                    <span class="font-bold text-secondary-2">"Bold"</span>
                                    <span class="font-extrabold text-secondary-2">"ExtraBold"</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Styles
                    <h3 class="text-lg font-rubik text-secondary-2 mb-2 font-medium">"Styles"</h3>

                    <TypographyRow
                        label="H1 / Headline 1"
                        font_info="Syne (Bold)"
                        size_info="72 px font size"
                        sample_class="text-7xl font-syne font-bold text-secondary-2"
                        sample_text="The quick brown fox..."
                    />
                    <TypographyRow
                        label="H2 / Headline 2"
                        font_info="Syne"
                        size_info="64 px font size"
                        sample_class="text-6xl font-syne text-secondary-2"
                        sample_text="The quick brown ju..."
                    />
                    <TypographyRow
                        label="H3 / Headline 3"
                        font_info="Syne"
                        size_info="56 px font size"
                        sample_class="text-5xl font-syne text-secondary-2"
                        sample_text="The quick brown fox ..."
                    />
                    <TypographyRow
                        label="H4 / Headline 4"
                        font_info="Rubik (Regular)"
                        size_info="24 px font size"
                        sample_class="text-2xl font-rubik text-secondary-2 leading-relaxed"
                        sample_text="There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form,"
                    />

                    <div class="grid grid-cols-12 items-center py-8 border-t border-grey-200">
                        <div class="col-span-3">
                            <p class="text-sm font-bold text-secondary-2 font-rubik">"Body Large"</p>
                            <p class="text-xs text-grey-500 font-rubik mt-1">"Rubik (Regular)"</p>
                            <p class="text-xs text-grey-400 font-rubik">"18 px font size"</p>
                        </div>
                        <div class="col-span-9 text-right">
                            <p class="text-lg font-rubik text-secondary-2 leading-relaxed">
                                "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable."
                            </p>
                        </div>
                    </div>

                    <div class="grid grid-cols-12 items-center py-8 border-t border-grey-200">
                        <div class="col-span-3">
                            <p class="text-sm font-bold text-secondary-2 font-rubik">"Body Medium"</p>
                            <p class="text-xs text-grey-500 font-rubik mt-1">"Rubik (Regular)"</p>
                            <p class="text-xs text-grey-400 font-rubik">"16px font size"</p>
                        </div>
                        <div class="col-span-9 text-right">
                            <p class="text-base font-rubik text-secondary-2 leading-relaxed">
                                "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable."
                            </p>
                        </div>
                    </div>

                    <div class="grid grid-cols-12 items-center py-8 border-t border-grey-200">
                        <div class="col-span-3">
                            <p class="text-sm font-bold text-secondary-2 font-rubik">"Button"</p>
                            <p class="text-xs text-grey-500 font-rubik mt-1">"Rubik (Regular)"</p>
                            <p class="text-xs text-grey-400 font-rubik">"16px font size"</p>
                        </div>
                        <div class="col-span-9 text-right">
                            <p class="text-base font-rubik text-secondary-2 leading-relaxed">
                                "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable."
                            </p>
                        </div>
                    </div>
                </section>

                // ─── COLORS SECTION ───
                <section id="colors-section" class="mb-24">
                    <SectionBadge text="Colors" />

                    // Brand Colors
                    <div class="mt-12 mb-12">
                        <h3 class="text-lg font-rubik text-secondary-2 mb-6 font-medium">"Brand Colors"</h3>
                        <ColorSwatch hex="#F6A02D" bg_class="bg-brand" large=true />
                    </div>

                    // Secondary Colors
                    <div class="mb-12">
                        <h3 class="text-lg font-rubik text-secondary-2 mb-6 font-medium">"Secondary Colors"</h3>
                        <div class="flex gap-6">
                            <ColorSwatch hex="#1C1614" bg_class="bg-secondary-1" />
                            <ColorSwatch hex="#2A2A2C" bg_class="bg-secondary-2" />
                            <ColorSwatch hex="#3C3F49" bg_class="bg-secondary-3" />
                            <ColorSwatch hex="#FFFFFF" bg_class="bg-white" bordered=true />
                        </div>
                    </div>

                    // Greyscale
                    <div class="mb-8">
                        <h3 class="text-lg font-rubik text-secondary-2 mb-6 font-medium">"Greyscale"</h3>
                        // Light row
                        <div class="flex gap-3 mb-3">
                            <GreySwatch hex="#F9FAFB" bg_class="bg-grey-50" bordered=true />
                            <GreySwatch hex="#F4F4F6" bg_class="bg-grey-100" />
                            <GreySwatch hex="#E5E6EB" bg_class="bg-grey-200" />
                            <GreySwatch hex="#D3D5DA" bg_class="bg-grey-300" />
                            <GreySwatch hex="#9EA3AE" bg_class="bg-grey-400" />
                        </div>
                        // Dark row
                        <div class="flex gap-3">
                            <GreySwatch hex="#6C727F" bg_class="bg-grey-500" />
                            <GreySwatch hex="#4D5461" bg_class="bg-grey-600" />
                            <GreySwatch hex="#394150" bg_class="bg-grey-700" />
                            <GreySwatch hex="#212936" bg_class="bg-grey-800" />
                            <GreySwatch hex="#161D21" bg_class="bg-grey-900" />
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}
