mod meta_extrator;
pub mod recipe_dao;
pub mod types;
pub mod utils;

use std::env;

use chrono::Utc;
use meta_extrator::extract_from_url;
use recipe_dao::{MongoRecipeDao, RecipeDao};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, fs::FileServer, get, launch, post, routes};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use types::recipe::Recipe;

#[get("/")]
async fn get_index() -> Template {
    return Template::render("index", context! {});
}

#[post("/search")]
async fn search() -> Template {
    return Template::render("search-results", context! {});
}
#[derive(Debug, Deserialize, Clone)]
struct CreateRecipe {
    url: String,
}

#[post("/recipe", data = "<recipe>")]
async fn add_recipe(recipe: Json<CreateRecipe>) -> Result<String, Status> {
    let dao = MongoRecipeDao {
        uri: env::var("db_uri").unwrap(),
    };
    if !recipe.url.starts_with("http") {
        return Ok(String::from(
            "<p>invalid url, please enter a valid one with http or https</p>",
        ));
    }

    let mt = extract_from_url(&recipe.url).await;
    let data = Recipe {
        id: None,
        url: recipe.url.clone(),
        preview_image: Some(mt.image),
        title: Some(mt.title),
        added: Utc::now(),
    };
    let res = dao.insert_recipe(data).await;
    if res.is_err() {
        return Err(Status::InternalServerError);
    }
    return Ok(res.unwrap());
}

#[get("/recipe")]
async fn get_recipes() -> Template {
    let dao = MongoRecipeDao {
        uri: env::var("db_uri").unwrap(),
    };
    let data = dao.get_recipe(None).await;
    println!("Receipe: {:?}", data);
    Template::render(
        "recipes",
        context! {nothing: data.len() == 0, recipes: data},
    )
}

#[delete("/recipe/<id>")]
async fn delete_recipe(id: String) -> Status {
    let dao = MongoRecipeDao {
        uri: env::var("db_uri").unwrap(),
    };
    let data = dao.delete_recipe(id).await;
    if data.is_err() {
        return Status::InternalServerError;
    }
    return Status::Ok;
}

#[launch]
fn rocket() -> _ {
    if env::var("db_uri").is_err() {
        panic!("Need to set db_uri in env.")
    }

    rocket::build()
        .mount(
            "/",
            routes![get_index, search, add_recipe, get_recipes, delete_recipe],
        )
        .mount("/public", FileServer::from("public"))
        .attach(Template::fairing())
}
