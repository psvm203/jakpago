use crate::utils::api::requests::{
    ApiRequest, BadResponse, CharacterBasic, CharacterBasicRequest, CharacterPropensityRequest,
    CharacterRequest, GuildBasicInformationRequest, GuildRequest, GuildSkill,
};

mod constants {
    pub const ENHANCE_MASTERY: &str = "강화의 달인";
    pub const UPGRADE_SALVATION: &str = "실패를 두려워 않는";
}

pub struct ProbabilityContext {
    _world_name: String,
    pub handicraft: u32,
    pub enhance_mastery: u32,
    pub upgrade_salvation: u32,
}

pub async fn fetch_probability_context(
    character_name: String,
) -> Result<ProbabilityContext, BadResponse> {
    let ocid = get_ocid(character_name).await?;
    let handicraft = get_handicraft(ocid.clone()).await?;
    let character_basic = get_character_basic(ocid).await?;
    let world_name = character_basic.world_name;

    if let Some(guild_name) = character_basic.character_guild_name {
        let guild_id = get_guild_id(guild_name, world_name.clone()).await?;
        let guild_skills = get_guild_skills(guild_id).await?;

        let enhance_mastery = guild_skills
            .iter()
            .find(|x| x.skill_name == constants::ENHANCE_MASTERY)
            .map_or(0, |skill| skill.skill_level);

        let upgrade_salvation = guild_skills
            .iter()
            .find(|x| x.skill_name == constants::UPGRADE_SALVATION)
            .map_or(0, |skill| skill.skill_level);

        Ok(ProbabilityContext {
            _world_name: world_name,
            handicraft,
            enhance_mastery,
            upgrade_salvation,
        })
    } else {
        Ok(ProbabilityContext {
            _world_name: world_name,
            handicraft,
            enhance_mastery: 0,
            upgrade_salvation: 0,
        })
    }
}

async fn get_ocid(character_name: String) -> Result<String, BadResponse> {
    CharacterRequest {
        character_name,
    }
    .get()
    .await
    .map(|character| character.ocid)
}

async fn get_handicraft(ocid: String) -> Result<u32, BadResponse> {
    CharacterPropensityRequest {
        ocid,
    }
    .get()
    .await
    .map(|propensity| propensity.handicraft_level)
}

async fn get_character_basic(ocid: String) -> Result<CharacterBasic, BadResponse> {
    CharacterBasicRequest {
        ocid,
    }
    .get()
    .await
}

async fn get_guild_id(guild_name: String, world_name: String) -> Result<String, BadResponse> {
    GuildRequest {
        guild_name,
        world_name,
    }
    .get()
    .await
    .map(|guild| guild.oguild_id)
}

async fn get_guild_skills(oguild_id: String) -> Result<Vec<GuildSkill>, BadResponse> {
    GuildBasicInformationRequest {
        oguild_id,
    }
    .get()
    .await
    .map(|x| x.guild_skill)
}
