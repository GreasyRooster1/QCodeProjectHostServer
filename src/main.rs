use std::collections::HashMap;
use std::{fs, io};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;
use std::sync::Mutex;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::http::Uri;
use futures::executor::block_on;
use log::{debug, error, info, warn};
use simplelog::*;
use derive_more::Display;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub const THREAD_POOL_SIZE:usize = 64;
pub const NOT_FOUND_PAGE:&str = include_str!("../404.html");

pub const HOST_IP:&str = "0.0.0.0";
pub const HOST_PORT:&str = "80";
pub const BLOCK_INDEXING:bool = true;

//pub const WHITELIST_EXTENSIONS: [&str;16] = ["png","jpg","wav","mp3","html","css","js","jsx","ts","tsx","jpeg","webp","txt","csv","json","http"];
pub const BLACKLIST_HOSTS: [&str;1] = ["code"];

#[derive(Debug, Clone)]
struct SessionData {
    username: String,
    token: String,
}


// #[tokio::main]
async fn bye() {
    //
    // CombinedLogger::init(
    //     vec![
    //         TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
    //         WriteLogger::new(LevelFilter::Info, Config::default(), File::create("logs/logs.log").unwrap()),
    //     ]
    // ).unwrap();
    //
    // let gcp_service_account = credentials_provider().await.unwrap();
    // // Create live (not emulated) context for Firebase app
    // let live_app = App::live(gcp_service_account.into()).await.unwrap();
    // let auth_admin = live_app.auth();
    // let live_token_verifier = live_app.id_token_verifier().await.unwrap();
    //
    // let address = format!("{HOST_IP}:{HOST_PORT}");
    // println!("Now listening on {address}");
    //
    // let sessions_storage: Mutex<HashMap<String, SessionData>> = Mutex::new(HashMap::new());
    //
    // let cert = include_str!("../cert/cacert.pem").as_bytes().to_vec();
    // let pkey = include_str!("../cert/cakey.pem").as_bytes().to_vec();
    // let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjkwOTg1NzhjNDg4MWRjMDVlYmYxOWExNWJhMjJkOGZkMWFiMzRjOGEiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vcWNvZGUtY2RmYzYiLCJhdWQiOiJxY29kZS1jZGZjNiIsImF1dGhfdGltZSI6MTc0MTU1NTM0NywidXNlcl9pZCI6ImpFTWgza2VtM2VNUVZsaHl2ODZsaGl2dDNTSDMiLCJzdWIiOiJqRU1oM2tlbTNlTVFWbGh5djg2bGhpdnQzU0gzIiwiaWF0IjoxNzQ1NjkwODU4LCJleHAiOjE3NDU2OTQ0NTgsImVtYWlsIjoiZ3JlYXN5cm9vc3RlcjFAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOmZhbHNlLCJmaXJlYmFzZSI6eyJpZGVudGl0aWVzIjp7ImVtYWlsIjpbImdyZWFzeXJvb3N0ZXIxQGdtYWlsLmNvbSJdfSwic2lnbl9pbl9wcm92aWRlciI6InBhc3N3b3JkIn19.FTkmJBpl8DvKyR4BRf3d7-sVzBfzrcRI6gflQlafIhPMfBf2If8DV3TzLfIeaoqLOkOhfh_qE4MaHa-RagFsywY9AJjBR0TTJ2hYLnTxOi2ShkKZfnsV7OIQy32aK3_ln2ihzHJan5pKyapNfwZGR7IS1RR8kMfrRGEvL-5-bonHB_0Z3QCA-el6spfXRQIpKY5kgNRt4biTRc6skAET1ZYm-91YT_GlgCqdTA2GA-c2rYPUUusANW-TXL1_o2FHEs6iNqai_STX15Q2Sqz0XlLlngTg-CgPGQPexBN1EDw_8FPfgoCJhkHdy2zSPFrkPysZiMlTsym7wUVKJDdbKQ";
    //
    // rouille::start_server(address, move |request| {
    //     let log_ok = |req: &Request, resp: &Response, _elap: std::time::Duration| {
    //         info!("{} {} {} {}", request.remote_addr(), req.method(), req.raw_url(),req.header("Host").unwrap());
    //     };
    //     let log_err = |req: &Request, _elap: std::time::Duration| {
    //         error!("{} Handler panicked: {} {}", request.remote_addr(), req.method(), req.raw_url());
    //     };
    //     rouille::log_custom(request, log_ok, log_err,  || {
    //         router!(request,
    //             (GET) (/) => {
    //                 debug!("{} {} {} {} redirecting to index.html ",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap());
    //                 resolve_uri(request, "index.html".to_string())
    //             },
    //
    //             (GET) (/stats) => {
    //                 panic!("Not implemented yet")
    //             },
    //
    //             _ => {
    //                 let req_path = request.url();
    //
    //                 if request.method() == "GET" {
    //
    //                 } else if request.method() == "PUT" {
    //                     info!("{} {} {} {} requested file edit",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap());
    //                     put_uri(request, req_path)
    //                 } else if request.method() == "OPTIONS" {
    //                     info!("{} {} {} {} requested CORS options",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap());
    //                     Response::empty_204()
    //                     .with_additional_header("Access-Control-Allow-Origin", "*")
    //                     .with_additional_header("Access-Control-Allow-Methods", "GET, PUT, DELETE, OPTIONS")
    //                 }else {
    //                     warn!("{} {} {} {} unknown request method",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap());
    //                     Response::empty_404()
    //                 }
    //             }
    //         )
    //     })
    // });//,cert,pkey).unwrap().run();
}

#[derive(Debug, Display)]
enum ApiError {
    #[display("Failed to parse path")]
    PathParseFailed,
    #[display("Failed to open file")]
    FileOpenFailed,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::PathParseFailed => actix_web::http::StatusCode::NOT_FOUND,
            ApiError::FileOpenFailed => actix_web::http::StatusCode::NOT_FOUND,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = dotenvy::dotenv() {
        println!("cargo:warning=Could not load .env file: {}", e);
    }

    let file_appender = rolling::Builder::new()
        .rotation(rolling::Rotation::DAILY)
        .filename_prefix("api.log")
        .max_log_files(14)
        .build("./logs")
        .expect("failed to build appender");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        // file layer — daily rotating, no ANSI colors in files
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        // stdout layer — keep console output too
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();


    HttpServer::new(||
        App::new()
        .service(serve_web)
        .default_service(web::route().to(not_found))
    )
        .bind(("localhost", 8080))?
        .run()
        .await
}

#[get("/{path:.*}")]
async fn serve_web(req: HttpRequest, path: web::Path<String>) -> Result<NamedFile,ApiError> {
    let mut uri = path.into_inner();
    if uri == "/" {
        uri ="index.html".to_string();
    }
    let host = req.headers().get("Host").unwrap().to_str().unwrap();
    let path = match get_path_from_host(host.to_string(), &uri) {
        Ok(p) => p,
        Err(e) => {
            warn!("error getting on {host}: {:?}", e);
            return Err(ApiError::PathParseFailed);
        }
    };
    info!("Requested path {:?}",path);

    let file = match NamedFile::open_async(path).await{
        Ok(f) => f,
        Err(e) => {
            warn!("error on serving file on {host}: {:?}", e);
            return Err(ApiError::FileOpenFailed);
        }
    };
    Ok(file)
}


async fn not_found(req: HttpRequest) -> impl Responder{
    warn!("Not found: {}", req.uri());
    NOT_FOUND_PAGE
}


// fn put_uri(request: &Request,uri:String)->Response {
//     //block_on(verify_token(token, &live_token_verifier));
//
//     let host = request.header("Host").unwrap();
//     let path = get_path_from_host(host.to_string(),uri).unwrap();
//     let mut buffer = String::new();
//     let pathObj = Path::new(&path);
//     let extension = pathObj.extension().unwrap().to_str().unwrap();
//
//     // if !WHITELIST_EXTENSIONS.contains(&extension){
//     //     return rouille::Response::text("forbidden extension").with_status_code(403);
//     // }
//
//     let bytes = request.data().unwrap().bytes();
//     let _ = match fs::create_dir_all(pathObj.parent().unwrap()){
//         Ok(_) => {}
//         Err(_) => {}
//     };
//     let mut file:File = File::create(&path).unwrap();
//
//     file.write_all(&[]).expect("could not clear file");
//     for byte in bytes {
//         file.write(&[byte.unwrap()]).expect("failed to write");
//     }
//
//     info!("{} {} {} {} wrote to path {:?}",request.remote_addr(), request.method(), request.raw_url(),request.header("Host").unwrap(), path);
//
//     rouille::Response::empty_204()
// }


fn get_path_from_host(host:String,uri:&String)->Result<String,String>{
    let words: Vec<_> = host.split(".").collect();
    let path = PathBuf::from_str(format!("./data/{}{}",words[0],uri).as_str()).unwrap();
    if path.components().any(|x| x == Component::ParentDir) {
        warn!("directory traversal attempted!");
        return Err("directory traversal".to_string());
    }
    Ok(path.as_path().to_str().unwrap().to_string())
}

// async fn verify_token<T: TokenVerifier>(token: &str, verifier: &T) {
//     match verifier.verify_token(token).await {
//         Ok(token) => {
//             let user_id = token.critical_claims.sub;
//             println!("Token for user {user_id} is valid!")
//         }
//         Err(err) => {
//             println!("Token is invalid because {err}!")
//         }
//     }
// }