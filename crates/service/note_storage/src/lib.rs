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

#![deny(unused_crate_dependencies)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_tests)]

use std::collections::HashMap;
use std::sync::Arc;

use handler::package::search::get_package_map;
use sqlx::Pool;
use sqlx::Postgres;
use test_runner as _;
use tokio::sync::Mutex;
pub mod utils;
use utils::db_utils::init_pg_connect_pool;

use crate::handler::extension::get_extension;
mod api;
pub(crate) mod api_model;
mod app_context;
pub(crate) mod business_;
pub(crate) mod common;
mod dataobj;
mod handler;
mod route;
pub(crate) mod status_code;

pub async fn main() {
    let reload_log_level_handle = logger::init_logger();
    // clap::Command::new(env!("CARGO_PKG_NAME"))
    // .version(env!("VERSION"))
    // .get_matches();
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "--version" {
        println!("{}", env!("VERSION"));
        return;
    }

    if business::kubernetes::is_k8s() {
        match license_generator::verify_license(
            license_generator::DEFAULT_LICENSE_PUBLIC_KEY_PATH,
            license_generator::DEFAULT_LICENSE_PATH,
        ) {
            Ok(license) => {
                let expire_timestamp = license.expire_timestamp;
                tokio::spawn(async move {
                    loop {
                        let timestamp = tokio::task::spawn_blocking(|| {
                            license_generator::get_timestamp_from_internet()
                        })
                        .await
                        .unwrap();
                        if timestamp > expire_timestamp {
                            tracing::error!("license expire, exit...");
                            std::process::exit(1);
                        }
                        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                    }
                });
            }
            Err(err) => {
                tracing::error!("verify_license fail, exit... {err}");
                std::process::exit(1);
            }
        }
    }

    // tokio::spawn(sum_project_disk(rb.clone()));

    // let path = Path::new("/opt/config/config.toml");
    let project_info_map = Arc::new(Mutex::new(HashMap::<String, HashMap<String, String>>::new()));
    let pg_option: Option<Pool<Postgres>> = if business::kubernetes::is_k8s() {
        tokio::spawn(get_package_map(project_info_map.clone(), true));
        let pg_pool = init_pg_connect_pool().await;
        Some(pg_pool)
    } else {
        tokio::spawn(get_package_map(project_info_map.clone(), false));
        None
    };

    tracing::debug!("pg_option->{:#?}", pg_option);
    if business::kubernetes::is_k8s() {
        tokio::spawn(get_extension::get_extension());
    }

    let app =
        route::init_router(project_info_map.clone(), pg_option, reload_log_level_handle).await;

    let address = std::net::SocketAddr::from((
        std::net::Ipv4Addr::UNSPECIFIED,
        business::note_storage_port(),
    ));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
