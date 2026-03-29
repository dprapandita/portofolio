use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub category: String,
    pub client: String,
    pub work_category: String,
    pub date_text: String,
    pub tools: String,
    pub story: String,
    pub outcome: String,
    pub image_url: String,
}

use leptos::prelude::*;

#[server(GetProjects, "/api")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use crate::app::state::AppState;
    use leptos::context::use_context;
    use sqlx::Row;

    let app_state = match use_context::<AppState>() {
        Some(state) => state,
        None => return Err(ServerFnError::ServerError("State missing".into())),
    };
    let pool = &app_state.pool;

    let rows = match sqlx::query("SELECT * FROM projects ORDER BY created_at DESC").fetch_all(pool).await {
        Ok(res) => res,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    let projects = rows.into_iter().map(|row| {
        Project {
            id: row.get("id"),
            title: row.get("title"),
            category: row.get("category"),
            client: row.get("client"),
            work_category: row.get("work_category"),
            date_text: row.get("date_text"),
            tools: row.get("tools"),
            story: row.get("story"),
            outcome: row.get("outcome"),
            image_url: row.get("image_url"),
        }
    }).collect();

    Ok(projects)
}

#[server(GetProject, "/api")]
pub async fn get_project(id: String) -> Result<Option<Project>, ServerFnError> {
    use crate::app::state::AppState;
    use leptos::context::use_context;
    use sqlx::Row;

    let app_state = match use_context::<AppState>() {
        Some(state) => state,
        None => return Err(ServerFnError::ServerError("State missing".into())),
    };
    let pool = &app_state.pool;

    let row_opt = match sqlx::query("SELECT * FROM projects WHERE id=?")
        .bind(id)
        .fetch_optional(pool)
        .await {
            Ok(res) => res,
            Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    Ok(row_opt.map(|row| Project {
        id: row.get("id"),
        title: row.get("title"),
        category: row.get("category"),
        client: row.get("client"),
        work_category: row.get("work_category"),
        date_text: row.get("date_text"),
        tools: row.get("tools"),
        story: row.get("story"),
        outcome: row.get("outcome"),
        image_url: row.get("image_url"),
    }))
}

#[server(DeleteProject, "/api")]
pub async fn delete_project(id: String) -> Result<(), ServerFnError> {
    use leptos::context::use_context;
    use crate::app::state::AppState;
    use crate::auth::server::get_auth_user;

    if get_auth_user().is_none() {
         return Err(ServerFnError::ServerError("Unauthorized".into()));
    }

    let app_state = match use_context::<AppState>() {
        Some(state) => state,
        None => return Err(ServerFnError::ServerError("State missing".into())),
    };
    if let Err(e) = sqlx::query("DELETE FROM projects WHERE id=?")
        .bind(id)
        .execute(&app_state.pool)
        .await {
        return Err(ServerFnError::ServerError(e.to_string()));
    }
    Ok(())
}
