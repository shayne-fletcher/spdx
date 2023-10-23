use crate::{LicenseExceptionId, LicenseId};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseList {
    pub license_list_version: String,
    pub licenses: Vec<LicenseSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseExceptionList {
    pub license_list_version: String,
    pub release_date: chrono::NaiveDate,
    pub exceptions: Vec<LicenseExceptionSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseSummary {
    pub reference: String,
    pub is_deprecated_license_id: bool,
    pub details_url: String,
    pub reference_number: i32,
    pub name: String,
    pub license_id: LicenseId,
    pub see_also: Vec<Url>,
    pub is_osi_approved: bool,
    pub is_fsf_libre: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseExceptionSummary {
    pub reference: String,
    pub is_deprecated_license_id: bool,
    pub details_url: String,
    pub name: String,
    pub reference_number: i32,
    pub see_also: Vec<Url>,
    pub license_exception_id: LicenseExceptionId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseDetail {
    pub is_deprecated_license_id: bool,
    pub license_text: String,
    pub standard_license_template: String,
    pub name: String,
    pub license_id: LicenseId,
    pub cross_ref: Vec<LicenseDetailCrossRef>,
    pub see_also: Vec<Url>,
    pub is_osi_approved: bool,
    pub license_text_html: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseDetailCrossRef {
    pub match_: String,
    pub url: Url,
    pub is_valid: bool,
    pub is_live: bool,
    pub timestamp: DateTime<FixedOffset>,
    pub is_way_back_link: bool,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseExceptionDetail {
    pub is_deprecated_license_id: bool,
    pub license_exception_text: String,
    pub name: String,
    pub license_comments: String,
    pub see_also: Vec<Url>,
    pub license_exception_id: LicenseExceptionId,
    pub exception_text_html: String,
}
