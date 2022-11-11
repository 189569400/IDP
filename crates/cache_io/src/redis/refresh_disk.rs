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

use bb8_redis::redis;
use err::ErrorTrace;
use redis::AsyncCommands;
use tracing::error;

use crate::keys::ipynb_key;
use crate::RefreshDto;

pub(crate) async fn spawn_refresh_disk(
    pool: bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
    mut refresh_receiver: tokio::sync::mpsc::Receiver<RefreshDto>,
) {
    while let Some(refresh_dto) = refresh_receiver.recv().await {
        if let Err(err) = refresh_disk(refresh_dto, &pool).await {
            error!("{err:#?}");
        }
    }
    error!("panicked! refresh_receiver EOF! all tx was close");
}

#[tracing::instrument]
async fn refresh_disk(
    refresh_dto: RefreshDto,
    // https://github.com/djc/bb8/issues/95
    // Response was of incompatible type: \"Response type not vector compatible.\" (response was status(\"PONG\"))
    // can't use both bb8 and aio connection, we should use bb8 pool only
    // connection: &mut redis::aio::Connection,
    pool: &bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
) -> Result<(), ErrorTrace> {
    let mut connection = pool.get().await?;
    let key = ipynb_key(&refresh_dto.path, refresh_dto.project_id);
    let val_vec = connection.hvals::<_, Vec<String>>(key).await?;
    let notebook = crate::redis_hvals_to_notebook(val_vec)?;

    // FIXME temporary solution: writing notebook is forbidden when it's cells is empty.
    if notebook.cells.is_empty() {
        // user can't delete all cell
        return Err(ErrorTrace::new(
            "panicked notebook cells is empty, skip write notebook to disk",
        ));
    }
    common_tools::file_tool::write_notebook_to_disk(refresh_dto.path, &notebook).await
}
