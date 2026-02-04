use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::{database::mongodb::MongoRepo, models::projects::Project, repositories::project_repo::ProjectRepository};




pub async fn list_projects(State(db):State<Arc<MongoRepo>>)->impl IntoResponse {
    match ProjectRepository::get_all_project(&db).await {
        Ok(projects)=>{
            (StatusCode::OK,Json(projects)).into_response()
        },
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,e.to_string()).into_response()
    }
}
pub async fn add_projects(State(db):State<Arc<MongoRepo>>,Json(payload):Json<Project>)->impl IntoResponse{
    match ProjectRepository::create_project(&db, payload).await {
        Ok(result)=>(StatusCode::CREATED,Json(result)).into_response(),
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,e.to_string()).into_response()
    }
}
pub async fn get_project(
    State(db):State<Arc<MongoRepo>>,
    Path(id):Path<String>,
)->impl IntoResponse {
    match ProjectRepository::get_project_by_id(&db, &id).await {
        Ok(Some(project))=>(StatusCode::OK,Json(project)).into_response(),
        Ok(None)=>(StatusCode::NOT_FOUND,"proje bulunamadı").into_response(),
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,e.to_string()).into_response()
    }
}