use anyhow::Result;

use serde::{Deserialize, Serialize};

use virt::interface::Interface;
use virt::storage_pool::StoragePool;
use virt::storage_vol::StorageVol;

use quickxml_to_serde::{xml_string_to_json, Config};

use crate::errors::AppError;
use crate::virt_connection::VirtConnection;

#[derive(Serialize, Deserialize)]
struct VolumeInfo {
    name: String,
    path: String,
    kind: u32,
    capacity: u64,
    allocation: u64,
}

impl VolumeInfo {
    fn new(name: String, path: String, kind: u32, capacity: u64, allocation: u64) -> Self {
        return VolumeInfo {
            name,
            path,
            kind,
            capacity,
            allocation,
        };
    }
}

pub async fn get_storage() -> Result<String, AppError> {
    let conn = VirtConnection::new()?.connect()?;
    let pool = StoragePool::lookup_by_name(&conn, "default")?;

    let mut vol_list = vec![];

    for volume_name in pool.list_volumes()? {
        let storage_volume = StorageVol::lookup_by_name(&pool, &volume_name)?;
        let info = storage_volume.get_info()?;

        vol_list.push(VolumeInfo::new(
            storage_volume.get_name().unwrap_or_default(),
            storage_volume.get_path().unwrap_or_default(),
            info.kind,
            info.capacity,
            info.allocation,
        ));
    }

    VirtConnection::disconnect(conn)?;
    return Ok(serde_json::to_string_pretty(&vol_list)?);
}

pub async fn get_networks() -> Result<String, AppError> {
    let conn = VirtConnection::new()?.connect()?;
    let network_list = conn.list_networks()?;

    VirtConnection::disconnect(conn)?;
    return Ok(serde_json::to_string_pretty(&network_list)?);
}

pub async fn get_interfaces() -> Result<String, AppError> {
    let conn = VirtConnection::new()?.connect()?;

    let mut ifaces = vec![];
    let interfaces = conn.list_interfaces()?;
    for interface in interfaces {
        let interface = Interface::lookup_by_name(&conn, &interface)?;
        ifaces.push(interface.get_xml_desc(0)?);
    }
    VirtConnection::disconnect(conn)?;
    return Ok(serde_json::to_string_pretty(&xml_string_to_json(
        ifaces.join(","),
        &Config::new_with_defaults(),
    )?)?);
}
