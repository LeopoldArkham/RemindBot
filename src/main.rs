#[macro_use]
extern crate lazy_static;

extern crate hyper;
extern crate hyper_native_tls;
extern crate json;

use std::io::Read;
use std::time::Duration;
use std::thread;

// Hyper
use hyper::Client;
use hyper::header::{Authorization, UserAgent};
use hyper::net::HttpsConnector;
use hyper::method::Method;

// hyper_native_tls
use hyper_native_tls::NativeTlsClient;

// Up next:
// - Cleanup pass
// - Natural time processing in separate file!()
// - Remove all unwraps
// - Handle actual reminders



struct Reminder {
    author: String,
    reply_url: String,
    // time:
}

lazy_static! {
    static ref  auth: Authorization<String> = Authorization(format!("token {}", include_str!("../token.txt")));
    static ref  agent: UserAgent =  UserAgent("RemindBot".to_string());
}

// impl Reminder {
//     fn new<T: Into<String>>(_author: T, _reply_url: T) -> Reminder {
//         Reminder {
//             author: _author.into(),
//             reply_url: _reply_url.into()
//         }
//     }
// }

fn get_notifications(client: &Client) -> json::JsonValue {
    authenticated_request(client, "https://api.github.com/notifications", Method::Get, None)
}

fn mark_as_read(client: &Client, id: &str) -> json::JsonValue {
    authenticated_request(client, &format!("https://api.github.com/notifications/threads/{}", id), Method::Patch, None)
}

fn authenticated_request(client: &Client, url: &str, _method: Method, body: Option<&str>) -> json::JsonValue {
    // Rework this later
    let method = match _method {
        Method::Get => Client::get,
        Method::Post => Client::post,
        Method::Patch => Client::patch,
        _ => unreachable!(),
    };
    
    // Static request builder that's pre-authenticated?
    let mut resp = method(client, url)
        .header(auth.clone())
        .header(agent.clone())
        .body(body.unwrap_or(""))
        .send()
        .expect("Authenticated request failure");
    let mut buf = String::new();
    let _ = resp.read_to_string(&mut buf);
    json::parse(&buf).unwrap()
}

fn main() {
    // Setting up TLS
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    
    let delay = Duration::from_secs(120);
    
    loop {
        let notifications = get_notifications(&client);
        for notif in notifications.members() {
            let thread_id = notif["id"].as_str().expect("ID not found");
            let latest_comment_url = notif["subject"]["latest_comment_url"].as_str().expect("Latest comment url not found");
            let latest_comment = authenticated_request(&client, latest_comment_url, Method::Get, None);
            
            let author = latest_comment["user"]["login"].as_str().unwrap();
            let issue_url = if latest_comment["issue_url"] == json::JsonValue::Null {
                    latest_comment["url"].as_str().unwrap()
                } else {
                    latest_comment["issue_url"].as_str().unwrap()
                };
            let reply_url = format!("{}/comments", issue_url);
            
            let body = format!("{{ \"body\": \"@{} I serve at your will, O Ineffable One.\"}}", author);
            let _ = authenticated_request(&client, &reply_url, Method::Post, Some(&body)); // <--- Hehe.

            let _ = mark_as_read(&client, thread_id);

        }
        thread::sleep(delay);
    }
    // Notification -> Comment -> Issue url -> Append /comments -> reply url
    //                        |-> Comment Body
    //                        |-> Comment author 


    // // println!("{}", resp.headers);
}
