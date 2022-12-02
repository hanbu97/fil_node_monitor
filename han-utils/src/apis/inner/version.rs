use super::super::*;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVersionRep {
    pub git_version: String,
    pub compile_time: String,
}

pub async fn get_version() -> Res<GetVersionRep> {
    Res::success(GetVersionRep {
        git_version: env!("GIT_VERSION").to_string(),
        compile_time: env!("COMPILE_TIME").to_string(),
    })
}
