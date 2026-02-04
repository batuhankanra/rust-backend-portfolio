use crate::{database::mongodb::MongoRepo, models::projects::Project};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::bson::doc;
use futures::stream::StreamExt;


pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn create_project(db:&MongoRepo,mut new_project:Project)->Result<InsertOneResult,mongodb::error::Error>{
        new_project.created_at=Some(Utc::now());
        new_project.updated_at=Some(Utc::now());
        db.db
            .collection::<Project>("projects")
            .insert_one(new_project)
            .await
    }
    pub async  fn get_all_project(db:&MongoRepo)->Result<Vec<Project>,mongodb::error::Error>{
        let mut cursors=db.db
            .collection::<Project>("projects").find(doc! {}).await?;
        let mut projectss:Vec<Project>=Vec::new();
        while let Some(result)=cursors.next().await {
            match result {
                Ok(project)=>projectss.push(project),
                Err(e)=>return Err(e)
            }
        }
        Ok(projectss)
    }

    pub async fn get_project_by_id(db:&MongoRepo,id:&str)->Result<Option<Project>,mongodb::error::Error>{
        let obj_id = ObjectId::parse_str(id).map_err(|_| {
            mongodb::error::Error::custom("Geçersiz ID formatı")
        })?;

        let filter = doc! { "_id": obj_id };
        db.db
            .collection::<Project>("projects")
            .find_one(filter)
            .await

    }
}