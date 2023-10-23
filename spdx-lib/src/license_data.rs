use crate::{LicenseExceptionId, LicenseId};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseSummary {
    #[serde(rename = "reference")]
    pub reference: Url,
    #[serde(rename = "isDeprecatedLicenseId")]
    pub is_deprecated_licence_id: bool,
    #[serde(rename = "detailsUrl")]
    pub details_url: Url,
    #[serde(rename = "referenceNumber")]
    pub reference_umber: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "licenseId")]
    pub license_id: LicenseId,
    #[serde(rename = "seeAlso")]
    pub see_also: Vec<Url>,
    #[serde(rename = "isOsiApproved")]
    pub is_osi_approved: bool,
    #[serde(rename = "isFsfLibre")]
    pub is_fsf_libre: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseException {
    #[serde(rename = "isDeprecatedLicenseId")]
    pub is_deprecated_license_id: bool,
    #[serde(rename = "licenseExceptionText")]
    pub license_exception_text: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "licenseComments")]
    pub license_comments: String,
    #[serde(rename = "seeAlso")]
    pub see_also: Vec<Url>,
    #[serde(rename = "licenseExceptionId")]
    pub license_exception_id: LicenseExceptionId,
    #[serde(rename = "exceptionTextHtml")]
    pub exception_text_html: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseDetail {
    #[serde(rename = "isDeprecatedLicenseId")]
    pub is_deprecated_license_id: bool,
    #[serde(rename = "licenseText")]
    pub license_text: String,
    #[serde(rename = "standardLicenseTemplate")]
    pub standard_license_template: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "licenseId")]
    pub license_id: LicenseId,
    #[serde(rename = "crossRef")]
    pub cross_ref: Vec<LicenseDetailCrossRef>,
    #[serde(rename = "seeAlso")]
    pub see_also: Vec<Url>,
    #[serde(rename = "isOsiApproved")]
    pub is_osi_approved: bool,
    #[serde(rename = "licenseTextHtml")]
    pub license_text_html: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseDetailCrossRef {
    #[serde(rename = "match")]
    pub match_: String,
    #[serde(rename = "url")]
    pub url: Url,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "isLive")]
    pub is_live: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<FixedOffset>,
    #[serde(rename = "isWayBackLink")]
    pub is_way_back_link: bool,
    #[serde(rename = "order")]
    pub order: i32,
}
