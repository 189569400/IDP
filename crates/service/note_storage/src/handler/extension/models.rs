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

use std::collections::HashSet;

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;

lazy_static! {
    static ref INVISABLE_EXTENSION: HashSet<&'static str> = {
        let a = include_bytes!("../../../extension_config/config.json");
        let extension_config: ExtensionConfig = serde_json::from_slice(a).unwrap();
        let mut m: HashSet<&'static str> = HashSet::new();
        extension_config.invisible.into_iter().for_each(|x| {
            m.insert(Box::leak(x.into_boxed_str()));
        });
        m
    };
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionConfig {
    pub init: Vec<String>,
    pub invisible: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub user_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub user_id: u64,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionResp {
    pub name: String,
    pub version: String,
    pub url: Option<String>,
    pub entry: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub icon: Option<String>,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub visible: Option<bool>,
}

impl PartialEq for ExtensionResp {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}
impl Eq for ExtensionResp {}

impl PartialOrd for ExtensionResp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.version.partial_cmp(&other.version)
    }
}
impl Ord for ExtensionResp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.name.cmp(&other.name) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.version.cmp(&other.version)
    }
}

impl ExtensionResp {
    pub fn is_visible(&self) -> bool {
        !INVISABLE_EXTENSION.contains(self.name.as_str())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstalledExtensionResp {
    pub name: String,
    pub version: String,
    pub optional_version: Option<Vec<String>>,
    pub url: Option<String>,
    pub entry: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub icon: Option<String>,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub visible: Option<bool>,
}

impl PartialEq for InstalledExtensionResp {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}
impl Eq for InstalledExtensionResp {}

impl PartialOrd for InstalledExtensionResp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.version.partial_cmp(&other.version)
    }
}
impl Ord for InstalledExtensionResp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.name.cmp(&other.name) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.version.cmp(&other.version)
    }
}

impl InstalledExtensionResp {
    pub fn is_visible(&self) -> bool {
        !INVISABLE_EXTENSION.contains(self.name.as_str())
    }
}
