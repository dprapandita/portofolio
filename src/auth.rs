#[cfg(feature = "ssr")]
pub mod server {
    use axum::http::header::SET_COOKIE;
    use axum::http::HeaderValue;
    use axum::http::request::Parts;
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use leptos_axum::ResponseOptions;

    pub fn set_auth_cookie(username: &str) {
        let cookie = Cookie::build(("admin_session", username.to_string()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Strict)
            .build();
            
        if let Some(res) = leptos::context::use_context::<ResponseOptions>() {
            res.insert_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());
        }
    }

    pub fn get_auth_user() -> Option<String> {
        let req = leptos::context::use_context::<Parts>()?;
        let cookies = req.headers.get("cookie")?.to_str().ok()?;
        
        for cookie_str in cookies.split(';') {
            let trimmed = cookie_str.trim();
            if trimmed.starts_with("admin_session=") {
                let user = trimmed["admin_session=".len()..].to_string();
                if !user.is_empty() {
                    return Some(user);
                }
            }
        }
        None
    }

    pub fn logout() {
        let cookie = Cookie::build(("admin_session", ""))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Strict)
            // Invalidating the cookie
            .build();
            
        if let Some(res) = leptos::context::use_context::<ResponseOptions>() {
            res.insert_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());
        }
    }
}
