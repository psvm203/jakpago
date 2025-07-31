use gloo_net::http::Request;
use serde::Deserialize;

const API_KEY: &str = include_str!("key.txt");
const ORIGIN: &str = "https://open.api.nexon.com";
const GET_OCID_PATH: &str = "/maplestory/v1/id";
const GET_PROPENSITY_PATH: &str = "/maplestory/v1/character/propensity";
const STATUS_SUCCESS: u16 = 200;

#[derive(Deserialize)]
struct Character {
    ocid: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CharacterPropensity {
    date: Option<String>,
    charisma_level: usize,
    sensibility_level: usize,
    insight_level: usize,
    willingness_level: usize,
    handicraft_level: usize,
    charm_level: usize,
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
    DataUnderMaintenance,
    GameUnderMaintenance,
    ApiUnderMaintenance,
    UnknownErrorResponse,
    ParseError,
    NetworkError,
}

async fn send_get_request<T: for<'de> Deserialize<'de>>(
    url: String,
    params: Vec<(&'static str, String)>,
) -> Result<T, ApiError> {
    let request = Request::get(&url)
        .query(params)
        .header("accept", "application/json")
        .header("x-nxopen-api-key", API_KEY);

    match request.send().await {
        Ok(response) => {
            let status = response.status();

            if status != STATUS_SUCCESS {
                match response.json::<Error>().await {
                    Ok(data) => match data.name.as_str() {
                        "OPENAPI00001" => return Err(ApiError::InternalServerError),
                        "OPENAPI00002" => return Err(ApiError::Forbidden),
                        "OPENAPI00003" => return Err(ApiError::InvalidIdentifier),
                        "OPENAPI00004" => return Err(ApiError::InvalidParameter),
                        "OPENAPI00005" => return Err(ApiError::InvalidApiKey),
                        "OPENAPI00006" => return Err(ApiError::InvalidPath),
                        "OPENAPI00007" => return Err(ApiError::TooManyRequests),
                        "OPENAPI00009" => return Err(ApiError::DataUnderMaintenance),
                        "OPENAPI00010" => return Err(ApiError::GameUnderMaintenance),
                        "OPENAPI00011" => return Err(ApiError::ApiUnderMaintenance),
                        _ => return Err(ApiError::UnknownErrorResponse),
                    },
                    Err(_) => return Err(ApiError::UnknownErrorResponse),
                }
            }

            match response.json::<T>().await {
                Ok(data) => Ok(data),
                Err(_) => Err(ApiError::ParseError),
            }
        }
        Err(_) => Err(ApiError::NetworkError),
    }
}

async fn get_ocid(character_name: String) -> Result<String, ApiError> {
    send_get_request::<Character>(
        format!("{ORIGIN}{GET_OCID_PATH}"),
        vec![("character_name", character_name)],
    )
    .await
    .map(|character| character.ocid)
}

async fn get_handicraft_level(ocid: String) -> Result<usize, ApiError> {
    send_get_request::<CharacterPropensity>(
        format!("{ORIGIN}{GET_PROPENSITY_PATH}"),
        vec![("ocid", ocid)],
    )
    .await
    .map(|propensity| propensity.handicraft_level)
}

pub async fn get_handicraft_level_by_name(character_name: String) -> Result<usize, ApiError> {
    match get_ocid(character_name).await {
        Ok(ocid) => get_handicraft_level(ocid).await,
        Err(err) => Err(err),
    }
}
