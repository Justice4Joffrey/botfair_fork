// import requests
//
// #openssl x509 -x509toreq -in certificate.crt -out CSR.csr -signkey privateKey.key
//
//
// payload = 'username=myusername&password=password'
// headers = {'X-Application': 'SomeKey', 'Content-Type': 'application/x-www-form-urlencoded'}
//
// resp = requests.post('', data=payload, cert=('client-2048.crt', 'client-2048.key'), headers=headers)
//
// if resp.status_code == 200:
//   resp_json = resp.json()
//   print resp_json['loginStatus']
//   print resp_json['sessionToken']
// else:
//   print "Request failed."

#[macro_use]
extern crate log;

use reqwest::{Client, Identity};
use serde::{Deserialize, Serialize};
use std::fs;

mod generated_api;
mod json_rpc;

const CERTLOGIN_URI: &str =
    "https://identitysso-cert.betfair.com/api/certlogin";
const JSONRPC_URI: &str =
    "https://api.betfair.com/exchange/betting/json-rpc/v1";
const PFXFILE: &str = "/home/esotericnonsense/betfair/identity.pfx";
const APPKEYFILE: &str = "/home/esotericnonsense/betfair/betfair-app-key";
const USERFILE: &str = "/home/esotericnonsense/betfair/betfair-user";
const PASSFILE: &str = "/home/esotericnonsense/betfair/betfair-pass";

#[derive(Debug)]
pub enum AnyError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Other,
}

impl From<std::io::Error> for AnyError {
    fn from(e: std::io::Error) -> Self {
        AnyError::Io(e)
    }
}

impl From<reqwest::Error> for AnyError {
    fn from(e: reqwest::Error) -> Self {
        AnyError::Reqwest(e)
    }
}

#[derive(Debug, Serialize)]
struct LoginRequestForm {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct LoginResponse {
    sessionToken: Option<String>,
    loginStatus: String, // TODO enum this
}

fn get_session_token() -> Result<String, AnyError> {
    let username = fs::read_to_string(USERFILE)?.replace("\n", "");
    let password = fs::read_to_string(PASSFILE)?.replace("\n", "");

    let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:40001")?;
    let ident =
        Identity::from_pkcs12_der(std::fs::read(PFXFILE)?.as_slice(), "")?;
    let cl: Client = Client::builder().identity(ident).proxy(proxy).build()?;

    let appheader = format!("{}", rand::random::<u128>());

    let login_request_form = LoginRequestForm { username, password };

    info!("LoginRequest ...");
    let login_response: LoginResponse = cl
        .post(CERTLOGIN_URI)
        .header("X-Application", appheader)
        .form(&login_request_form)
        .send()?
        .json()?;

    info!("LoginResponse: {:?}", login_response.loginStatus);

    match login_response.sessionToken {
        Some(token) => Ok(token),
        None => Err(AnyError::Other),
    }
}

fn make_request_builder(
    session_token: &str,
) -> Result<reqwest::RequestBuilder, AnyError> {
    let app_key = fs::read_to_string(APPKEYFILE)?.replace("\n", "");
    let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:40001")?;
    let cl: Client = Client::builder().proxy(proxy).build()?;

    Ok(cl
        .post(JSONRPC_URI)
        .header("X-Application", app_key)
        .header("X-Authentication", session_token))
}

use generated_api::*;

fn main() -> Result<(), AnyError> {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();

    let session_token = get_session_token()?;

    let catalogues: Vec<MarketCatalogue> = listMarketCatalogue(
        make_request_builder(&session_token)?,
        MarketFilter::default(),
        None,
        None,
        10,
        None,
    )?;
    for catalogue in catalogues.iter() {
        info!(
            "{} {} {:?}",
            catalogue.marketId, catalogue.marketName, catalogue.totalMatched
        );
    }

    let market_ids: Vec<MarketId> = catalogues
        .iter()
        .map(|x: &MarketCatalogue| x.marketId.clone())
        .collect();

    let books: Vec<MarketBook> = listMarketBook(
        make_request_builder(&session_token)?,
        market_ids,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;
    // info!("{:?}", books);

    let s: String = serde_json::to_string(&books).expect("whatever");
    println!("{}", s);
    Ok(())
}
