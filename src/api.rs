use gloo_net::http::{Request, Response};
use serde::Deserialize;

const ORIGIN: &str = "https://nexon-open-api-proxy.psvm203.workers.dev";
const GET_OCID_PATH: &str = "/maplestory/v1/id";
const GET_PROPENSITY_PATH: &str = "/maplestory/v1/character/propensity";
const GET_BASIC_INFORMATION_PATH: &str = "/maplestory/v1/character/basic";
const GET_GUILD_ID_PATH: &str = "/maplestory/v1/guild/id";
const GET_GUILD_BASIC_INFORMATION_PATH: &str = "/maplestory/v1/guild/basic";
const STATUS_SUCCESS: u16 = 200;

#[derive(Deserialize)]
struct Character {
    ocid: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CharacterPropensity {
    date: Option<String>,
    charisma_level: u32,
    sensibility_level: u32,
    insight_level: u32,
    willingness_level: u32,
    handicraft_level: u32,
    charm_level: u32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CharacterBasic {
    date: Option<String>,
    character_name: String,
    world_name: String,
    character_gender: String,
    character_class: String,
    character_class_level: String,
    character_level: u32,
    character_exp: u64,
    character_exp_rate: String,
    character_guild_name: String,
    character_image: String,
    character_date_create: String,
    access_flag: String,
    liberation_quest_clear_flag: String,
    liberation_quest_clear: String,
}

#[derive(Deserialize)]
struct Guild {
    oguild_id: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct GuildSkill {
    skill_name: String,
    skill_description: String,
    skill_level: u32,
    skill_effect: String,
    skill_icon: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct GuildBasicInformation {
    date: Option<String>,
    world_name: String,
    guild_name: String,
    guild_level: u32,
    guild_fame: u32,
    guild_point: u32,
    guild_master_name: String,
    guild_member_count: u32,
    guild_member: Vec<String>,
    guild_skill: Vec<GuildSkill>,
    guild_noblesse_skill: Vec<GuildSkill>,
}

#[derive(Deserialize)]
struct Error {
    name: String,
    #[allow(dead_code)]
    message: String,
}

pub enum ApiError {
    InternalServerError,
    Forbidden,
    InvalidIdentifier,
    InvalidParameter,
    InvalidApiKey,
    InvalidPath,
    TooManyRequests,
    DataNotReady,
    GameUnderMaintenance,
    ApiUnderMaintenance,
    UnknownErrorResponse,
    ParseError,
    NetworkError,
}

fn map_error_code(error_code: &str) -> ApiError {
    use ApiError::*;

    match error_code {
        "OPENAPI00001" => InternalServerError,
        "OPENAPI00002" => Forbidden,
        "OPENAPI00003" => InvalidIdentifier,
        "OPENAPI00004" => InvalidParameter,
        "OPENAPI00005" => InvalidApiKey,
        "OPENAPI00006" => InvalidPath,
        "OPENAPI00007" => TooManyRequests,
        "OPENAPI00009" => DataNotReady,
        "OPENAPI00010" => GameUnderMaintenance,
        "OPENAPI00011" => ApiUnderMaintenance,
        _ => UnknownErrorResponse,
    }
}

async fn parse_error_response<T>(response: Response) -> Result<T, ApiError> {
    match response.json::<Error>().await {
        Ok(error) => Err(map_error_code(&error.name)),
        Err(_) => Err(ApiError::UnknownErrorResponse),
    }
}

async fn send_get_request<T: for<'de> Deserialize<'de>>(
    url: String,
    params: Vec<(&'static str, String)>,
) -> Result<T, ApiError> {
    let response =
        Request::get(&url).query(params).send().await.map_err(|_| ApiError::NetworkError)?;

    if response.status() != STATUS_SUCCESS {
        return parse_error_response(response).await;
    }

    response.json::<T>().await.map_err(|_| ApiError::ParseError)
}

async fn get_ocid(character_name: String) -> Result<String, ApiError> {
    send_get_request::<Character>(
        format!("{ORIGIN}{GET_OCID_PATH}"),
        vec![("character_name", character_name)],
    )
    .await
    .map(|character| character.ocid)
}

async fn get_handicraft_level(ocid: String) -> Result<u32, ApiError> {
    send_get_request::<CharacterPropensity>(
        format!("{ORIGIN}{GET_PROPENSITY_PATH}"),
        vec![("ocid", ocid)],
    )
    .await
    .map(|propensity| propensity.handicraft_level)
}

pub async fn get_handicraft_level_by_character_name(
    character_name: String,
) -> Result<u32, ApiError> {
    let ocid = get_ocid(character_name).await?;
    get_handicraft_level(ocid).await
}

async fn get_basic_information(ocid: String) -> Result<CharacterBasic, ApiError> {
    send_get_request::<CharacterBasic>(
        format!("{ORIGIN}{GET_BASIC_INFORMATION_PATH}"),
        vec![("ocid", ocid)],
    )
    .await
}

async fn get_guild_id(guild_name: String, world_name: String) -> Result<String, ApiError> {
    send_get_request::<Guild>(
        format!("{ORIGIN}{GET_GUILD_ID_PATH}"),
        vec![("guild_name", guild_name), ("world_name", world_name)],
    )
    .await
    .map(|guild| guild.oguild_id)
}

async fn get_guild_basic_information(guild_id: String) -> Result<GuildBasicInformation, ApiError> {
    send_get_request::<GuildBasicInformation>(
        format!("{ORIGIN}{GET_GUILD_BASIC_INFORMATION_PATH}"),
        vec![("oguild_id", guild_id)],
    )
    .await
}

pub async fn get_guild_skill_level_by_character_name(
    character_name: String,
    skill_name: &'static str,
) -> Result<u32, ApiError> {
    let ocid = get_ocid(character_name).await?;
    let guild_name = get_basic_information(ocid.clone()).await?.character_guild_name;
    let world_name = get_basic_information(ocid).await?.world_name;
    let guild_id = get_guild_id(guild_name, world_name).await?;
    let skills = get_guild_basic_information(guild_id).await?.guild_skill;

    let level = skills
        .iter()
        .find(|skill| skill.skill_name == skill_name)
        .map(|skill| skill.skill_level)
        .unwrap_or(0);

    Ok(level)
}
