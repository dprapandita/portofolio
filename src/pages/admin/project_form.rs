use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[server(SaveProject, "/api")]
pub async fn save_project(
    id: Option<String>,
    title: String,
    category: String,
    client: String,
    work_category: String,
    date_text: String,
    tools: String,
    story: String,
    outcome: String,
    image_url: String,
) -> Result<String, ServerFnError> {
    use crate::app::state::AppState;
    use leptos::context::use_context;
    use crate::auth::server::get_auth_user;

    if get_auth_user().is_none() {
        return Err(ServerFnError::ServerError("Unauthorized".into()));
    }

    let app_state = match use_context::<AppState>() {
        Some(state) => state,
        None => return Err(ServerFnError::ServerError("State missing".into())),
    };
    let pool = &app_state.pool;

    if let Some(pid) = id.filter(|s| !s.is_empty()) {
        if let Err(e) = sqlx::query("UPDATE projects SET title=?, category=?, client=?, work_category=?, date_text=?, tools=?, story=?, outcome=?, image_url=? WHERE id=?")
            .bind(title).bind(category).bind(client).bind(work_category).bind(date_text)
            .bind(tools).bind(story).bind(outcome).bind(image_url).bind(&pid)
            .execute(pool).await {
            return Err(ServerFnError::ServerError(e.to_string()));
        }
        Ok(pid)
    } else {
        let new_id = uuid::Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query("INSERT INTO projects (id, title, category, client, work_category, date_text, tools, story, outcome, image_url) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&new_id).bind(title).bind(category).bind(client).bind(work_category).bind(date_text)
            .bind(tools).bind(story).bind(outcome).bind(image_url)
            .execute(pool).await {
            return Err(ServerFnError::ServerError(e.to_string()));
        }
        Ok(new_id)
    }
}

#[component]
pub fn ProjectFormPage() -> impl IntoView {
    use leptos_router::hooks::use_params_map;
    let params = use_params_map();
    let current_id = move || params.get().get("id");

    let save_action = ServerAction::<SaveProject>::new();
    let navigate = use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = save_action.value().get() {
            navigate("/portal-admin", Default::default());
        }
    });

    view! {
        <div class="min-h-screen py-16 text-white max-w-4xl mx-auto px-6">
            <h1 class="font-syne font-bold text-4xl border-b border-border pb-6 mb-10">"Project Details"</h1>
            
                <div class="bg-bg-card p-10 rounded-3xl shadow-xl border border-border flex flex-col gap-6">
                    <ActionForm action=save_action>
                        <input type="hidden" name="id" value=move || current_id().unwrap_or_default() />
                        <div class="flex flex-col gap-6">
                            // Form layout
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div class="flex flex-col gap-2">
                                    <label class="font-syne text-sm text-text-secondary uppercase">"Title"</label>
                                    <input name="title" required class="bg-transparent border-b border-border py-2 focus:border-brand outline-none" placeholder="e.g. Orion UI Kit" />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <label class="font-syne text-sm text-text-secondary uppercase">"Tag (Category)"</label>
                                    <input name="category" required class="bg-transparent border-b border-border py-2 focus:border-brand outline-none" placeholder="e.g. BRANDING" />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <label class="font-syne text-sm text-text-secondary uppercase">"Client"</label>
                                    <input name="client" required class="bg-transparent border-b border-border py-2 focus:border-brand outline-none" placeholder="Client Name" />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <label class="font-syne text-sm text-text-secondary uppercase">"Work Category"</label>
                                    <input name="work_category" required class="bg-transparent border-b border-border py-2 focus:border-brand outline-none" placeholder="e.g. UX/UI Design" />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <label class="font-syne text-sm text-text-secondary uppercase">"Date"</label>
                                    <input name="date_text" required class="bg-transparent border-b border-border py-2 focus:border-brand outline-none" placeholder="March 2026" />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <label class="font-syne text-sm text-text-secondary uppercase">"Tools Used (Comma separated)"</label>
                                    <input name="tools" required class="bg-transparent border-b border-border py-2 focus:border-brand outline-none" placeholder="Figma, Rust, Tailwind" />
                                </div>
                            </div>

                            <div class="flex flex-col gap-2 mt-4">
                                <label class="font-syne text-sm text-text-secondary uppercase">"Image URL"</label>
                                <input name="image_url" required class="bg-transparent border-b border-border py-2 focus:border-brand outline-none" placeholder="https://images.unsplash..." />
                            </div>
                            
                            <div class="flex flex-col gap-2 mt-4">
                                <label class="font-syne text-sm text-text-secondary uppercase">"Project Story"</label>
                                <textarea name="story" required rows="4" class="bg-transparent border-b border-border py-2 focus:border-brand outline-none resize-none"></textarea>
                            </div>
                            
                            <div class="flex flex-col gap-2 mt-4">
                                <label class="font-syne text-sm text-text-secondary uppercase">"Outcome"</label>
                                <textarea name="outcome" required rows="3" class="bg-transparent border-b border-border py-2 focus:border-brand outline-none resize-none"></textarea>
                            </div>

                            <div class="flex justify-end gap-4 mt-8 pt-6 border-t border-border">
                                <a href="/portal-admin" class="px-6 py-3 border border-border text-text-secondary rounded-full font-rubik uppercase text-xs hover:border-white transition-colors">
                                    "Cancel"
                                </a>
                                <button type="submit" class="bg-brand text-white px-8 py-3 rounded-full font-syne font-bold tracking-widest uppercase hover:bg-white hover:text-black transition-colors shadow">
                                    "Save Project"
                                </button>
                            </div>
                        </div>
                    </ActionForm>
                </div>
        </div>
    }
}
