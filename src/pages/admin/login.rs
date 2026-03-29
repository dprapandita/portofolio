use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[server(AdminLogin, "/api")]
pub async fn admin_login(username: String, password: String) -> Result<String, ServerFnError> {
    use crate::app::state::AppState;
    use bcrypt::verify;
    use crate::auth::server::set_auth_cookie;

    let app_state = match leptos::context::use_context::<AppState>() {
        Some(state) => state,
        None => return Err(ServerFnError::ServerError("State missing".into())),
    };
    let pool = &app_state.pool;

    let user_res: Result<Option<(String,)>, _> = sqlx::query_as("SELECT password_hash FROM users WHERE username = ?")
        .bind(&username)
        .fetch_optional(pool)
        .await;
        
    let user = match user_res {
        Ok(u) => u,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    if let Some((hash,)) = user {
        // In a real high-traffic app, bcrypt should run in spawn_blocking
        if verify(&password, &hash).unwrap_or(false) {
            set_auth_cookie(&username);
            return Ok("Success".to_string());
        }
    }

    Err(ServerFnError::ServerError("Invalid credentials".to_string()))
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<AdminLogin>::new();
    let navigate = use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = login_action.value().get() {
            navigate("/portal-admin", Default::default());
        }
    });

    view! {
        <div class="min-h-[80vh] flex flex-col justify-center items-center py-16 text-white bg-bg-dark">
            <div class="w-full max-w-md bg-bg-card p-12 rounded-3xl shadow-2xl border border-border">
                <div class="text-center mb-8 flex flex-col items-center">
                    <div class="flex items-center gap-1">
                        <span class="text-3xl font-syne font-bold tracking-widest uppercase">"MY"</span>
                        <span class="text-3xl font-syne font-bold outline-text text-transparent" style="-webkit-text-stroke: 1px #fff;">"SELF"</span>
                    </div>
                    <p class="text-text-secondary mt-2 font-rubik text-sm tracking-widest uppercase">"Secure Access"</p>
                </div>
                
                <div class="flex flex-col gap-6">
                    <ActionForm action=login_action>
                        <div class="flex flex-col gap-6">
                            <div class="flex flex-col gap-2">
                                <label class="font-syne uppercase tracking-wider text-xs font-bold text-text-secondary">"Username"</label>
                                <input type="text" name="username" required
                                       class="bg-transparent border-b border-border outline-none py-3 text-white focus:border-brand transition-colors font-rubik" 
                                       placeholder="Admin ID" />
                            </div>
                            
                            <div class="flex flex-col gap-2">
                                <label class="font-syne uppercase tracking-wider text-xs font-bold text-text-secondary">"Password"</label>
                                <input type="password" name="password" required
                                       class="bg-transparent border-b border-border outline-none py-3 text-white focus:border-brand transition-colors font-rubik" 
                                       placeholder="••••••••" />
                            </div>

                            <Show when=move || login_action.pending().get() fallback=|| view!{}>
                                <div class="text-center text-xs text-brand mt-2 font-rubik">"Verifying..."</div>
                            </Show>

                            <Show when=move || login_action.value().get().is_some() fallback=|| view!{}>
                                {move || match login_action.value().get() {
                                    Some(Err(_)) => view! { <div class="text-center text-xs text-red-500 font-rubik mt-2">"Access Denied"</div> }.into_any(),
                                    _ => view! {}.into_any(),
                                }}
                            </Show>
                            
                            <div class="mt-6 flex justify-center">
                                <button type="submit" class="bg-brand text-white font-syne uppercase tracking-wider text-sm font-bold w-full py-4 rounded-full hover:bg-white hover:text-black transition-colors duration-300">
                                    "Enter Portal"
                                </button>
                            </div>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
}
