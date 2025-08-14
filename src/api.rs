use gloo_net::http;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

const ORIGIN: &str = "https://nexon-open-api-proxy.psvm203.workers.dev";
const GET_OCID_PATH: &str = "/maplestory/v1/id";
const GET_PROPENSITY_PATH: &str = "/maplestory/v1/character/propensity";
const GET_BASIC_INFORMATION_PATH: &str = "/maplestory/v1/character/basic";
const GET_GUILD_ID_PATH: &str = "/maplestory/v1/guild/id";
const GET_GUILD_BASIC_INFORMATION_PATH: &str = "/maplestory/v1/guild/basic";
const STATUS_SUCCESS: u16 = 200;
const UNKNOWN_RESPONSE_ERROR_MESSAGE: &str = "알 수 없는 응답 오류:";
const NETWORK_ERROR_MESSAGE: &str = "네트워크 오류:";
const PARSE_ERROR_MESSAGE: &str = "응답 파싱 오류:";

trait ApiRequest {
    type Response;

    fn endpoint_path() -> &'static str;

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

    async fn parse_error_response<T>(response: http::Response) -> Result<T, ApiError> {
        match response.json::<Error>().await {
            Ok(error) => Err(Self::map_error_code(&error.name)),
            Err(error) => {
                gloo_console::error!(UNKNOWN_RESPONSE_ERROR_MESSAGE, error.to_string());
                Err(ApiError::UnknownErrorResponse)
            }
        }
    }

    async fn get_api_data(&self) -> Result<Self::Response, ApiError>
    where
        Self: Serialize,
        Self::Response: DeserializeOwned,
    {
        let path = Self::endpoint_path();
        let params = serde_urlencoded::to_string(self).unwrap();
        let url = format!("{ORIGIN}{path}?{params}");

        let response = http::Request::get(&url).send().await.map_err(|error| {
            gloo_console::error!(NETWORK_ERROR_MESSAGE, error.to_string());
            ApiError::NetworkError
        })?;

        if response.status() != STATUS_SUCCESS {
            return Self::parse_error_response(response).await;
        }

        response.json::<Self::Response>().await.map_err(|error| {
            gloo_console::error!(PARSE_ERROR_MESSAGE, error.to_string());
            ApiError::ParseError
        })
    }
}

#[derive(Deserialize)]
struct Character {
    ocid: String,
}

#[derive(Serialize)]
struct CharacterRequest {
    character_name: String,
}

impl ApiRequest for CharacterRequest {
    type Response = Character;

    fn endpoint_path() -> &'static str {
        GET_OCID_PATH
    }
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

#[derive(Serialize)]
struct CharacterPropensityRequest {
    ocid: String,
}

impl ApiRequest for CharacterPropensityRequest {
    type Response = CharacterPropensity;

    fn endpoint_path() -> &'static str {
        GET_PROPENSITY_PATH
    }
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
    character_guild_name: Option<String>,
    character_image: String,
    character_date_create: String,
    access_flag: String,
    liberation_quest_clear_flag: String,
    liberation_quest_clear: String,
}

#[derive(Serialize)]
struct CharacterBasicRequest {
    ocid: String,
}

impl ApiRequest for CharacterBasicRequest {
    type Response = CharacterBasic;

    fn endpoint_path() -> &'static str {
        GET_BASIC_INFORMATION_PATH
    }
}

#[derive(Deserialize)]
struct Guild {
    oguild_id: String,
}

#[derive(Serialize)]
struct GuildRequest {
    guild_name: String,
    world_name: String,
}

impl ApiRequest for GuildRequest {
    type Response = Guild;

    fn endpoint_path() -> &'static str {
        GET_GUILD_ID_PATH
    }
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

#[derive(Serialize)]
struct GuildBasicInformationRequest {
    oguild_id: String,
}

impl ApiRequest for GuildBasicInformationRequest {
    type Response = GuildBasicInformation;

    fn endpoint_path() -> &'static str {
        GET_GUILD_BASIC_INFORMATION_PATH
    }
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

async fn get_ocid(character_name: String) -> Result<String, ApiError> {
    CharacterRequest {
        character_name,
    }
    .get_api_data()
    .await
    .map(|character| character.ocid)
}

async fn get_handicraft_level(ocid: String) -> Result<u32, ApiError> {
    CharacterPropensityRequest {
        ocid,
    }
    .get_api_data()
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
    CharacterBasicRequest {
        ocid,
    }
    .get_api_data()
    .await
}

async fn get_guild_id(guild_name: String, world_name: String) -> Result<String, ApiError> {
    GuildRequest {
        guild_name,
        world_name,
    }
    .get_api_data()
    .await
    .map(|guild| guild.oguild_id)
}

async fn get_guild_basic_information(oguild_id: String) -> Result<GuildBasicInformation, ApiError> {
    GuildBasicInformationRequest {
        oguild_id,
    }
    .get_api_data()
    .await
}

pub async fn get_guild_skill_level_by_character_name(
    character_name: String,
    skill_name: &'static str,
) -> Result<u32, ApiError> {
    let ocid = get_ocid(character_name).await?;
    let basic_info = get_basic_information(ocid).await?;

    let guild_name = match basic_info.character_guild_name {
        Some(guild_name) => guild_name,
        None => return Ok(0),
    };

    let world_name = basic_info.world_name;
    let guild_id = get_guild_id(guild_name, world_name).await?;
    let skills = get_guild_basic_information(guild_id).await?.guild_skill;

    let level = skills
        .iter()
        .find(|skill| skill.skill_name == skill_name)
        .map(|skill| skill.skill_level)
        .unwrap_or(0);

    Ok(level)
}
