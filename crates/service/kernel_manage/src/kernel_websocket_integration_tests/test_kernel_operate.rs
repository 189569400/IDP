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

use std::net::TcpStream;

use common_model::service::rsp::CODE_SUCCESS;
use hyper_tungstenite::tungstenite;
use hyper_tungstenite::tungstenite::stream::MaybeTlsStream;
use hyper_tungstenite::tungstenite::WebSocket;
use kernel_common::Message;
use tungstenite::Message as WsMsg;

use crate::handler::execute_code::execute_req_model::CellTypeMeta;
use crate::handler::execute_code::execute_req_model::ExecuteCodeReq;
use crate::handler::kernel_list::KernelListItem;

// const DOCKER_ALL_IN_ONE_PORT: u16 = 3003;
const REGION: &str = "a";
const TEAM_ID: u64 = 1;
const PROJECT_ID: u64 = 1;
const PATH: &str = "/demo.ipynb";

#[derive(serde::Deserialize, Debug)]
struct RspDe<T> {
    code: u32,
    message: String,
    data: T,
}

fn deploy() -> u16 {
    let port = business::os_utils::get_unused_port();
    std::env::set_var("KERNEL_MANAGE_PORT", port.to_string());
    /*
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("b").arg("--bin").arg("idp_kernel");
    cmd.spawn()
        .unwrap_or_else(|_| panic!("{cmd:?} spawn err"))
        .wait()
        .unwrap_or_else(|_| panic!("{cmd:?} wait err"));
    */
    std::thread::Builder::new()
        .name("kernel_manage".to_string())
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async { crate::main().await });
        })
        .unwrap();

    let mut retry = 0;
    loop {
        if retry == 100 {
            panic!("TCP bind timeout");
        }
        if std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port)).is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        retry += 1;
    }

    // websocket connect would spawn a new process
    /*
    std::thread::Builder::new()
        .name("idp_kernel".to_string())
        .spawn(|| {
            idp_kernel::main(vec![
                "idp_kernel".to_string(),
                serde_json::to_string(&kernel_common::Header {
                    project_id: PROJECT_ID,
                    team_id: TEAM_ID,
                    path: PATH.to_string(),
                    ..Default::default()
                })
                .unwrap(),
                "ray_id".to_string()
            ]);
        })
        .unwrap();
    */
    port
}

fn ws_code_req(code: &str, cell_id: &str) -> ExecuteCodeReq {
    ExecuteCodeReq {
        header: kernel_common::Header {
            project_id: PROJECT_ID,
            team_id: TEAM_ID,
            path: PATH.to_string(),
            cell_id: cell_id.to_string(),
            ..Default::default()
        },
        batch_id: 0,
        resource: kernel_common::spawn_kernel_process::Resource::default(),
        input_reply: None,
        cell_type: CellTypeMeta::Code {},
        code: code.to_string(),
        region: REGION.to_string(),
        enable_save_session: None,
    }
}

fn connect(
    host: &str,
    port: u16,
    team_id: u64,
    project_id: u64,
) -> WebSocket<MaybeTlsStream<TcpStream>> {
    let url = format!("ws://{host}:{port}/api/v1/execute/ws/kernel/execute?projectId={project_id}");

    let mut req = tungstenite::client::IntoClientRequest::into_client_request(url).unwrap();
    req.headers_mut().insert(
        "Cookie",
        hyper::http::HeaderValue::from_str(&format!("teamId={team_id}")).unwrap(),
    );
    let (conn, _rsp) = tungstenite::connect(req).unwrap();
    conn
}

#[cfg(not)]
#[test]
#[ignore]
fn temp() {
    for (host, port, region, team_id, project_id, path) in [(
        std::env::var("HOST").unwrap(),
        80,
        "r",
        1,
        1,
        "demo/contact_us.idpnb",
    )] {
        let url = format!(
            "http://{host}:{port}/{region}/api/v2/idp-note-rs/content/cat?teamId={team_id}&projectId={project_id}&path={path}"
        );
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap();
        // let rsp = client.get(url).header("Cookie", format!("region={region}; teamId={team_id}")).send().unwrap().text().unwrap();
        let rsp = client
            .get(url)
            .header("Cookie", format!("region={region}"))
            .send()
            .unwrap()
            .text()
            .unwrap();
        let rsp = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&rsp).unwrap();
        let cells = rsp["data"]["content"]["cells"].as_array().unwrap();
        let mut cell_id = "".to_string();
        for cell in cells {
            if cell["cell_type"] == "code" {
                // cell_id = cell
                cell_id = cell["metadata"]["id"].as_str().unwrap().to_string();
            }
        }
        dbg!(cell_id);
        // let req = ws_code_req(code, cell_id);
        // let rsp = reqwest::blocking::get(url).unwrap().json::<serde_json::Map<String, serde_json::Value>>().unwrap();
        // dbg!(&rsp["data"]);
    }
}

#[test]
#[ignore]
fn test_interrupt_request() {
    logger::init_logger();
    let port = deploy();
    let base_url = format!("http://127.0.0.1:{port}/api/v1/execute");
    let req = ws_code_req(
        r#"
import time
while True:
    print('now = ', time.ctime())
    time.sleep(1)
    "#,
        "90c21aa3-5d17-4fef-b64c-640e17b24203",
    );
    let mut stream = connect("127.0.0.1", port, TEAM_ID, PROJECT_ID);
    stream
        .write_message(WsMsg::Text(serde_json::to_string(&req).unwrap()))
        .unwrap();
    loop {
        let rsp = stream.read_message().unwrap();
        let rsp = rsp.to_text().unwrap();
        if rsp.contains("now = ") {
            break;
        }
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_millis(1500))
        .build()
        .unwrap();
    let kernel_list_url = format!("{base_url}/kernel/list?projectId={PROJECT_ID}&teamId={TEAM_ID}");

    // alternative: use kernel_common hash API to get inode
    let mut retry = 0;
    let inode = loop {
        if retry == 10 {
            panic!("kernel list timeout");
        }
        let kernel_list = client
            .get(&kernel_list_url)
            .send()
            .unwrap()
            .json::<RspDe<Vec<KernelListItem>>>()
            .unwrap();
        if let Some(kernel) = kernel_list.data.get(0) {
            assert_eq!(kernel.state, "busy");
            break kernel.inode.clone();
        }
        retry += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    };

    let kernel_interrupt_url = format!("{base_url}/kernel/interrupt?inode={inode}");
    let interrupt_rsp = client
        .get(kernel_interrupt_url)
        .send()
        .unwrap()
        .json::<RspDe<()>>()
        .unwrap();
    assert_eq!(interrupt_rsp.code, CODE_SUCCESS);
    assert_eq!(interrupt_rsp.message, "success");
    tracing::info!("interrupt_rsp = {interrupt_rsp:?}");

    let mut retry = 0;
    loop {
        if retry > 250 {
            panic!("timeout: kernel state not idle after interrupt")
        }
        let kernel_list = client
            .get(&kernel_list_url)
            .send()
            .unwrap()
            .json::<RspDe<Vec<KernelListItem>>>()
            .unwrap()
            .data;
        if kernel_list[0].state == "idle" {
            break;
        }
        retry += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let shutdown_all_url = format!("{base_url}/kernel/shutdown_all?projectId={PROJECT_ID}");
    let shutdown_rsp = reqwest::blocking::get(shutdown_all_url)
        .unwrap()
        .json::<RspDe<()>>()
        .unwrap();
    tracing::info!("shutdown_rsp = {shutdown_rsp:?}");
}

#[test]
#[ignore = "WIP"]
fn test_input() {
    logger::init_logger();
    let port = deploy();
    let mut req = ws_code_req(
        r#"
input()
    "#,
        "90c21aa3-5d17-4fef-b64c-640e17b24203",
    );
    let mut stream = connect("127.0.0.1", port, TEAM_ID, PROJECT_ID);
    stream
        .write_message(WsMsg::Text(serde_json::to_string(&req).unwrap()))
        .unwrap();
    let _execute_input = stream.read_message().unwrap().into_text().unwrap();

    let input_request = stream.read_message().unwrap().into_text().unwrap();
    let input_request = serde_json::from_str::<Message>(&input_request).unwrap();
    dbg!(&input_request);
    req.input_reply = Some("user_input".to_string());
    stream
        .write_message(WsMsg::Text(serde_json::to_string(&req).unwrap()))
        .unwrap();

    let rsp = stream.read_message().unwrap().into_text().unwrap();
    dbg!(rsp);
}
