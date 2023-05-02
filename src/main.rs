#[derive(rust_embed::RustEmbed)]
#[folder = "ui/dist/"]
struct Assets;

static INDEX_HTML: &str = "index.html";

use std::env;

use virt::connect::Connect;
use virt::error::Error;
use virt::sys;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VmItem {
    id: u32,
    name: String,
    active: bool,
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

use axum::{routing::get, Json, Router};
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

async fn getVms() -> String {
    let conn = connect();
    if let Ok(doms) = conn.list_all_domains(
        sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE | sys::VIR_CONNECT_LIST_DOMAINS_INACTIVE,
    ) {
        let mut vmList: Vec<VmItem> = Vec::new();
        for dom in doms {
            vmList.push(VmItem {
                id: dom.get_id().unwrap_or(0),
                name: dom.get_name().unwrap_or_else(|_| String::from("no-name")),
                active: dom.is_active().unwrap_or(false),
            });
        }

        disconnect(conn);
        return serde_json::to_string(&vmList).unwrap_or_default();
    } else {
        return "".to_string();
    };
}

#[tokio::main]
async fn main() {
    //let shared_state = Arc::new(AppState { libvirtConn: conn });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/vm", get(getVms));
    //    .with_state(shared_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
