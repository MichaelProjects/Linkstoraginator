use chrono::{DateTime, Utc};
use futures::{StreamExt, TryStreamExt};
use mongodb::{bson::doc, Client, Collection};
use rocket::async_trait;
use serde::Serialize;

use crate::types::recipe::Recipe;
use crate::utils::parse_and_describe;
#[async_trait]
pub trait RecipeDao {
    async fn insert_recipe(&self, recipe: Recipe) -> Result<String, anyhow::Error>;
    async fn get_recipe(&self, id: Option<String>) -> Vec<ReadableRecipe>;
    async fn delete_recipe(&self, id: String) -> Result<(), anyhow::Error>;
}

pub struct MongoRecipeDao {
    pub uri: String,
}

async fn get_collection(uri: &String, collection_name: &String) -> Collection<Recipe> {
    let client = Client::with_uri_str(uri).await.unwrap();
    client
        .database("recipe_project")
        .collection(collection_name)
}

#[derive(Debug, Serialize)]
pub struct ReadableRecipe {
    pub id: Option<String>,
    pub url: String,
    pub preview_image: Option<String>,
    pub title: Option<String>,
    pub added: DateTime<Utc>,
    pub humanioid_added: String,
}
impl ReadableRecipe {
    fn new(r: Recipe) -> ReadableRecipe {
        return ReadableRecipe {
            id: r.id,
            url: r.url,
            preview_image: r.preview_image,
            title: r.title,
            added: r.added.clone(),
            humanioid_added: parse_and_describe(r.added, chrono_tz::Tz::Europe__Berlin).unwrap(),
        };
    }
}

#[async_trait]
impl RecipeDao for MongoRecipeDao {
    async fn insert_recipe(&self, recipe: Recipe) -> Result<String, anyhow::Error> {
        let collection = get_collection(&self.uri, &String::from("recipe")).await;
        let res = collection.insert_one(recipe, None).await?;
        Ok(res.inserted_id.to_string())
    }
    async fn get_recipe(&self, id: Option<String>) -> Vec<ReadableRecipe> {
        let collection = get_collection(&self.uri, &String::from("recipe")).await;
        if id.is_none() {
            let mut cursor = collection.find(doc! {}, None).await.unwrap();
            let mut recipes: Vec<ReadableRecipe> = vec![];
            while let Some(data) = cursor.try_next().await.unwrap() {
                recipes.push(ReadableRecipe::new(data))
            }
            recipes.reverse();
            return recipes;
        }

        return vec![];
    }
    async fn delete_recipe(&self, id: String) -> Result<(), anyhow::Error> {
        let collection = get_collection(&self.uri, &String::from("recipe")).await;
        let res = collection.delete_one(doc! {"_id": &id}, None).await;
        if res?.deleted_count != 1 {
            return Err(anyhow::Error::msg(format!(
                "Could not find recipe to delete for: {}",
                id
            )));
        }
        return Ok(());
    }
}
