#[macro_use]
extern crate log;

use reqwest::{Client, Identity};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

mod generated_api;
mod json_rpc;

#[derive(Debug)]
pub enum AnyError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Other,
}

type Result<T> = std::result::Result<T, AnyError>;

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

pub struct BFCredentials {
    username: String,
    password: String,
    pfx: Vec<u8>,
    app_key: String,
}

impl BFCredentials {
    fn new(
        username: String,
        password: String,
        pfx_path: String,
        app_key: String,
    ) -> Result<Self> {
        let pfx = std::fs::read(pfx_path)?;
        Ok(BFCredentials {
            username,
            password,
            pfx,
            app_key,
        })
    }

    fn as_login_request_form(&self) -> LoginRequestForm {
        LoginRequestForm {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
    fn pfx(&self) -> &Vec<u8> {
        &self.pfx
    }
    fn app_key(&self) -> &String {
        &self.app_key
    }
}

pub struct BFClient {
    client: reqwest::Client,
    session_token: Arc<RwLock<Option<String>>>,
    creds: BFCredentials,
    proxy_uri: Option<String>,
}

impl BFClient {
    pub fn new(
        creds: BFCredentials,
        proxy_uri: Option<String>,
    ) -> Result<Self> {
        let client: reqwest::Client = match &proxy_uri {
            Some(uri) => {
                let proxy = reqwest::Proxy::all(uri)?;
                Client::builder().proxy(proxy).build()?
            }
            None => reqwest::Client::new(),
        };
        Ok(BFClient {
            client,
            session_token: Arc::new(RwLock::new(None)),
            creds,
            proxy_uri,
        })
    }

    // TODO keepalive
    // https://identitysso.betfair.com/api/keepAliveo
    // Accept (mandatory)
    // Header that signals that the response should be returned as JSON	application/json
    // X-Authentication (mandatory)
    // Header that represents the session token that needs to be keep alive	Session Token
    // X-Application (optional)
    // Header the Application Key used by the customer to identify the product.	App Key
    // Response structure
    //
    //
    // {
    //   "token":"<token_passed_as_header>",
    //   "product":"product_passed_as_header",
    //   "status":"<status>",
    //   "error":"<error>"
    // }
    // Status values
    //
    //
    // SUCCESS
    // FAIL
    // Error values
    //
    //
    // INPUT_VALIDATION_ERROR
    // INTERNAL_ERROR
    // NO_SESSION

    // general notes
    // We would therefore recommend that all Betfair API request are sent with the ‘Accept-Encoding: gzip, deflate’ request header.
    // We recommend that Connection: keep-alive header is set for all requests to guarantee a persistent connection and therefore reducing latency. Please note: Idle keep-alive connection to the API endpoints are closed every 3 minutes.
    // You should ensure that you handle the INVALID_SESSION_TOKEN error within your code by creating a new session token via the API login method.

    fn login(&self) -> Result<String> {
        const CERTLOGIN_URI: &str =
            "https://identitysso-cert.betfair.com/api/certlogin";

        let ident = Identity::from_pkcs12_der(self.creds.pfx().as_slice(), "")?;

        let client: reqwest::Client = match &(self.proxy_uri) {
            Some(uri) => {
                let proxy = reqwest::Proxy::all(uri)?;
                Client::builder().identity(ident).proxy(proxy).build()?
            }
            None => Client::builder().identity(ident).build()?,
        };

        let login_request_form = self.creds.as_login_request_form();

        info!("LoginRequest ...");
        let login_response: LoginResponse = client
            .post(CERTLOGIN_URI)
            .header(
                "X-Application",
                format!("schroedinger_{}", rand::random::<u128>()),
            )
            .form(&login_request_form)
            .send()?
            .json()?;

        info!("LoginResponse: {:?}", login_response.loginStatus);

        match login_response.sessionToken {
            Some(token) => {
                let mut x = self.session_token.write().unwrap();
                *x = Some(token.clone());
                Ok(token)
            }
            None => Err(AnyError::Other),
        }
    }

    pub fn make_request_builder(&self) -> Result<reqwest::RequestBuilder> {
        const JSONRPC_URI: &str =
            "https://api.betfair.com/exchange/betting/json-rpc/v1";
        let token_guard = self.session_token.read().unwrap();
        let token_opt = token_guard.clone();
        drop(token_guard);
        let token = match token_opt {
            Some(x) => x,
            None => self.login()?,
        };
        Ok(self
            .client
            .post(JSONRPC_URI)
            .header("X-Application", self.creds.app_key())
            .header("X-Authentication", token))
    }
}

use generated_api::*;

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();

    const USER_PATH: &str = "/home/esotericnonsense/betfair/betfair-user";
    const PASS_PATH: &str = "/home/esotericnonsense/betfair/betfair-pass";
    const PFX_PATH: &str = "/home/esotericnonsense/betfair/identity.pfx";
    const APPKEY_PATH: &str = "/home/esotericnonsense/betfair/betfair-app-key";
    const PROXY_URI: &str = "socks5h://127.0.0.1:40001";

    let username = std::fs::read_to_string(USER_PATH)?.replace("\n", "");
    let password = std::fs::read_to_string(PASS_PATH)?.replace("\n", "");
    let app_key = std::fs::read_to_string(APPKEY_PATH)?.replace("\n", "");
    let bf_creds =
        BFCredentials::new(username, password, PFX_PATH.to_owned(), app_key)?;
    let bf_client = BFClient::new(bf_creds, Some(PROXY_URI.to_owned()))?;

    let catalogues: Vec<MarketCatalogue> = listMarketCatalogue(
        bf_client.make_request_builder()?,
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
        bf_client.make_request_builder()?,
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
