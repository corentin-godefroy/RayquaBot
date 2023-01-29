use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::component::{ActionRowComponent};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::client::Context;
use serenity::model::application::command::{Command};
use mongodb::{Client as MongoClient, Client};
use mongodb::bson::{Document};
use chrono;
use chrono::{NaiveDate, NaiveDateTime};
use serenity::futures::StreamExt;
use serenity::model::application::component::InputTextStyle;
use serenity::model::channel::{ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::channel::ChannelType::{Text, Voice};
use serenity::model::guild::Role;
use serenity::model::id::{GuildId};
use serenity::model::Permissions;
use tokio::join;
use crate::commands::common_functions::{send_error_from_command, send_error_from_modal};
use crate::commands::constants::*;
use crate::doc;

struct DATE {
    jour : u8,
    mois : u8,
    annee : u16
}

impl ToString for DATE {
    fn to_string(&self) -> String {
        format!("{:02}-{:02}-{:04}", self.jour, self.mois, self.annee)
    }
}

impl Clone for DATE {
    fn clone(&self) -> Self {
        DATE {
            jour: self.jour,
            mois: self.mois,
            annee: self.annee
        }
    }
}

pub async fn new_edition_setup(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command
            .name("new_edition")
            .description("Créé une nouvelle édition.")
            .default_member_permissions(Permissions::ADMINISTRATOR)
    })
        .await.unwrap();
}

pub async fn new_edition(command : &ApplicationCommandInteraction, ctx : &Context, client: &Client) {
    let roles = ctx.http.get_guild(command.guild_id.unwrap().0).await.unwrap();
    let admin_role = roles.role_by_name(ADMIN_ROLE_NAME);
    if admin_role.is_none(){
        send_error_from_command(&command, &ctx, format!("Le rôle **@{}** n'existe pas. Est-tu sûr que le serveur est setup ? Si tu n'est pas sûr, fais --__/setup__**. **IMPORTANT**, __le serveur doit être communautaire__ pour que cette commande fonctionne.", ADMIN_ROLE_NAME).as_str()).await;
        return;
    }
    let admin_role = admin_role.unwrap();
    if !command.member.as_ref().unwrap().roles.contains(&admin_role.id){
        send_error_from_command(&command, &ctx, format!("Tu n'as pas les droits réquis pour cette commande. Seul les **@{}** ont le droit de créer une nouvelle édition.", ADMIN_ROLE_NAME).as_str()).await;
        return;
    }
    
    let collection =  client.database(RAYQUABOT_DB).collection::<Document>(SERVER_COLLECTION);
    let serveur_setup = collection.find_one(
        doc! {
            GUILD_ID : command.guild_id.unwrap().0.to_string()
        },
        None
    ).await.unwrap();
    
    let collection =  client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION);
    let editions = collection.count_documents(
        doc! {
            GUILD_ID : command.guild_id.unwrap().0.to_string(),
            ORGANIZER: command.guild_id.unwrap().0.to_string()
        },
        None
    ).await.unwrap();
    
    if editions >= 25 {
        command.create_interaction_response(&ctx, |response|{
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|{
                    message.embed(|embed|{
                        embed.colour(RED_COLOR)
                            .title("Limite maximale d'édition atteinte !")
                            .description("Tu as atteint la limite de 25 édition planifiable simultanément.")
                    })
                })
        }).await.unwrap();
        return;
    }
    
    if ctx.http.get_guild(command.guild_id.unwrap().0).await.unwrap().channels(&ctx.http).await.unwrap().get(&command.channel_id).unwrap().name != MODERATION_CONVERSATION_CHANNEL_NAME{
        send_error_from_command(command, &ctx, format!("Pour ajouter une édition rends toi dans le salon #{} de la catégorie {}", MODERATION_CONVERSATION_CHANNEL_NAME, MODERATION_CATEGORY_NAME).as_str()).await;
        return;
    }
    
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::Modal)
            .interaction_response_data(|message|
                message.components(|components| {
                    components
                        .create_action_row(|action_row| {
                            action_row.create_input_text(|input_text| {
                                input_text
                                    .custom_id(EDITION_NAME)
                                    .placeholder("Le couple nom/numéro doit être différent des anciennes éditions")
                                    .min_length(4)
                                    .max_length(50)
                                    .required(true)
                                    .label("Nom de l'édition avec un numéro (ou année).")
                                    .style(InputTextStyle::Short)
                            })
                        })
                        .create_action_row(|action_row| {
                            action_row.create_input_text(|input_text| {
                                input_text
                                    .custom_id(CREATE_EDITION_INSCRIPTION_ID)
                                    .placeholder("JJ/MM/AAAA-JJ/MM/AAAA")
                                    .min_length(21)
                                    .max_length(21)
                                    .required(true)
                                    .label("Date de début et de fin des inscription")
                                    .style(InputTextStyle::Short)
                            })
                        })
                        .create_action_row(|action_row| {
                            action_row.create_input_text(|input_text| {
                                input_text
                                    .custom_id(CREATE_EDITION_COMPETITION_ID)
                                    .placeholder("JJ/MM/AAAA-JJ/MM/AAAA")
                                    .min_length(21)
                                    .max_length(21)
                                    .required(true)
                                    .label("Date de début et de fin de la compétition")
                                    .style(InputTextStyle::Short)
                            })
                        })
                })
                    .title("Dates de la compétition")
                    .custom_id(CREATE_NEW_EDITION)
            )
    })
        .await
        .expect("Failed to send interaction response");
}

pub async fn new_edition_modal(client : &MongoClient, mci : ModalSubmitInteraction, ctx : serenity::client::Context) {
    let nom_competition = match mci
        .data
        .components
        .get(0)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it,
        _ => return,
    };

    let date_inscription = match mci
        .data
        .components
        .get(1)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it,
        _ => return,
    };

    let date_competition = match mci
        .data
        .components
        .get(2)
        .unwrap()
        .components
        .get(0)
        .unwrap()
    {
        ActionRowComponent::InputText(it) => it,
        _ => return,
    };

    let date_inscription = parse_two_dates(date_inscription.value.as_str());
    let date_competition = parse_two_dates(date_competition.value.as_str());
    let guild = mci.guild_id.unwrap();
    let organisateur = mci.user.id.0.to_string();

    let date_ins = match_dates(date_inscription, &mci, &ctx).await.unwrap();
    let date_comp = match_dates(date_competition, &mci, &ctx).await.unwrap();

    let timestamp_debut_inscription = get_timestamp_from_date(&date_ins.0 ,  &mci, &ctx);
    let timestamp_fin_inscription   = get_timestamp_from_date(&date_ins.1 , &mci, &ctx);
    let timestamp_debut_competition = get_timestamp_from_date(&date_comp.0, &mci, &ctx);
    let timestamp_fin_competition   = get_timestamp_from_date(&date_comp.1, &mci, &ctx);

    let timestamps = join!(timestamp_debut_inscription, timestamp_fin_inscription, timestamp_debut_competition, timestamp_fin_competition);

    if timestamps.3 < chrono::Utc::now().timestamp() {
        send_error_from_modal(&mci, &ctx, "Tu ne peut pas ajouter d'édition passée.").await;
        return;
    }

    let resultat = edition_overlap_check(&timestamps.0, &timestamps.3, &client, &guild).await;

    match resultat {
        Ok(_) => {
            let collection =  client.database(RAYQUABOT_DB).collection(EDITIONS_COLLECTION);
            
            let already_exist = collection.find_one(
                doc! {
                    ORGANIZER: mci.user.id.0.to_string(),
                    GUILD_ID: mci.guild_id.unwrap().0.to_string(),
                    EDITION_NAME: nom_competition.value.as_str()
                }, None
            ).await.unwrap();

            if already_exist.is_some() {
                send_error_from_modal(&mci, &ctx, "Une edition porte déjà ce nom.").await;
                return;
            }

            //enregisterement infos dan la bdd
            let doc = doc! {
                ORGANIZER    : organisateur,
                EDITION_NAME : nom_competition.value.as_str(),
                GUILD_ID     : &guild.0.to_string(),
                INSCRIPTION_START_DATE : &timestamps.0,
                INSCRIPTION_END_DATE   : &timestamps.1,
                COMPETITION_START_DATE : &timestamps.2,
                COMPETITION_END_DATE   : &timestamps.3,
                BDD_POKE_RED_GREEN_BLUE              : 0,
                BDD_POKE_YELLOW                      : 0,
                BDD_POKE_GOLD_SILVER                 : 0,
                BDD_POKE_CRYSTAL                     : 0,
                BDD_POKE_RUBY_SAPPHIRE               : 0,
                BDD_POKE_FIRERED_LEAFGREEN           : 0,
                BDD_POKE_EMERALD                     : 0,
                BDD_POKE_DIAMOND_PEARL               : 0,
                BDD_POKE_PLATINUM                    : 0,
                BDD_POKE_HEARTGOLD_SOULSILVER        : 0,
                BDD_POKE_BLACK_WHITE                 : 0,
                BDD_POKE_BLACK2_WHITE2               : 0,
                BDD_POKE_X_Y                         : 0,
                BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE   : 0,
                BDD_POKE_SUN_MOON                    : 0,
                BDD_POKE_ULTRASUN_ULTRAMOON          : 0,
                BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE   : 0,
                BDD_POKE_SWORD_SHIELD                : 0,
                BDD_POKE_BRILLANTDIAMOND_SHININGPEARL: 0,
                BDD_POKE_LEGENDARCEUS                : 0,
                BDD_POKE_SCARLET_VIOLET              : 0,
                BDD_POKE_DONJON_MYSTERE              : 0,
                BDD_POKE_COLOSEUM                    : 0,
                BDD_POKE_STADIUM_EU                  : 0,
                BDD_POKE_STADIUM_JAP                 : 0,
                BDD_POKE_STADIUM_2                   : 0,
                BDD_POKE_XD                          : 0
            };
            collection.insert_one(doc, None).await.unwrap();

            mci.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message|
                        message.embed(|embed| {
                            embed.title(&nom_competition.value.to_string())
                                .description(format!("Voici les informations de l'édition {}", &nom_competition.value.to_string()))
                                .field("Date de début des inscriptions" , &format!("{}", NaiveDateTime::from_timestamp(timestamps.0, 0).format("%d/%m/%Y")), false)
                                .field("Date de fin des inscriptions"   , &format!("{}", NaiveDateTime::from_timestamp(timestamps.1, 0).format("%d/%m/%Y")), false)
                                .field("Date de début de la compétition", &format!("{}", NaiveDateTime::from_timestamp(timestamps.2, 0).format("%d/%m/%Y")), false)
                                .field("Date de fin de la compétition"  , &format!("{}", NaiveDateTime::from_timestamp(timestamps.3, 0).format("%d/%m/%Y")), false)
                                .field("Que faire ensuite ?",
                                       "- Modifier les valeurs de probabilité en faisant \"/edit_methode\"\n\
                                       - Modifier les valeurs de temps en faisant \"/edit_time\"\n",
                                false
                                )
                                .color(GREEN_COLOR)
                        })
                    )
            })
                .await
                .expect("Failed to send inteaction response");
        }
        Err(e) => send_error_from_modal(&mci, &ctx, &e).await
    }
}

async fn find_role_by_name(ctx  : &Context, mci : &ModalSubmitInteraction, name : &str) -> Result<Role, bool> {
    let roles = mci.guild_id.unwrap().roles(&ctx).await.unwrap();
    for role in roles {
        if role.1.name == name{
            return Ok(role.1);
        }
    }
    return Err(false)
}

async fn find_channel_by_name(ctx : &Context, mci : &ModalSubmitInteraction, name : &str) -> Result<GuildChannel, bool> {
    let channels = mci.guild_id.unwrap().channels(&ctx).await.unwrap();
    for channel in channels {
        if channel.1.name.eq(&name) {
            return Ok(channel.1);
        }
    }
    Err(false)
}

async fn match_dates(date : Result<(DATE, DATE), String>, mci : &ModalSubmitInteraction, ctx: &Context) -> Result<(DATE, DATE), String> {
    return match date {
        Ok(ref dates) => { Ok(dates.to_owned()) }
        Err(ref e) => {
            send_error_from_modal(&mci, &ctx, &e).await;
            Err(e.to_string())
        }
    }
}

fn parse_two_dates(date : &str) -> Result<(DATE, DATE), String> {
    let dates : Vec<String> = date.split("-").map(|s| s.to_string()).collect();
    if dates.len() != 2 { return Err(format!("les dates {} sont mal écrites. Respecte bien le format JJ/MM/AAAA-JJ/MM/AAAA", date).to_string())}

    let date1 = dates.get(0).unwrap();
    let date1 = parse_one_date(date1).unwrap();


    let date2= dates.get(1).unwrap();
    let date2 = parse_one_date(date2).unwrap();
    Ok((date1, date2))
}

fn parse_one_date(date : &str) -> Result<DATE, String> {
    let date : Vec<String> = date.split("/").map(|s| s.to_string()).collect();
    if date.len() != 3 {return Err("La date n'est pas correctement écrite. Respecte l'écriture suivante : JJ/MM/AAAA-JJ/MM/AAAA".to_string());}
    let date = DATE {
        jour : date[0].parse::<u8>().unwrap(),
        mois : date[1].parse::<u8>().unwrap(),
        annee : date[2].parse::<u16>().unwrap()
    };
    Ok(date)
}

async fn edition_overlap_check(timestamp_debut : &i64, timestamp_fin : &i64, client : &MongoClient, guild_id : &GuildId) ->Result<(), String>{
    if timestamp_debut.to_owned() == 0 || timestamp_fin.to_owned() == 0 {
        return if timestamp_fin.to_owned() == 0 {
            Err("la date de fin n'est pas valide !".to_string())
        } else {
            Err("la date de début n'est pas valide !".to_string())
        }
    }

    if timestamp_fin.to_owned() < timestamp_debut.to_owned() {
        return Err("La date de fin est avant la date de début !".to_string())
    }
    let querry = doc! {
        GUILD_ID: &guild_id.0.to_string(),
        "$nor": [
            doc! { INSCRIPTION_START_DATE: doc! {"$gt": timestamp_fin}},
            doc! { INSCRIPTION_END_DATE  : doc! {"$lt": timestamp_debut}}
        ]
    };

    let count = client.database(RAYQUABOT_DB).collection::<Document>(EDITIONS_COLLECTION).count_documents(querry,None).await.unwrap();

    if count > 0 {
        return Err("L'édition chevauche une édition existante !".to_string());
    }
    return Ok(());
}

async fn get_timestamp_from_date(date : &DATE, msi: &ModalSubmitInteraction, ctx: &Context) -> i64 {
    match NaiveDate::from_ymd_opt(date.annee as i32, date.mois as u32, date.jour as u32) {
        Some(date) => {
            let date = date.and_hms(0, 0, 0);
            let timestamp = date.timestamp();
            timestamp
        },
        None => {
             send_error_from_modal(&msi, &ctx, &format!("La date \"{}\" entrée est invalide", &date.to_string() ).as_str()).await;
            return 0
        }
    }
}
