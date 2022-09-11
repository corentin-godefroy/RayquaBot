pub mod mongo_functions {
    use std::borrow::Borrow;
    use std::ops::{Deref, DerefMut};
    use lazy_static::lazy_static;
    use mongodb::Client;
    use mongodb::options::ClientOptions;
    use serenity::client::Context;
    use serenity::framework::standard::macros::command;
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use crate::{doc, MongoClient, MongoClientOptions, ping_reactor};

    pub async fn new_edition_insertion(client : &Client, command : ApplicationCommandInteraction, ctx : Context) {
        let name = command.data.options[0].value.as_ref().unwrap().as_str().unwrap();
        let number = command.data.options[1].value.as_ref().unwrap().as_str().unwrap();
        let edition = name.to_string() + " " + &*number.to_string();
        let collection =  client.database("RayquaBot").collection("editions");
        let doc = doc! {
            "edition": edition
        };
        collection.insert_one(doc, None).await.expect("Failed to insert document");
    }
}