use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, str::FromStr};

mod constants {
    pub const ORIGIN: &str = "https://nexon-open-api-proxy.psvm203.workers.dev";
}

pub trait ApiRequest {
    const PATH: &'static str;

    type ApiResponse;

    async fn get(&self) -> Result<Self::ApiResponse, BadResponse>
    where
        Self: Serialize,
        for<'de> Self::ApiResponse: Deserialize<'de>,
    {
        let origin = constants::ORIGIN;
        let path = Self::PATH;
        let params =
            serde_urlencoded::to_string(self).map_err(|_| BadResponse::ParameterSerializeError)?;
        let url = format!("{origin}{path}?{params}");

        let response = Request::get(&url).send().await.map_err(|_| BadResponse::NetworkError)?;

        if !response.ok() {
            return Err(Self::parse_error_response(response).await);
        }

        response.json::<Self::ApiResponse>().await.map_err(|_| BadResponse::ParseError)
    }

    async fn parse_error_response(response: Response) -> BadResponse {
        match response.json::<Error>().await {
            Ok(error) => BadResponse::from_str(&error.name).unwrap(),
            Err(_) => BadResponse::UnknownResponse,
        }
    }
}

#[derive(Debug)]
pub enum BadResponse {
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
    UnknownResponse,
    ParameterSerializeError,
    ParseError,
    NetworkError,
}

impl FromStr for BadResponse {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OPENAPI00001" => Ok(Self::InternalServerError),
            "OPENAPI00002" => Ok(Self::Forbidden),
            "OPENAPI00003" => Ok(Self::InvalidIdentifier),
            "OPENAPI00004" => Ok(Self::InvalidParameter),
            "OPENAPI00005" => Ok(Self::InvalidApiKey),
            "OPENAPI00006" => Ok(Self::InvalidPath),
            "OPENAPI00007" => Ok(Self::TooManyRequests),
            "OPENAPI00009" => Ok(Self::DataNotReady),
            "OPENAPI00010" => Ok(Self::GameUnderMaintenance),
            "OPENAPI00011" => Ok(Self::ApiUnderMaintenance),
            _ => Ok(Self::UnknownResponse),
        }
    }
}

#[derive(Deserialize)]
struct Error {
    name: String,
    message: String,
}

#[derive(Deserialize)]
pub struct Character {
    pub ocid: String,
}

#[derive(Serialize)]
pub struct CharacterRequest {
    pub character_name: String,
}

impl ApiRequest for CharacterRequest {
    const PATH: &'static str = "/maplestory/v1/id";

    type ApiResponse = Character;
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct CharacterPropensity {
    date: Option<String>,
    charisma_level: u32,
    sensibility_level: u32,
    insight_level: u32,
    willingness_level: u32,
    pub handicraft_level: u32,
    charm_level: u32,
}

#[derive(Serialize)]
pub struct CharacterPropensityRequest {
    pub ocid: String,
}

impl ApiRequest for CharacterPropensityRequest {
    const PATH: &'static str = "/maplestory/v1/character/propensity";

    type ApiResponse = CharacterPropensity;
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct CharacterBasic {
    date: Option<String>,
    character_name: String,
    pub world_name: String,
    character_gender: String,
    character_class: String,
    character_class_level: String,
    character_level: u32,
    character_exp: u64,
    character_exp_rate: String,
    pub character_guild_name: Option<String>,
    character_image: String,
    character_date_create: String,
    access_flag: String,
    liberation_quest_clear: String,
}

#[derive(Serialize)]
pub struct CharacterBasicRequest {
    pub ocid: String,
}

impl ApiRequest for CharacterBasicRequest {
    const PATH: &'static str = "/maplestory/v1/character/basic";

    type ApiResponse = CharacterBasic;
}

#[derive(Deserialize)]
pub struct Guild {
    pub oguild_id: String,
}

#[derive(Serialize)]
pub struct GuildRequest {
    pub guild_name: String,
    pub world_name: String,
}

impl ApiRequest for GuildRequest {
    const PATH: &'static str = "/maplestory/v1/guild/id";

    type ApiResponse = Guild;
}

#[allow(clippy::struct_field_names, dead_code)]
#[derive(Deserialize)]
pub struct GuildSkill {
    pub skill_name: String,
    skill_description: String,
    pub skill_level: u32,
    skill_effect: String,
    skill_icon: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct GuildBasicInformation {
    date: Option<String>,
    world_name: String,
    guild_name: String,
    guild_level: u32,
    guild_fame: u32,
    guild_point: u32,
    guild_master_name: String,
    guild_member_count: u32,
    guild_member: Vec<String>,
    pub guild_skill: Vec<GuildSkill>,
    guild_noblesse_skill: Vec<GuildSkill>,
}

#[derive(Serialize)]
pub struct GuildBasicInformationRequest {
    pub oguild_id: String,
}

impl ApiRequest for GuildBasicInformationRequest {
    const PATH: &'static str = "/maplestory/v1/guild/basic";

    type ApiResponse = GuildBasicInformation;
}
