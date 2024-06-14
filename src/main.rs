// Importa um modulo(pacote) interno, seja por .rs ou folder com esse nome
mod utils;

// Usa crates(bibliotecas) externas
use std::{collections::HashMap, hash::Hash};
use firebase_rs::*;
use serde::{Deserialize, Serialize};

// usa o modulo e funções especificas dele
use utils::functions::{string_to_response, string_to_user};

// Definido uma struct(semelhante a uma classe), #derive é uma macro de atributo
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let user = User {
        name: "nome do usuario".to_string(),
        age: 18,
        email: "email@doUsuario.com".to_string(),
    };

    let firebase = Firebase::new("your-url-from-firebase").unwrap();

    let response = set_user(&firebase, &user).await;

    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User deleted");
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response{
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}",users);
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users");
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}