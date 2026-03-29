use leptos::prelude::*;

#[server(CheckAuth, "/api")]
pub async fn check_auth() -> Result<String, ServerFnError> {
    use crate::auth::server::get_auth_user;
    if let Some(user) = get_auth_user() {
        Ok(user)
    } else {
        Err(ServerFnError::ServerError("Unauthorized".into()))
    }
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth = Resource::new(|| (), |_| async move { check_auth().await });
    let projects_resource = Resource::new(|| (), |_| async move { crate::models::get_projects().await.unwrap_or_default() });
    let delete_action = ServerAction::<crate::models::DeleteProject>::new();

    view! {
        <Transition fallback=|| view! { <div class="text-white text-center py-20 font-syne">"Loading Dashboard..."</div> }>
            {move || {
                match auth.get() {
                    Some(Ok(user)) => view! {
                        <div class="min-h-screen text-white py-16 max-w-7xl mx-auto px-6">
                            <div class="flex justify-between items-center border-b border-border pb-8 mb-10">
                                <div>
                                    <h1 class="font-syne font-bold text-4xl">"Dashboard"</h1>
                                    <p class="text-text-secondary font-rubik mt-2">"Welcome back, " {user}</p>
                                </div>
                                <a href="/portal-admin/new" class="bg-brand text-white px-6 py-3 rounded-full font-syne font-bold text-sm uppercase tracking-widest hover:bg-white hover:text-black transition-colors">
                                    "Add Project"
                                </a>
                            </div>

                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                                <Transition fallback=|| view! { <div class="col-span-full text-center text-text-muted py-10 font-syne">"Loading Projects..."</div> }>
                                    {move || {
                                        view! {
                                            <>
                                            {move || projects_resource.get().unwrap_or_default().into_iter().map(|proj| {
                                                view! {
                                                    <div class="bg-bg-card rounded-3xl overflow-hidden shadow-lg border border-border flex flex-col group relative">
                                                        <div class="h-48 w-full relative">
                                                            <img src=proj.image_url class="w-full h-full object-cover grayscale group-hover:grayscale-0 transition-all duration-500" />
                                                            <div class="absolute inset-0 bg-black/40"></div>
                                                            <div class="absolute bottom-4 left-4 flex flex-col">
                                                                <span class="text-white font-syne font-bold text-xl">{proj.title.clone()}</span>
                                                                <span class="text-brand font-rubik text-xs uppercase tracking-widest">{proj.category.clone()}</span>
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="p-6 flex justify-between items-center bg-bg-card border-t border-border">
                                                            <a href=format!("/portal-admin/edit/{}", proj.id) class="text-sm border border-border px-4 py-2 hover:bg-white hover:text-black transition-colors rounded-full font-rubik uppercase">
                                                                "Edit"
                                                            </a>
                                                            <ActionForm action=delete_action>
                                                                <input type="hidden" name="id" value=proj.id />
                                                                <button type="submit" class="text-sm border border-red-500/50 text-red-500 px-4 py-2 hover:bg-red-500 hover:text-white transition-colors rounded-full font-rubik uppercase">
                                                                    "Delete"
                                                                </button>
                                                            </ActionForm>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                            </>
                                        }
                                    }}
                                </Transition>
                            </div>
                        </div>
                    }.into_any(),
                    _ => view! {
                        <div class="text-center py-32 text-red-500 font-syne font-bold text-2xl">
                            "Unauthorized Access. Redirecting..."
                            <script>
                                "setTimeout(() => window.location.href = '/portal-admin-login', 2000);"
                            </script>
                        </div>
                    }.into_any()
                }
            }}
        </Transition>
    }
}
