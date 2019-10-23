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

const CERTLOGIN_URI: &str =
    "https://identitysso-cert.betfair.com/api/certlogin";
const PFXFILE: &str = "/home/esotericnonsense/betfair/identity.pfx";
//const APPKEYFILE: &str = "/home/esotericnonsense/betfair/betfair-app-key";
const USERFILE: &str = "/home/esotericnonsense/betfair/betfair-user";
const PASSFILE: &str = "/home/esotericnonsense/betfair/betfair-pass";

#[derive(Debug)]
enum AnyError {
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

    let ident =
        Identity::from_pkcs12_der(std::fs::read(PFXFILE)?.as_slice(), "")?;
    let cl: Client = Client::builder().identity(ident).build()?;

    let appheader = format!("{}", rand::random::<u128>());

    let login_request_form = LoginRequestForm { username, password };
    info!("{:?}", login_request_form);
    let login_response: LoginResponse = cl
        .post(CERTLOGIN_URI)
        .header("X-Application", appheader)
        .form(&login_request_form)
        .send()?
        .json()?;

    info!("{:?}", login_response);

    match login_response.sessionToken {
        Some(token) => Ok(token),
        None => Err(AnyError::Other),
    }
}

fn main() -> Result<(), AnyError> {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();

    match get_session_token() {
        Ok(x) => {
            info!("got token {}", x);
            Ok(())
        }
        Err(e) => {
            if let AnyError::Reqwest(f) = e {
                error!("got error {}", f);
            } else {
                error!("got error {:?}", e);
            }
            Err(AnyError::Other)
        }
    }
}
