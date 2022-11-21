use business::business_term::ProjectId;
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
use business::business_term::TeamId;

use super::control;
use crate::common::error::IdpGlobalError;

// 拼接db_file_name,通过dashboard启动,指定这个文件,会自动创建sqlite对应数据库
// English: Splicing db_file_name, start through dashboard, specify this file, will automatically create the corresponding sqlite database
pub async fn datasource_new(
    team_id: TeamId,
    project_id: ProjectId,
    datasource_name: String,
) -> Result<String, IdpGlobalError> {
    let db_file_name = format!("idp_{}.db", datasource_name);
    let datasource_list = get_datasource_list(team_id, project_id).await?;
    // if exists the same name, return error
    if datasource_list.contains(&db_file_name) {
        //TODO change status code
        return Err(IdpGlobalError::ErrorCodeMsg(
            131500,
            "db file name already exist".to_string(),
        ));
    }
    let db_url = control::get_dburl_by_db_file_name(team_id, project_id, &db_file_name);
    match control::start_hpopt_backend(db_url, team_id, project_id).await {
        Ok(_) => {
            // if start success, shutdown backend and return db_file_name(we just need create db schema via start backend).
            let db_url = control::get_dburl_by_db_file_name(team_id, project_id, &db_file_name);
            control::stop_hpopt_backend(db_url).await?;

            Ok(db_file_name)
        }
        Err(e) => Err(e),
    }
}
// pub async fn delete_datasource(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<(), IdpGlobalError> {
//     //todo!
//     Ok(())
// }
///
/// /store/{team_id}/projects/project_id/hp[opt_datasource]
pub async fn get_datasource_list(
    team_id: TeamId,
    project_id: ProjectId,
) -> Result<Vec<String>, std::io::Error> {
    // get datasource dir path
    let datasource_path = business::path_tool::get_hpopt_datasource_path(team_id, project_id);
    // create file struct by path and get all file name.
    let mut datasource_list = Vec::new();
    if let Ok(dir) = std::fs::read_dir(datasource_path.clone()) {
        dir.for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            datasource_list.push(file_name.to_string());
                        }
                    }
                }
            }
        });
    } else {
        // this dir not exist, create it.
        // print log on console. todo:need change to log crate.
        println!(
            "datasource dir not exist, create it. path: {}",
            datasource_path
        );
        // log::info!("datasource dir not exist, create it. path: {}",datasource_path);
        std::fs::create_dir_all(datasource_path)?;
    }
    Ok(datasource_list)
}

#[cfg(not)]
#[tokio::test]
async fn test_datasource_list() {
    let team_id = 19980923;
    let project_id = 1001;
    let datasource_list = get_datasource_list(team_id, project_id).await;
    println!("{:?}", datasource_list);
}
