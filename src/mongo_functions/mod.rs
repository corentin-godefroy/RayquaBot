pub mod mongo_functions {
    use mongodb::Client;
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use crate::doc;

    pub async fn new_edition_insertion(client : &Client, command : &ApplicationCommandInteraction) {
        let name = command.data.options.get(0).unwrap().value.as_ref().unwrap().to_string();
        let numero = command.data.options.get(1).unwrap().value.as_ref().unwrap().to_string();
        let edition = name.to_string() + " " + &*numero.to_string();
        let organisateur = command.user.id.0.to_string();

        let collection =  client.database("RayquaBot").collection("editions");
        let doc = doc! {
            "organisateur" : organisateur,
            "edition": edition
        };
        collection.insert_one(doc, None).await.expect("Failed to insert document");
    }
}