use gloo_net::http::Request;
use serde::Deserialize;

const API_KEY: &str = include_str!("key.txt");
const GET_OCID_URL: &str = "https://open.api.nexon.com/maplestory/v1/id";
const GET_PROPENSITY_URL: &str = "https://open.api.nexon.com/maplestory/v1/character/propensity";
const STATUS_SUCCESS: u16 = 200;
const RESPONSE_ERROR_MESSAGE: &str = "API 응답 에러:";
const DESERIALIZE_ERROR_MESSAGE: &str = "응답 파싱 에러:";
const NETWORK_ERROR_MESSAGE: &str = "네트워크 에러:";

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

async fn send_get_request<T: for<'de> Deserialize<'de>>(
    url: &'static str,
    params: Vec<(&'static str, String)>,
) -> Option<T> {
    let request = Request::get(url)
        .query(params)
        .header("accept", "application/json")
        .header("x-nxopen-api-key", API_KEY);

    match request.send().await {
        Ok(response) => {
            let status = response.status();

            if status != STATUS_SUCCESS {
                gloo_console::error!(RESPONSE_ERROR_MESSAGE, status);
                return None;
            }

            match response.json::<T>().await {
                Ok(data) => Some(data),
                Err(err) => {
                    gloo_console::error!(DESERIALIZE_ERROR_MESSAGE, err.to_string());
                    None
                }
            }
        }
        Err(err) => {
            gloo_console::error!(NETWORK_ERROR_MESSAGE, err.to_string());
            None
        }
    }
}

async fn get_ocid(character_name: String) -> Option<String> {
    send_get_request::<Character>(GET_OCID_URL, vec![("character_name", character_name)])
        .await
        .map(|character| character.ocid)
}

async fn get_handicraft_level(ocid: String) -> Option<usize> {
    send_get_request::<CharacterPropensity>(GET_PROPENSITY_URL, vec![("ocid", ocid)])
        .await
        .map(|propensity| propensity.handicraft_level)
}

pub async fn get_handicraft_level_by_name(character_name: String) -> Option<usize> {
    match get_ocid(character_name).await {
        Some(ocid) => get_handicraft_level(ocid).await,
        None => None,
    }
}
