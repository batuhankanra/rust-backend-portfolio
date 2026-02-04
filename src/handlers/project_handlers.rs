use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::{database::mongodb::MongoRepo, error::{AppError, MyJson}, models::projects::Project, repositories::project_repo::ProjectRepository};




pub async fn list_projects(State(db):State<Arc<MongoRepo>>)->Result<impl IntoResponse,AppError> {
    let projects=ProjectRepository::get_all_project(&db).await?;
    Ok((StatusCode::OK,Json(projects)))
}
pub async fn add_projects(State(db):State<Arc<MongoRepo>>,MyJson(payload):MyJson<Project>)->Result<impl IntoResponse,AppError>{
    let res=ProjectRepository::create_project(&db, payload).await?;
    Ok((StatusCode::CREATED,Json(res)))
}
pub async fn get_project(
    State(db):State<Arc<MongoRepo>>,
    Path(id):Path<String>,
)->Result<impl IntoResponse,AppError> {
    let project =ProjectRepository::get_project_by_id(&db, &id).await?;
    Ok(Json(project))
}

pub async fn delete_project(
    State(db):State<Arc<MongoRepo>>,
    Path(id):Path<String>
)->Result<impl IntoResponse,AppError>  {
    let project=ProjectRepository::delete_project(&db, &id).await?;
    Ok((StatusCode::OK,Json(project)))
}

pub async fn update_project(
    State(db):State<Arc<MongoRepo>>,
    Path(id):Path<String>,
    Json(payload):Json<Project>)->Result<impl IntoResponse,AppError>{
        let res=ProjectRepository::update_project(&db, &id, payload).await?;
        Ok((StatusCode::OK,Json(res)))
    }