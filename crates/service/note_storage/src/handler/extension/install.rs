// Copyright 2022 BaihaiAI, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Write;

use axum::extract::Query;
use common_model::Rsp;
use err::ErrorTrace;

use super::models::ExtensionReq;
use crate::handler::extension::models::ExtensionResp;

pub async fn install(Query(req): Query<ExtensionReq>) -> Result<Rsp<String>, ErrorTrace> {
    install_handler(req.team_id, req.user_id, &req.name, &req.version).await
}

pub async fn install_handler(
    team_id: u64,
    user_id: u64,
    name: &str,
    version: &str,
) -> Result<Rsp<String>, ErrorTrace> {
    let installed_extensions_path = business::path_tool::user_extensions_path(team_id, user_id);
    let extension_name = format!("{}/{}", name, version);
    tracing::info!(
        "run extensions install api, path:{installed_extensions_path} ,name:{extension_name}"
    );
    let recommended_extension_path =
        business::path_tool::recommended_extensions().join(&extension_name);
    //需要安装的插件的路径
    let extension_path = format!("{}/{}", &installed_extensions_path, name);
    std::fs::create_dir_all(&extension_path)?;

    common_tools::command_tools::copy(
        recommended_extension_path.to_str().unwrap(),
        &extension_path,
    )?;
    //需要安装的插件config信息
    let jdata = std::fs::read_to_string(recommended_extension_path.join("config.json"))?;
    let mut new_extension_config = serde_json::from_str::<ExtensionResp>(&jdata)?;

    let url = format!("{installed_extensions_path}/{extension_name}/");
    new_extension_config.url = Some(url.clone());
    //已安装插件列表
    let extensions_config_path =
        std::path::Path::new(&installed_extensions_path).join("extensions_config.json");

    let mut contents = super::get_extensions_config(&extensions_config_path).await?;

    contents.push(new_extension_config);

    let data = serde_json::to_string(&contents).unwrap();
    let mut f = std::fs::File::create(extensions_config_path).unwrap();
    f.write_all(data.as_bytes()).unwrap();

    Ok(Rsp::success(url))
}
