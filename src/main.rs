#[derive(rust_embed::RustEmbed)]
#[folder = "ui/dist/"]
struct Assets;

use virt::connect::Connect;
use virt::error::Error;
use virt::sys;

use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::Response,
    routing::{delete, get, patch},
    Router,
};

use virtman::virt_lookups::*;
use virtman::virtual_machine::VirtualMachine;

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

                    if let Ok((sched_type, nparams)) = dom.get_scheduler_type() {
                        println!("SchedType: {}, nparams: {}", sched_type, nparams);
                    }
                }
            }
            return Ok(());
        }
    }
    Err(Error::last_error())
}

#[tokio::main]
async fn main() {
    //let shared_state = Arc::new(AppState { libvirtConn: conn });

    // build our application with a single route
    let app = Router::new()
        .fallback(static_handler)
        //.route("/", get(|| async { "Hello, World!" }))
        .route("/api/vm", get(VirtualMachine::all))
        .route("/api/vm/:name", delete(VirtualMachine::delete))
        .route("/api/vm/:name/start", patch(VirtualMachine::start))
        .route("/api/vm/:name/suspend", patch(VirtualMachine::suspend))
        .route("/api/vm/:name/resume", patch(VirtualMachine::resume))
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
