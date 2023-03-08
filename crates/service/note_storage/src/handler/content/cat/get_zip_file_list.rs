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

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use err::ErrorTrace;

#[derive(serde::Serialize, Debug, Default)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct ZipNode {
    /// frontend ant design require a unique key for each tree node, here we must abs path
    pub absolute_path: String, //"/store/idp-note/projects/1/notebooks",
    pub file_name: String, //"notebooks",
    pub has_children: bool,
    // Axum resp model require Send+Sync, so we can't use Rc
    // tokio::sync::Mutex not impl serde, so we use std's
    pub children: Vec<Arc<Mutex<ZipNode>>>,
    #[serde(skip)]
    pub is_top_level: bool,
}

trait ZipTraverse {
    fn is_dir(&self) -> bool;
    fn path(&self) -> std::path::PathBuf;
}

impl ZipTraverse for String {
    fn is_dir(&self) -> bool {
        self.ends_with('/')
    }

    fn path(&self) -> std::path::PathBuf {
        std::path::Path::new(self).to_path_buf()
    }
}

/*
= note: `FnMut` closures only have access to their captured variables while they are executing...
= note: ...therefore, they cannot allow references to captured variables to escape
= note: requirement occurs because of the type `ZipFile<'_>`, which makes the generic argument `'_` invariant
= note: the struct `ZipFile<'a>` is invariant over the parameter `'a`
= help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

((0..zip.len()).map(|i| {
    let file = zip.by_index(i)?;
    Ok(Arc::new(file))
}))
*/
impl ZipTraverse for tar::Entry<'_, flate2::read::GzDecoder<std::fs::File>> {
    fn is_dir(&self) -> bool {
        self.header().entry_type().is_dir()
    }

    fn path(&self) -> std::path::PathBuf {
        self.path().unwrap().to_path_buf()
    }
}

pub fn preview_zip_file_list(path: &PathBuf) -> Result<Vec<Arc<Mutex<ZipNode>>>, ErrorTrace> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open(path)?)?;
    let mut iter = Vec::new();
    for i in 0..zip.len() {
        let index = zip.by_index(i)?;
        let name_raw = index.name_raw();
        let name = match String::from_utf8((name_raw).to_vec()) {
            Ok(name) => name,
            Err(_) => {
                let (text, _encoding, has_error) = encoding_rs::GB18030.decode(name_raw);
                if has_error {
                    String::from_utf8_lossy(name_raw).to_string()
                } else {
                    text.to_string()
                }
            }
        };
        iter.push(Ok(name));
    }
    // file_names is unordered
    // files_to_tree(zip.file_names().map(|x| Ok(x)))

    files_to_tree(iter)
}

pub fn preview_gzip_file_list(path: &PathBuf) -> Result<Vec<Arc<Mutex<ZipNode>>>, ErrorTrace> {
    let gzip = flate2::read::GzDecoder::new(std::fs::File::open(path)?);
    let mut archive = tar::Archive::new(gzip);
    files_to_tree(archive.entries()?)
}

// TODO support non-dir multi single file zip
// if zip/gzip is a single file, then root element is not a dir
// debug_assert!(entry.is_dir());
fn files_to_tree<T: ZipTraverse>(
    entries: impl IntoIterator<Item = Result<T, std::io::Error>>,
) -> Result<Vec<Arc<Mutex<ZipNode>>>, ErrorTrace> {
    let mut dirs_map = std::collections::HashMap::new();

    let top_level = Arc::new(Mutex::new(ZipNode {
        file_name: "/".to_string(),
        absolute_path: "/".to_string(),
        has_children: false,
        children: Vec::new(),
        is_top_level: true,
    }));

    // dirs_map key is abs_path, if two folder has same name their key must unique
    dirs_map.insert("/".to_string(), top_level);

    for entry_res in entries {
        let entry = entry_res?;
        let path = &entry.path();
        let path_str = path.to_str().unwrap();

        let mut absolute_path = "/".to_string();
        for cur_node_filename in path_str.split('/') {
            if cur_node_filename.is_empty() {
                continue;
            }
            let parent_abs_path = absolute_path.clone();
            absolute_path.push('/');
            absolute_path.push_str(cur_node_filename);

            if dirs_map.get(&absolute_path).is_none() {
                let cur_node = Arc::new(Mutex::new(ZipNode {
                    file_name: cur_node_filename.to_string(),
                    absolute_path: absolute_path.clone(),
                    ..Default::default()
                }));
                dirs_map[&parent_abs_path]
                    .lock()
                    .unwrap()
                    .children
                    .push(cur_node.clone());
                dirs_map.insert(absolute_path.clone(), cur_node);
            }
        }
    }
    let mut ret = Vec::new();
    for (_, node) in dirs_map.into_iter() {
        let mut node_ = node.lock().unwrap();
        node_.has_children = node_.children.is_empty();
        if node_.is_top_level {
            drop(node_);
            let next_node = Arc::try_unwrap(node).unwrap().into_inner().unwrap();
            for v in next_node.children.into_iter() {
                ret.push(v);
            }
            break;
        }
    }

    Ok(ret)
}

#[cfg(not)]
#[tokio::test]
async fn test_zip_files_to_tree() {
    let nodes = preview_zip_file_list(
        &std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_cases/two_files_and_a_dir.zip"
        ))
        .to_path_buf(),
    )
    .await
    .unwrap();
    let mut nodes_file_name = Vec::new();
    for node in nodes {
        nodes_file_name.push(node.lock().await.file_name.clone());
    }
    assert_eq!(nodes_file_name, vec![
        "usr",
        "settings.json",
        "keybindings.json"
    ]);
}

#[test]
fn test_tar_gz_files_to_tree() {
    let nodes = preview_gzip_file_list(
        &std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_cases/two_files_and_a_dir.tgz"
        ))
        .to_path_buf(),
    )
    .unwrap();
    assert_eq!(
        nodes
            .into_iter()
            .map(|node| node.lock().unwrap().file_name.clone())
            .collect::<Vec<_>>(),
        vec!["usr", "settings.json", "keybindings.json"]
    );
}

#[test]
#[ignore]
fn t() {
    dbg!(preview_zip_file_list(&std::path::Path::new("/data/f1.zip").to_path_buf()).unwrap());
}
