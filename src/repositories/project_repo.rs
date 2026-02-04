use crate::{database::mongodb::MongoRepo, models::projects::Project};
use mongodb::results::InsertOneResult;
use mongodb::bson::doc;
use futures::stream::StreamExt;


pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn create_project(db:&MongoRepo,new_project:Project)->Result<InsertOneResult,mongodb::error::Error>{
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
}