pub const VALUE_START_REGISTRATION : &str = "edit_start_registration";
pub const VALUE_END_REGISTRATION : &str = "edit_end_registration";
pub const VALUE_START_COMPETITION : &str = "edit_start_competition";
pub const VALUE_END_COMPETITION : &str = "edit_end_competition";
pub const CREATE_NEW_EDITION : &str = "create_new_edition";
pub const CREATE_EDITION_INSCRIPTION_ID : &str = "inscription";
pub const CREATE_EDITION_COMPETITION_ID : &str = "competition";

pub const PING : &str = "ping";
pub const NEW_EDITION : &str = "new_edition";
pub const DELETE_EDITION : &str = "delete_edition";
pub const EDIT_EDITION : &str = "edit_edition";
 pub const GET_EDITION : &str = "get_edition";

pub const DELETE_EDITION_MODAL : &str = "delete_edition_modal";
pub const EDIT_START_INSCRIPTIONS: &str = "edit_start_inscription";
pub const EDIT_END_INSCRIPTIONS : &str = "edit_end_inscriptions";
pub const EDIT_START_COMPETITION : &str = "edit_start_competition";
pub const EDIT_END_COMPETITION : &str = "edit_end_competition";

pub const EDIT_START_EDITION_END : &str = "edit_start_inscriptions_end"; // ?????

pub const EDITIONS_COLLECTION : &str = "editions";
pub const DISCORD_INFO_COLLECTION : &str = "discord_infos";
pub const RAYQUABOT_DB : &str = "RayquaBot";
pub const INSCRIPTION_START_DATE: &str = "inscription_start_date";
pub const INSCRIPTION_END_DATE: &str = "inscription_end_date";
pub const COMPETITION_START_DATE: &str = "competition_start_date";
pub const COMPETITION_END_DATE: &str = "competition_end_date";
pub const GUILD_ID : &str = "guild_id";
pub const EDITION_NAME: &str = "edition_name";
pub const ORGANISATOR: &str = "organisateur";

pub const EDITION_FILE : &str = "edition_file";
pub const ADMIN_ROLE_ID :&str =  "admin_role_id";
pub const HOST_ROLE_ID : &str = "host_role_id";
pub const INSCRIT_ROLE_ID : &str = "inscrit_role_id";
pub const MODERRATION_CATEGORY_ID: &str = "moderation_category_id";
pub const EDITION_CATEGORY_ID: &str = "edition_category_id";

pub const RED_COLOR : i32 = 0xff0000;
pub const GREEN_COLOR : i32 = 0x00ff00;
pub const CONTACT : &str = "contact.cgbots@gmail.com";

pub const TYPE_DEBUT_INSCIRPTIONS : &str = "debut d'inscriptions";
pub const TYPE_FIN_INSCRIPTIONS : &str = "fin d'inscriptions";
pub const TYPE_DEBUT_COMPETITION : &str = "debut de compétition";
pub const TYPE_FIN_COMPETITION : &str = "fin de compétition";

pub enum TypeDate {
 StartRegistration,
 EndRegistration,
 StartCompetition,
 EndCompetition,
}

pub const EDITION_SELECT : &str = "edition_select";

pub const SETUP_ENV: &str = "setup_env";
pub const IMPORT_ENV: &str = "import_env";

pub const ADMIN_ROLE_NAME: &str = "Admin";
pub const HOST_ROLE_NAME: &str = "Host";
pub const INSCRIT_ROLE_NAME: &str = "Inscrit";
pub const EVERYONE_ROLE_NAME: &str = "@everyone";
pub const MODERATION_CATEGORY_NAME: &str = "Moderation";