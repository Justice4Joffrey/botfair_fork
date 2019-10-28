//! # automatically generated
//! This module has been automatically generated by botfair
//! from the Betfair APING documentation at
//! https://docs.developer.betfair.com
//!
//! Any documentation here has been generated directly from the API
//! docs.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub enum errorCode {
    /// The operation requested too much data
    #[serde(rename = "ANGX-0001")]
    TOO_MUCH_DATA,
    /// Invalid input data
    #[serde(rename = "ANGX-0002")]
    INVALID_INPUT_DATA,
    /// The session token passed is invalid
    #[serde(rename = "ANGX-0003")]
    INVALID_SESSION_INFORMATION,
    /// An application key is required for this operation
    #[serde(rename = "ANGX-0004")]
    NO_APP_KEY,
    /// A session token is required for this operation
    #[serde(rename = "ANGX-0005")]
    NO_SESSION,
    /// An unexpected internal error occurred that prevented successful request processing.
    #[serde(rename = "ANGX-0006")]
    UNEXPECTED_ERROR,
    /// The application key passed is invalid
    #[serde(rename = "ANGX-0007")]
    INVALID_APP_KEY,
    /// There are too many pending requests
    #[serde(rename = "ANGX-0008")]
    TOO_MANY_REQUESTS,
    /// The service is currently too busy to service this request
    #[serde(rename = "ANGX-0009")]
    SERVICE_BUSY,
    /// Internal call to downstream service timed out
    #[serde(rename = "ANGX-0010")]
    TIMEOUT_ERROR,
    /// The application key creation has failed
    #[serde(rename = "ANGX-0011")]
    APP_KEY_CREATION_FAILED,
    /// The application name specified already exists
    #[serde(rename = "ANGX-0012")]
    DUPLICATE_APP_NAME,
    /// The application name specified is too long
    #[serde(rename = "ANGX-0013")]
    APP_CREATION_FAILED,
    /// The request has exceeded the maximum allowed size
    #[serde(rename = "ANGX-0014")]
    REQUEST_SIZE_EXCEEDS_LIMIT,
    /// The access to this functionality is not allowed
    #[serde(rename = "ANGX-0015")]
    ACCESS_DENIED,
    /// Provided market group id does not identify a known market group
    #[serde(rename = "ANGX-0016")]
    INVALID_MARKET_GROUP,
    /// Unable to delete/update limit as it doesn't exist
    #[serde(rename = "ANGX-0017")]
    EXPOSURE_LIMIT_NOT_EXIST,
    /// Unable to unblock market group after exposure limit breach, market group is not blocked
    #[serde(rename = "ANGX-0018")]
    MARKET_GROUP_NOT_BLOCKED,
}