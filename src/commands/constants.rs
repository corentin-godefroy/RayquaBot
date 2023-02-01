pub const VALUE_START_REGISTRATION: &str = "edit_start_registration";
pub const VALUE_END_REGISTRATION: &str = "edit_end_registration";
pub const VALUE_START_COMPETITION: &str = "edit_start_competition";
pub const VALUE_END_COMPETITION: &str = "edit_end_competition";
pub const CREATE_NEW_EDITION: &str = "create_new_edition";
pub const CREATE_EDITION_INSCRIPTION_ID: &str = "inscription";
pub const CREATE_EDITION_COMPETITION_ID: &str = "competition";

pub const PING: &str = "ping";
pub const NEW_EDITION: &str = "new_edition";
pub const DELETE_EDITION: &str = "delete_edition";
pub const EDIT_EDITION: &str = "edit_edition";
pub const GET_EDITION: &str = "get_edition";
pub const REGISTRATION: &str = "inscription";
pub const ADD_VERSION: &str = "add_version";
pub const ADD_NAMES: &str = "add_names";
pub const VALIDATE: &str = "validate";
pub const VERSION_SETUP: &str = "version_setup";
pub const PRINT_VERSIONS: &str = "print_versions";


pub const DELETE_EDITION_MODAL: &str = "delete_edition_modal";
pub const LOCK_VERSION_MODAL: &str = "lock_version_modal";
pub const EDIT_START_INSCRIPTIONS: &str = "edit_start_inscription";
pub const EDIT_END_INSCRIPTIONS: &str = "edit_end_inscriptions";
pub const EDIT_START_COMPETITION: &str = "edit_start_competition";
pub const EDIT_END_COMPETITION: &str = "edit_end_competition";
pub const PRINT_VERSIONS_MODAL: &str = "print_versions_modal";

pub const EDIT_START_EDITION_END: &str = "edit_start_inscriptions_end"; // ?????

pub const EDITIONS_COLLECTION: &str = "editions";
pub const SERVER_COLLECTION: &str = "servers";
pub const PLAYER_COLLECTION: &str = "players";
pub const CAPTURES_COLLECTION: &str = "captures";
pub const RAYQUABOT_DB: &str = "RayquaBot";
pub const INSCRIPTION_START_DATE: &str = "inscription_start_date";
pub const INSCRIPTION_END_DATE: &str = "inscription_end_date";
pub const COMPETITION_START_DATE: &str = "competition_start_date";
pub const COMPETITION_END_DATE: &str = "competition_end_date";
pub const GUILD_ID: &str = "guild_id";
pub const EDITION_NAME: &str = "edition_name";
pub const ORGANIZER: &str = "organizer";

pub const EDITION_FILE: &str = "edition_file";
pub const ADMIN_ROLE_ID: &str = "admin_role_id";
pub const MODERATOR_ROLE_ID: &str = "moderator_role_id";
pub const HOST_ROLE_ID: &str = "host_role_id";
pub const REGISTERED_ROLE_ID: &str = "registered_role_id";
pub const VERIFIED_ROLE_ID: &str = "verified_role_id";
pub const MODERATION_CATEGORY_ID: &str = "moderation_category_id";
pub const COMPETITION_CATEGORY_ID: &str = "edition_category_id";
pub const REGISTRATION_CHANNEL_ID: &str = "registration_channel_id";

pub const RED_COLOR: i32 = 0xff0000;
pub const GREEN_COLOR: i32 = 0x00ff00;
pub const BLUE_COLOR: i32 = 0x0000ff;
pub const LIGHT_BLUE_COLOR: i32 = 0x04EEE6;
pub const CONTACT: &str = "contact.cgbots@gmail.com";

pub const START_TYPE_REGISTRATION: &str = "debut d'inscriptions";
pub const END_TYPE_REGISTRATION: &str = "fin d'inscriptions";
pub const START_TYPE_COMPETITION: &str = "debut de compétition";
pub const END_TYPE_COMPETITION: &str = "fin de compétition";

pub enum TypeDate {
    StartRegistration,
    EndRegistration,
    StartCompetition,
    EndCompetition,
}

pub const EDITION_SELECT: &str = "edition_select";

pub const SETUP_ENV: &str = "setup";
pub const IMPORT_ENV: &str = "import_env";

/*
//catégories et channels
Arbitrage                      //admin, host, et modérateurs
  validations en attente       //seuls les hosts peuvent valider les captures
  joueurs a problemes
  bans et exclusions
  discussions
  discussions (vocal)
Gradins                        //tout le monde
  acceuil
  règlement du serveur
  général écrit
  général vocal
L'Arène                        //cas par cas
  règlement et infos           //tout le monde
  annonces                     //tout le monde
  classement                   //tout le monde
  captures validées            //tout le monde
  1 inscriptions               //Tous ceux qui n'ont pas le role inscrit
  2 versions et charme chroma  //tout ceux qui ont le rôle Inscrit et hosts
  3 constitution des équipes   //tous ceux qui ont le rôle Vérifié et hosts
  gimiks                       //tout le monde, change chaque semaine
[Nom d'équipe]                 //ceux qui ont le role de l'équipe
  proposition capture
  ecrit
  vocal (vocal)

//roles
Admin //peux voir tous les salons
Host  //peut voir les salons de modération
Modérateur //peut voir tous les salons
Vérifié
Inscrit
[noms d'équipes]
@everyone
*/

// catégories et channels statiques
pub const MODERATION_CATEGORY_NAME: &str = "Arbitrage";
pub const WAITING_VALIDATION_CHANNEL_NAME: &str = "validations-en-attente";
pub const PROBLEMATIC_PLAYERS_CHANNEL_NAME: &str = "joueurs-a-problèmes";
pub const BAN_AND_EXCLUSION_CHANNEL_NAME: &str = "bans-et-exclusions";
pub const MODERATION_CONVERSATION_CHANNEL_NAME: &str = "discussions";
pub const MODERATION_VOCAL_CONVERSATION_CHANNEL_NAME: &str = "discussions-vocal";
pub const GENERAL_CATEGORY_NAME: &str = "Gradins";
pub const BOT_RETURN_CHANNEL_NAME: &str = "retour-bot";
pub const WELCOME_CHANNEL_NAME: &str = "acceuil";
pub const GENERAL_CHANNEL_NAME: &str = "général";
pub const GENERAL_VOCAL_CHANNEL_NAME: &str = "vocal-général";
pub const RULES_CHANNEL_NAME: &str = "règlement-du-serveur";
pub const COMPETITION_CATEGORY_NAME: &str = "L'Arène";
pub const RULES_AND_INFOS_CHANNEL_NAME: &str = "règlement-et-infos";
pub const COMPETITION_ANNOUNCES_CHANNEL_NAME: &str = "annonces";
pub const RANKING_CHANNEL_NAME: &str = "classement";
pub const VALIDATED_CAPTURES_CHANNEL_NAME: &str = "captures-validées";
pub const REGISTRATION_CHANNEL_NAME: &str = "1-inscriptions";
pub const VERSIONS_AND_CHARMS_CHANNEL_NAME: &str = "2-infos-dresseur";
pub const TEAM_CREATION_CHANNEL_NAME: &str = "3-constitution-des-équipes";
pub const FAQ_CHANNEL_NAME: &str = "faq-bot";

// roles
pub const ADMIN_ROLE_NAME: &str = "Admin";
pub const MODERATION_ROLE_NAME: &str = "Modérateur";
pub const HOST_ROLE_NAME: &str = "Host";
pub const VERIFIED_ROLE_NAME: &str = "Vérifié";
pub const REGISTERED_ROLE_NAME: &str = "Inscrit";
pub const EVERYONE_ROLE_NAME: &str = "@everyone";

pub const PLAYER_ID: &str = "player_id";
pub const VERSIONS: &str = "versions";
pub const TEAM: &str = "team";
pub const VERIFIED: &str = "verified";
pub const CHARMS: &str = "charms";

pub const CHARMS_COLLECTION: &str = "charms";
pub const VERSIONS_COLLECTION: &str = "versions";

//pour les valeurs dans la bdd
pub const BDD_POKE_RED_GREEN_BLUE              : &str = "red_green_blue";
pub const BDD_POKE_YELLOW                      : &str = "yellow";
pub const BDD_POKE_GOLD_SILVER                 : &str = "gold_silver";
pub const BDD_POKE_CRYSTAL                     : &str = "crystal";
pub const BDD_POKE_RUBY_SAPPHIRE               : &str = "ruby_sapphire";
pub const BDD_POKE_FIRERED_LEAFGREEN           : &str = "firered_leafgreen";
pub const BDD_POKE_EMERALD                     : &str = "emerald";
pub const BDD_POKE_DIAMOND_PEARL               : &str = "diamond_pearl";
pub const BDD_POKE_PLATINUM                    : &str = "platinum";
pub const BDD_POKE_HEARTGOLD_SOULSILVER        : &str = "heartgold_soulsilver";
pub const BDD_POKE_BLACK_WHITE                 : &str = "black_chite";
pub const BDD_POKE_BLACK2_WHITE2               : &str = "black2_white2";
pub const BDD_POKE_X_Y                         : &str = "x_y";
pub const BDD_POKE_OMEGA_RUBY_ALPHA_SAPPHIRE   : &str = "omega_ruby_alpha_sapphire";
pub const BDD_POKE_SUN_MOON                    : &str = "sun_moon";
pub const BDD_POKE_ULTRASUN_ULTRAMOON          : &str = "ultrasun_ultramoon";
pub const BDD_POKE_LETSGOPIKACHU_LETSGOEEVEE   : &str = "letsgopikachu_letsgoeevee";
pub const BDD_POKE_SWORD_SHIELD                : &str = "sword_shield";
pub const BDD_POKE_BRILLANTDIAMOND_SHININGPEARL: &str = "brillantdiamond_shiningpearl";
pub const BDD_POKE_LEGENDARCEUS                : &str = "legendarceus";
pub const BDD_POKE_SCARLET_VIOLET              : &str = "scarlet_violet";
pub const BDD_POKE_DONJON_MYSTERE              : &str = "donjon_mystere_dx";
pub const BDD_POKE_COLOSEUM                    : &str = "coloseum";
pub const BDD_POKE_STADIUM_EU                  : &str = "stadium_eu";
pub const BDD_POKE_STADIUM_JAP                 : &str = "stadium_jap";
pub const BDD_POKE_STADIUM_2                   : &str = "stadium_2";
pub const BDD_POKE_XD                          : &str = "xd";

//pour l'affichage dans les channels
pub const POKE_RED_GREEN_BLUE               : &str = "Rouge/Vert/Bleu";
pub const POKE_YELLOW                       : &str = "Jaune";
pub const POKE_GOLD_SILVER                  : &str = "Or/Argent";
pub const POKE_CRYSTAL                      : &str = "Crystal";
pub const POKE_RUBY_SAPPHIRE                : &str = "Ruby/Saphir";
pub const POKE_FIRERED_LEAFGREEN            : &str = "Rouge feu/Vert feuille";
pub const POKE_EMERALD                      : &str = "Emeraude";
pub const POKE_DIAMOND_PEARL                : &str = "Diamant/Perle";
pub const POKE_PLATINUM                     : &str = "Platine";
pub const POKE_HEARTGOLD_SOULSILVER         : &str = "Heartgold/Soulsilver";
pub const POKE_BLACK_WHITE                  : &str = "Blanc/Noir";
pub const POKE_BLACK2_WHITE2                : &str = "Blanc2/Noir2";
pub const POKE_X_Y                          : &str = "X/Y";
pub const POKE_OMEGA_RUBY_ALPHA_SAPPHIRE    : &str = "Ruby oméga/Saphir alpha";
pub const POKE_SUN_MOON                     : &str = "Soleil/Lune";
pub const POKE_ULTRASUN_ULTRAMOON           : &str = "UltraSoleil/UltraLune";
pub const POKE_LETSGOPIKACHU_LETSGOEEVEE    : &str = "Let's go Pikachu/\nLet's go Evoli";
pub const POKE_SWORD_SHIELD                 : &str = "Epée/Bouclier";
pub const POKE_BRILLANTDIAMOND_SHININGPEARL : &str = "Diamant Eteincelant/\nPerle Scintillante";
pub const POKE_LEGENDARCEUS                 : &str = "Légendes Arceus";
pub const POKE_SCARLET_VIOLET               : &str = "Ecarlate/Violet";
pub const POKE_DONJON_MYSTERE               : &str = "Donjon Mystère DX (switch)";
pub const POKE_COLOSEUM                     : &str = "Coloseum";
pub const POKE_STADIUM_EU                   : &str = "Stadium EU";
pub const POKE_STADIUM_JAP                  : &str = "Stadium JAP";
pub const POKE_STADIUM_2                    : &str = "Stadium 2";
pub const POKE_XD                           : &str = "XD";

pub const MESSAGE_ID: &str = "message_id";
pub const TRAINER_NAME: &str = "trainer_name";
pub const MORE_INFO: &str = "more_info";