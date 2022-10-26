
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use serenity::model::application::component::{ActionRowComponent};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::client::Context;
use serenity::model::application::command::{Command};
use mongodb::{Client as MongoClient};
use mongodb::bson::{Document};
use chrono;
use chrono::{NaiveDate, NaiveDateTime};



use serenity::model::application::component::InputTextStyle;
use serenity::model::channel::{ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::channel::ChannelType::{Text, Voice};

use serenity::model::guild::Role;
use serenity::model::id::{GuildId};
use serenity::model::Permissions;
use tokio::join;
use TypeDate::*;
use crate::commands::common_functions::{send_error_from_component, send_error_from_modal};
use crate::commands::constants::*;
use crate::{doc, ORGANISATOR};

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

pub async fn new_edition(command : &ApplicationCommandInteraction, ctx : &Context) {
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
                ORGANISATOR: mci.user.id.0.to_string(),
                EDITION_NAME: doc! {
                        "$eq": nom_competition.value.as_str()
                    }
                }, None
            ).await.unwrap();

            if already_exist.is_some() {
                send_error_from_modal(&mci, &ctx, "Une edition porte déjà ce nom.").await;
                return;
            }

            //TODO setup les permissions pour les catégories
            //     creer la catégorie gimmik
            //     Setup les commandes d'inscriptions et de création d'équipes et de modération dans les salons correspondant !!!!
            //     Setup les probas et temps par défaut

            //setup roles
            let admin_role = create_admin_role(&ctx, &mci);
            let host_role = create_host_role(&ctx, &mci);
            let inscrit_role = create_inscrit_role(&ctx, &mci);
            let roles : (Role, Role, Role) = join!(admin_role, host_role, inscrit_role);

            //setup channels et catégories
            let host_cat = create_host_category(&ctx, &mci);
            let edition_cat = create_edition_category(&ctx, &mci, &nom_competition.value, &roles.2);
            let categories = join!(host_cat, edition_cat);

            //enregisterement infos dan la bdd
            let doc = doc! {
                ORGANISATOR  : organisateur,
                EDITION_NAME : nom_competition.value.as_str(),
                GUILD_ID     : &guild.0.to_string(),
                INSCRIPTION_START_DATE : &timestamps.0,
                INSCRIPTION_END_DATE   : &timestamps.1,
                COMPETITION_START_DATE : &timestamps.2,
                COMPETITION_END_DATE   : &timestamps.3
            };
            let mut edition_result = collection.insert_one(doc, None).await.unwrap();

            let discord_info = doc! {
                EDITION_FILE    : &edition_result.inserted_id.as_object_id(),
                ADMIN_ROLE_ID   : &roles.0.id.0.to_string(),
                HOST_ROLE_ID    : &roles.1.id.0.to_string(),
                INSCRIT_ROLE_ID : &roles.2.id.0.to_string(),
                MODERRATION_CATEGORY_ID : &categories.0.id.to_string(),
                EDITION_CATEGORY_ID     : &categories.1.id.to_string()
            };
            client.database(RAYQUABOT_DB).collection(DISCORD_INFO_COLLECTION).insert_one(discord_info, None).await.expect(format!("Erreur a l'insertion des informations discord pour l'édition {}", &nom_competition.value).as_str());


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

async fn create_admin_role(ctx  : &Context, mci : &ModalSubmitInteraction) -> Role {
    let found = find_role_by_name(&ctx, &mci, ADMIN_ROLE_NAME).await;
    if found.is_err() {
        let role = mci.guild_id.unwrap().create_role(&ctx, |role| {
            role.name(ADMIN_ROLE_NAME)
                .colour(RED_COLOR as u64)
                .hoist(true)
                .position(0)
                .mentionable(true)
                .permissions(Permissions::ADMINISTRATOR)
        }).await.unwrap();
        return role;
    }
    return found.unwrap()
}

async fn create_host_role(ctx  : &Context, mci : &ModalSubmitInteraction) -> Role {
    let found = find_role_by_name(&ctx, &mci, HOST_ROLE_NAME).await;
    if found.is_err() {
        let role = mci.guild_id.unwrap().create_role(&ctx, |role|{
            role.name(HOST_ROLE_NAME)
                .colour(0xff8000) //orange
                .hoist(true)
                .position(0)
                .mentionable(true)
                .permissions(
                    Permissions::MANAGE_NICKNAMES |
                        Permissions::DEAFEN_MEMBERS |
                        Permissions::MANAGE_MESSAGES |
                        Permissions::VIEW_CHANNEL |
                        Permissions::KICK_MEMBERS |
                        Permissions::MENTION_EVERYONE |
                        Permissions::MUTE_MEMBERS |
                        Permissions::MOVE_MEMBERS |
                        Permissions::MODERATE_MEMBERS |
                        Permissions::READ_MESSAGE_HISTORY |
                        Permissions::CONNECT |
                        Permissions::SPEAK |
                        Permissions::STREAM
                )
        }).await.unwrap();
        return role;
    }
    found.unwrap()
}

async fn create_inscrit_role(ctx  : &Context, mci : &ModalSubmitInteraction) -> Role {
    let found = find_role_by_name(&ctx, &mci, INSCRIT_ROLE_NAME).await;
    if found.is_err() {
        let role = mci.guild_id.unwrap().create_role(&ctx, |role|{
            role.name(INSCRIT_ROLE_NAME)
                .colour(0x5E9A78)
                .hoist(true)
                .position(0)
                .mentionable(true)
                .permissions(
                    Permissions::empty()
                )
        }).await.unwrap();
        return role;
    }
    found.unwrap()
}

async fn create_host_category(ctx  : &Context, mci : &ModalSubmitInteraction) -> GuildChannel {
    let res = find_channel_by_name(&ctx, &mci, MODERATION_CATEGORY_NAME).await;

    if res.is_err() {
        let everyone_role = find_role_by_name(&ctx, &mci, EVERYONE_ROLE_NAME);
        let host_role = find_role_by_name(&ctx, &mci, HOST_ROLE_NAME);
        let (everyone_role, host_role) = join!(everyone_role, host_role);
        let perm = vec![
            PermissionOverwrite {
                allow: Permissions::SPEAK,
                deny: Permissions::all(),
                kind: PermissionOverwriteType::Role(everyone_role.unwrap().id)
            },
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL |
                    Permissions::SEND_MESSAGES |
                    Permissions::READ_MESSAGE_HISTORY |
                    Permissions::MOVE_MEMBERS |
                    Permissions::SPEAK |
                    Permissions::CONNECT,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(host_role.unwrap().id)
            }
        ];
        let category = mci.guild_id.unwrap().create_channel(&ctx, |new_cahannel| {
            new_cahannel.kind(ChannelType::Category)
                .name(MODERATION_CATEGORY_NAME)
                .permissions(perm)
        })
            .await.unwrap();
        let perm = vec![];
        let validation = create_channel_in_category("Validation", Text, &category, &ctx, &mci, &perm);
        let discussion = create_channel_in_category("Discussions", Text, &category, &ctx, &mci, &perm);
        let commandes = create_channel_in_category("Commandes", Text, &category, &ctx, &mci, &perm);
        let discussions_bis = create_channel_in_category("Discussions", Voice, &category, &ctx, &mci, &perm);
        let channels = join!(validation, discussion, commandes, discussions_bis);

        return category
    }

    res.unwrap()

    //TODO récupérer les id des channels dans la bdd, récupérer les channels, en faire un tuple, et l'envoyer
}

async fn create_edition_category(ctx  : &Context, mci : &ModalSubmitInteraction, edition : &str, inscrit_role : &Role) -> GuildChannel {
    let res = find_channel_by_name(&ctx, &mci, &edition).await;
    if res.is_err() {
        let category = mci.guild_id.unwrap().create_channel(&ctx, |new_cahannel| {
            new_cahannel.kind(ChannelType::Category)
                .name(&edition)
        })
            .await.unwrap();

        let everyone_role =  find_role_by_name(&ctx, &mci, EVERYONE_ROLE_NAME);
        let host_role = find_role_by_name(&ctx, &mci, HOST_ROLE_NAME);
        let inscrit_role = find_role_by_name(&ctx, &mci, INSCRIT_ROLE_NAME);
        let (everyone_role, host_role, inscrit_role) = join!(everyone_role, host_role, inscrit_role);

        let perms = vec![PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Role(everyone_role.as_ref().unwrap().id)
        }];
        let reglement = create_channel_in_category("Règlement"           , Text, &category, &ctx, &mci, &perms);

        let perms = vec![PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(everyone_role.as_ref().unwrap().id)
        }, PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(host_role.as_ref().unwrap().id)
        }];
        let inscriptions = create_channel_in_category("Inscriptions"        , Text, &category, &ctx, &mci, &perms);

        let perms = vec![PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::all(),
            kind: PermissionOverwriteType::Role(everyone_role.as_ref().unwrap().id)
        }, PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(inscrit_role.as_ref().unwrap().id)
        }, PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::MANAGE_MESSAGES | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(host_role.as_ref().unwrap().id)
        }];
        let equipes = create_channel_in_category("Equipes"             , Text, &category, &ctx, &mci, &perms);

        join!(reglement, inscriptions, equipes);
        return category
    }
    res.unwrap()
}

async fn create_channel_in_category(name : &str, channel_type : ChannelType, category : &GuildChannel, ctx : &Context, mci : &ModalSubmitInteraction, perms: &Vec<PermissionOverwrite>) -> GuildChannel {
    mci.guild_id.unwrap().create_channel(&ctx, |new_cahannel| {
        new_cahannel.kind(channel_type)
            .name(name)
            .category(category.id.0)
            .permissions(perms.to_owned())
    })
        .await.unwrap()
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
