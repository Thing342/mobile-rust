use serde::{Deserialize,Serialize};

use reqwest::blocking as request;

#[derive(Debug, Serialize, Deserialize)]
pub struct CycloneMessage {
    #[serde(rename = "atcfID")]
    atcf_id: Option<String>,
    #[serde(rename = "issuingUnit")]
    issuing_unit: Option<String>,
    #[serde(rename = "messageType")]
    message_type: Option<String>,
    #[serde(rename = "messageBinNumber")]
    message_bin_number: Option<String>,
    #[serde(rename = "advisoryNumber")]
    advisory_number: Option<String>,
    #[serde(rename = "messageDateTimeLocal")]
    message_date_time_local: Option<String>,
    #[serde(rename = "messageDateTimeUTC")]
    message_date_time_utc: Option<String>,
    #[serde(rename = "messageDateTimeUTC24")]
    message_date_time_utc24: Option<String>,
    #[serde(rename = "messageDateTimeLocalStr")]
    message_date_time_local_str: Option<String>,
    #[serde(rename = "timeEpochSeconds")]
    time_epoch_seconds: Option<String>,
    #[serde(rename = "systemType")]
    pub system_type: String,
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "centerLocLatitude")]
    center_loc_latitude: Option<String>,
    #[serde(rename = "centerLocLongitude")]
    center_loc_longitude: Option<String>,
    #[serde(rename = "centerLocLatitudeExpanded")]
    center_loc_latitude_expanded: Option<String>,
    #[serde(rename = "centerLocLongitudeExpanded")]
    center_loc_longitude_expanded: Option<String>,
    #[serde(rename = "systemIntensityMph")]
    pub system_intensity_mph: String,
    #[serde(rename = "systemIntensityKph")]
    system_intensity_kph: Option<String>,
    #[serde(rename = "systemIntensityKts")]
    system_intensity_kts: Option<String>,
    #[serde(rename = "systemMslpMb")]
    system_mslp_mb: Option<String>,
    #[serde(rename = "systemMslpInHg")]
    system_mslp_in_hg: Option<String>,
    #[serde(rename = "systemSaffirSimpsonCategory")]
    system_saffir_simpson_category: Option<String>,
    #[serde(rename = "formationChancePct48h")]
    formation_chance_pct48_h: Option<String>,
    #[serde(rename = "formationChancePct5d")]
    formation_chance_pct5_d: Option<String>,
    #[serde(rename = "systemDirectionOfMotion")]
    system_direction_of_motion: Option<String>,
    #[serde(rename = "systemSpeedMph")]
    system_speed_mph: Option<String>,
    #[serde(rename = "systemSpeedKph")]
    system_speed_kph: Option<String>,
    #[serde(rename = "systemSpeedKts")]
    system_speed_kts: Option<String>,
    #[serde(rename = "systemGeoRefPt1")]
    system_geo_ref_pt1: Option<String>,
    #[serde(rename = "systemGeoRefPt2")]
    system_geo_ref_pt2: Option<String>,
    message: Option<String>,
}

#[derive(Debug)]
pub enum ATCFError {
    ReqwestError(reqwest::Error),
    XMLError(serde_xml_rs::Error),
    Other(String),
}

impl From<reqwest::Error> for ATCFError {
    fn from(error: reqwest::Error) -> Self {
        ATCFError::ReqwestError(error)
    }
}

impl From<serde_xml_rs::Error> for ATCFError {
    fn from(error: serde_xml_rs::Error) -> Self {
        ATCFError::XMLError(error)
    }
}

const ATCF_BACKEND: &str = "https://ftp.nhc.noaa.gov/atcf";

pub fn get_atcf_info(atcf_id: &str) -> Result<CycloneMessage, ATCFError> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/adv/{}_info.xml", ATCF_BACKEND, atcf_id);
    let body = client.get(&url)
        .send()?
        .text()?;

    println!("{} {}", url, body);

    //let parsed = serde_xml_rs::from_str::<CycloneMessage>(&body)?;
    Ok(serde_xml_rs::from_str::<CycloneMessage>(&body)?)
    //Ok(body)
}

mod tests {
    #[test]
    fn test_get() {
        let data = super::get_atcf_info("al182021").unwrap();
        println!("{:#?}", data)
    }
}
