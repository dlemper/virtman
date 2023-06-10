#[derive(rust_embed::RustEmbed)]
#[folder = "ui/dist/"]
struct Assets;

static INDEX_HTML: &str = "index.html";

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => {
            let body = boxed(Full::from(content.data));

            Response::builder()
                .header(header::CONTENT_TYPE, "text/html")
                .body(body)
                .unwrap()
        }
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(boxed(Full::from("404")))
        .unwrap()
}

use std::env;

use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::Error;
use virt::storage_pool::StoragePool;
use virt::storage_vol::StorageVol;
use virt::sys;
use virt::interface::Interface;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VmItem {
    id: u32,
    name: String,
    active: bool,
}

#[derive(Serialize, Deserialize)]
struct VolInfo {
    name: String,
    path: String,
    kind: u32,
    capacity: u64,
    allocation: u64,
}

fn show_hypervisor_info(conn: &Connect) -> Result<(), Error> {
    if let Ok(hv_type) = conn.get_type() {
        if let Ok(mut hv_ver) = conn.get_hyp_version() {
            let major = hv_ver / 1000000;
            hv_ver %= 1000000;
            let minor = hv_ver / 1000;
            let release = hv_ver % 1000;
            println!(
                "Hypervisor: '{}' version: {}.{}.{}",
                hv_type, major, minor, release
            );
            return Ok(());
        }
    }
    Err(Error::last_error())
}

fn show_domains(conn: &Connect) -> Result<(), Error> {
    let flags = sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE | sys::VIR_CONNECT_LIST_DOMAINS_INACTIVE;

    if let Ok(num_active_domains) = conn.num_of_domains() {
        if let Ok(num_inactive_domains) = conn.num_of_defined_domains() {
            println!(
                "There are {} active and {} inactive domains",
                num_active_domains, num_inactive_domains
            );
            /* Return a list of all active and inactive domains. Using this API
             * instead of virConnectListDomains() and virConnectListDefinedDomains()
             * is preferred since it "solves" an inherit race between separated API
             * calls if domains are started or stopped between calls */
            if let Ok(doms) = conn.list_all_domains(flags) {
                for dom in doms {
                    let id = dom.get_id().unwrap_or(0);
                    let name = dom.get_name().unwrap_or_else(|_| String::from("no-name"));
                    let active = dom.is_active().unwrap_or(false);
                    println!("ID: {}, Name: {}, Active: {}", id, name, active);
                    if let Ok(dinfo) = dom.get_info() {
                        println!("Domain info:");
                        println!("    State: {}", dinfo.state);
                        println!("    Max Memory: {}", dinfo.max_mem);
                        println!("    Memory: {}", dinfo.memory);
                        println!("    CPUs: {}", dinfo.nr_virt_cpu);
                        println!("    CPU Time: {}", dinfo.cpu_time);
                    }
                    /*if let Ok(memtune) = dom.get_memory_parameters(0) {
                        println!("Memory tune:");
                        println!("    Hard Limit: {}", memtune.hard_limit.unwrap_or(0));
                        println!("    Soft Limit: {}", memtune.soft_limit.unwrap_or(0));
                        println!("    Min Guarantee: {}", memtune.min_guarantee.unwrap_or(0));
                        println!(
                            "    Swap Hard Limit: {}",
                            memtune.swap_hard_limit.unwrap_or(0)
                        );
                    }*/
                    /*if let Ok(numa) = dom.get_numa_parameters(0) {
                        println!("NUMA:");
                        println!("    Node Set: {}", numa.node_set.unwrap_or_default());
                        println!("    Mode: {}", numa.mode.unwrap_or(0));
                    }*/

                    if let Ok((sched_type, nparams)) = dom.get_scheduler_type() {
                        println!("SchedType: {}, nparams: {}", sched_type, nparams);
                    }

                    /*if let Ok(sched_info) = dom.get_scheduler_parameters() {
                        println!("Schedule Information:");
                        println!("\tScheduler\t: {}", sched_info.scheduler_type);
                        if let Some(shares) = sched_info.cpu_shares {
                            println!("\tcpu_shares\t: {}", shares);
                        }
                        if let Some(period) = sched_info.vcpu_bw.period {
                            println!("\tvcpu_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.vcpu_bw.quota {
                            println!("\tvcpu_quota\t: {}", quota);
                        }
                        if let Some(period) = sched_info.emulator_bw.period {
                            println!("\temulator_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.emulator_bw.quota {
                            println!("\temulator_quota\t: {}", quota);
                        }
                        if let Some(period) = sched_info.global_bw.period {
                            println!("\tglobal_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.global_bw.quota {
                            println!("\tglobal_quota\t: {}", quota);
                        }
                        if let Some(period) = sched_info.global_bw.period {
                            println!("\tiothread_period\t: {}", period);
                        }
                        if let Some(quota) = sched_info.global_bw.quota {
                            println!("\tiothread_quota\t: {}", quota);
                        }
                    }*/
                }
            }
            return Ok(());
        }
    }
    Err(Error::last_error())
}

/*fn main() {
    let uri = match env::args().nth(1) {
        Some(u) => u,
        None => String::from(""),
    };
    println!("Attempting to connect to hypervisor: '{}'", uri);

    let conn = match Connect::open(&uri) {
        Ok(c) => c,
        Err(e) => panic!("No connection to hypervisor: {}", e),
    };

    match conn.get_uri() {
        Ok(u) => println!("Connected to hypervisor at '{}'", u),
        Err(e) => {
            disconnect(conn);
            panic!("Failed to get URI for hypervisor connection: {}", e);
        }
    };

    if let Err(e) = show_hypervisor_info(&conn) {
        disconnect(conn);
        panic!("Failed to show hypervisor info: {}", e);
    }

    if let Err(e) = show_domains(&conn) {
        disconnect(conn);
        panic!("Failed to show domains info: {}", e);
    }

    fn disconnect(mut conn: Connect) {
        if let Err(e) = conn.close() {
            panic!("Failed to disconnect from hypervisor: {}", e);
        }
        println!("Disconnected from hypervisor");
    }
}*/

use axum::{
    body::{boxed, Full},
    extract::Path,
    http::{header, StatusCode, Uri},
    response::Response,
    routing::{delete, get, patch, post, put},
    Router,
};
/*use std::sync::Arc;

struct AppState {
    libvirtConn: Connect,
}*/

fn connect() -> Connect {
    let uri = match env::args().nth(1) {
        Some(u) => u,
        None => String::from(""),
    };
    println!("Attempting to connect to hypervisor: '{}'", uri);

    let conn = match Connect::open(&uri) {
        Ok(c) => c,
        Err(e) => panic!("No connection to hypervisor: {}", e),
    };

    match conn.get_uri() {
        Ok(u) => println!("Connected to hypervisor at '{}'", u),
        Err(e) => {
            disconnect(conn);
            panic!("Failed to get URI for hypervisor connection: {}", e);
        }
    };

    return conn;
}

fn disconnect(mut conn: Connect) {
    if let Err(e) = conn.close() {
        panic!("Failed to disconnect from hypervisor: {}", e);
    }
    println!("Disconnected from hypervisor");
}

async fn get_vms() -> String {
    let conn = connect();
    let mut ret = String::from("");

    if let Ok(doms) = conn.list_all_domains(
        sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE | sys::VIR_CONNECT_LIST_DOMAINS_INACTIVE,
    ) {
        let mut vm_list: Vec<VmItem> = Vec::new();

        for dom in doms {
            vm_list.push(VmItem {
                id: dom.get_id().unwrap_or(0),
                name: dom.get_name().unwrap_or_else(|_| String::from("no-name")),
                active: dom.is_active().unwrap_or(false),
            });
        }

        ret = serde_json::to_string(&vm_list).unwrap_or_default();
    }
    
    disconnect(conn);
    return ret;
}

async fn suspend_vm(Path(name): Path<String>) {
    let conn = connect();

    if let Ok(dom) = Domain::lookup_by_name(&conn, &name) {
        if dom.suspend().is_ok() {
            println!("Domain '{:?}' suspended, info: {:?}", name, dom.get_info());
            //thread::sleep(time::Duration::from_millis(sec * 1000));
        }
    }
    
    disconnect(conn);
    //Err(Error::last_error());
}

async fn start_vm(Path(name): Path<String>) {
    let conn = connect();
    
    if let Ok(dom) = Domain::lookup_by_name(&conn, &name) {
        if dom.create().is_ok() {
            println!("Domain '{:?}' resumed, info: {:?}", name, dom.get_info());
            return; //Ok(());
        }
    }
    
    disconnect(conn);
    //Err(Error::last_error())
}

async fn resume_vm(Path(name): Path<String>) {
    let conn = connect();
    
    if let Ok(dom) = Domain::lookup_by_name(&conn, &name) {
        if dom.resume().is_ok() {
            println!("Domain '{:?}' resumed, info: {:?}", name, dom.get_info());
            return; //Ok(());
        }
    }
    
    disconnect(conn);
    //Err(Error::last_error())
}

async fn delete_vm(Path(name): Path<String>) {
    let conn = connect();
    
    if let Ok(dom) = Domain::lookup_by_name(&conn, &name) {
        if dom.destroy().is_ok() {
            println!("Domain '{:?}' resumed, info: {:?}", name, dom.get_info());
            return; //Ok(());
        }
    }
    
    disconnect(conn);
    //Err(Error::last_error())
}

/*async fn create_vm() {
    let body = boxed(Full::from(content.data));
    let conn = connect();
    if let Ok(dom) = Domain::define_xml(&conn, &body) {
        if dom.destroy().is_ok() {
            //println!("Domain '{:?}' resumed, info: {:?}", name, dom.get_info());
            return; //Ok(());
        }
    }
    disconnect(conn);
    //Err(Error::last_error())
}*/

async fn get_storage() -> String {
    let conn = connect();
    let mut ret = String::from("");

    if let Ok(pool) = StoragePool::lookup_by_name(&conn, "default") {
        if let Ok(vols) = pool.list_volumes() {
            let mut vol_list: Vec<VolInfo> = Vec::new();

            for volname in vols {
                let vol = StorageVol::lookup_by_name(&pool, &volname).unwrap();
                let info = vol.get_info().unwrap();

                vol_list.push(VolInfo {
                    name: vol.get_name().unwrap_or_default(),
                    path: vol.get_path().unwrap_or_default(),
                    kind: info.kind,
                    capacity: info.capacity,
                    allocation: info.allocation,
                });
            }

            ret = serde_json::to_string(&vol_list).unwrap_or_default();
        }
    }

    disconnect(conn);
    return ret;
}

async fn get_networks() -> String {
    let conn = connect();
    let mut ret = String::from("");

    if let Ok(networks) = conn.list_networks() {
        ret = serde_json::to_string(&networks).unwrap_or_default();
    }

    disconnect(conn);
    return ret;
}

use serde_transcode::Transcoder;
use quickxml_to_serde::*;

async fn get_interfaces() -> String {
    let conn = connect();
    let mut ret = String::from("");

    if let Ok(interfaces) = conn.list_interfaces() {
        let ifaces: Vec<String> = interfaces.iter().map(|iface_name| {
            let interface = Interface::lookup_by_name(&conn, &iface_name).unwrap();
            let xml = interface.get_xml_desc(0).unwrap();
            return xml_string_to_json(xml, &Config::new_with_defaults()).unwrap().to_string();
        }).collect();

        ret = "[".to_owned()+&ifaces.join(",")+"]";
    }

    disconnect(conn);
    return ret;
}

#[tokio::main]
async fn main() {
    //let shared_state = Arc::new(AppState { libvirtConn: conn });

    // build our application with a single route
    let app = Router::new()
        .fallback(static_handler)
        //.route("/", get(|| async { "Hello, World!" }))
        .route("/api/vm", get(get_vms))
        .route("/api/vm/:name", delete(delete_vm))
        .route("/api/vm/:name/start", patch(start_vm))
        .route("/api/vm/:name/suspend", patch(suspend_vm))
        .route("/api/vm/:name/resume", patch(resume_vm))
        .route("/api/storage", get(get_storage))
        .route("/api/network", get(get_networks))
        .route("/api/interface", get(get_interfaces));
    //    .with_state(shared_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
