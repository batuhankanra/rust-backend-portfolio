use crate::error::AppError;
use crate::{database::mongodb::MongoRepo, models::projects::Project};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::bson::doc;


pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn update_project(db:&MongoRepo,id:&str,updated_data:Project)->Result<UpdateResult,AppError>{
        let obj_id=ObjectId::parse_str(id).map_err(|_| AppError::InvalidId(id.to_string()))?;
        let update_doc=doc! {
            "$set":{
                "title": updated_data.title,
                "description": updated_data.description,
                "tech_stack": updated_data.tech_stack,
                "github_url": updated_data.github_url,
                "live_url": updated_data.live_url,
            }
        };
        let res=db.db
            .collection::<Project>("projects")
            .update_one(doc! {"_id":obj_id}, update_doc)
            .await?;
        if res.matched_count==0{
            return Err(AppError::NotFound);
        }
        Ok(res)
    }
    pub async fn delete_project(db:&MongoRepo,id:&str)->Result<DeleteResult,AppError>{
        let obj_id=ObjectId::parse_str(id).map_err(|_| AppError::InvalidId(id.to_string()))?;
        let filter=doc! {"_id":obj_id};
        let res=db.db
            .collection::<Project>("projects")
            .delete_one(filter)
            .await?;
        if res.deleted_count==0{
            return Err(AppError::NotFound);
        }
        Ok(res)
    }
    pub async fn create_project(db:&MongoRepo,mut new_project:Project)->Result<InsertOneResult,AppError>{
        new_project.created_at=Some(Utc::now());
        new_project.updated_at=Some(Utc::now());
        let res=db.db
            .collection::<Project>("projects")
            .insert_one(new_project)
            .await?;
        Ok(res)
    }
    pub async  fn get_all_project(db:&MongoRepo)->Result<Vec<Project>,AppError>{
        let cursors=db.db
            .collection::<Project>("projects")
            .find(doc! {}).await?;
        let projectss:Vec<Project>=cursors.try_collect().await?;
       
        Ok(projectss)
    }

    pub async fn get_project_by_id(db:&MongoRepo,id:&str)->Result<Project,AppError>{
        let obj_id = ObjectId::parse_str(id).map_err(|_| AppError::InvalidId(id.to_string()))?;

        let filter = doc! { "_id": obj_id };
        let project=db.db
            .collection::<Project>("projects")
            .find_one(filter)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(project)

    }
}