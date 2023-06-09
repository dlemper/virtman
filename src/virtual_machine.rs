use anyhow::Result;
use axum::extract::Path;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use virt::domain::Domain;
use virt::sys;

use crate::errors::AppError;
use crate::virt_connection::VirtConnection;

pub struct VirtualMachine;

impl VirtualMachine {
    pub async fn all() -> Result<String, AppError> {
       return match VmItem::get_all().await? {
           Some(vm_list) => Ok(serde_json::to_string(&vm_list)?),
           None => Ok("".to_string())
       };
    }

    pub async fn suspend(Path(name): Path<String>) -> Result<StatusCode, AppError> {
        let conn = VirtConnection::new()?.connect()?;
        let dom = Domain::lookup_by_name(&conn, &name)?;
        if dom.suspend().is_ok() {
            println!("Domain '{:?}' suspended, info: {:?}", &name, dom.get_info());
        }
        VirtConnection::disconnect(conn)?;
        return Ok(StatusCode::OK);
    }

    pub async fn start(Path(name): Path<String>) -> Result<StatusCode, AppError> {
        let conn = VirtConnection::new()?.connect()?;
        let dom = Domain::lookup_by_name(&conn, &name)?;
        if dom.suspend().is_ok() {
            println!("Domain '{:?}' start, info: {:?}", &name, dom.get_info());
        }
        VirtConnection::disconnect(conn)?;
        return Ok(StatusCode::OK);
    }

    pub async fn resume(Path(name): Path<String>) -> Result<StatusCode, AppError> {
        let conn = VirtConnection::new()?.connect()?;
        let dom = Domain::lookup_by_name(&conn, &name)?;
        if dom.suspend().is_ok() {
            println!("Domain '{:?}' start, info: {:?}", &name, dom.get_info());
        }
        VirtConnection::disconnect(conn)?;
        return Ok(StatusCode::OK);
    }

    pub async fn delete(Path(name): Path<String>) -> Result<StatusCode, AppError> {
        let conn = VirtConnection::new()?.connect()?;
        let dom = Domain::lookup_by_name(&conn, &name)?;
        if dom.suspend().is_ok() {
            println!("Domain '{:?}' start, info: {:?}", &name, dom.get_info());
        }
        VirtConnection::disconnect(conn)?;
        return Ok(StatusCode::OK);
    }
}

#[derive(Serialize, Deserialize)]
struct VmItem {
    id: u32,
    name: String,
    active: bool,
}

impl VmItem {
    fn new(id: u32, name: String, active: bool) -> Self {
        VmItem { id, name, active }
    }

    async fn get_all() -> Result<Option<Vec<VmItem>>> {
        let conn = VirtConnection::new()?.connect()?;

        let list: Option<Vec<VmItem>> = if let Ok(doms) = conn.list_all_domains(
            sys::VIR_CONNECT_LIST_DOMAINS_ACTIVE | sys::VIR_CONNECT_LIST_DOMAINS_INACTIVE,
        ) {
            let mut vm_list: Vec<VmItem> = vec![];

            for dom in doms {
                vm_list.push(VmItem::new(
                    dom.get_id().unwrap_or(0),
                    dom.get_name().unwrap_or_else(|_| String::from("no-name")),
                    dom.is_active().unwrap_or(false),
                ));
            }
            return Ok(Some(vm_list));
        } else {
            None
        };

        VirtConnection::disconnect(conn)?;
        return Ok(list);
    }
    
}
