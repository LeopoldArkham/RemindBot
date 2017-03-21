extern crate hyper;
extern crate hyper_native_tls;
extern crate json;

use std::io::Read;

// Hyper
use hyper::Client;
use hyper::header::{Authorization, UserAgent};
use hyper::net::HttpsConnector;
// use hyper::method::Method as hyper::Method;

// hyper_native_tls
use hyper_native_tls::NativeTlsClient;


// A second mention within one thread overrides the previous one.
// Solution: Poll fast then mark thread as read.

// TODO: Use Modified-Since headers to limit API polling exhaustion
// TODO: Identify reply address for Issue-threads
// TODO: Generalize issue workflow to PR's
// TODO: Auto API polling every 2 seconds

// Struct reminder:
// Author
// reply_url
// Reminder time

// struct Reminder {
//     author: String,
//     reply_url: String,
//     // time:
// }

// impl Reminder {
//     // fn new<T: Into<String>>(_author: T, _reply_url) -> Reminder {
//     //     Reminder {
//     //         author: _author.into(),
//     //         reply_url: _reply_url.into()
//     //     }
//     // }

//     fn from_parsed_comment(parsed: JsonValue) -> Reminder {
//         let comment_url = n["subject"]["latest_comment_url"].as_str().unwrap();
//         let mut resp = client.get(comment_url)
//             .header(auth.clone())
//             .header(agent.clone())
//             .send()
//             .unwrap();
//         let mut buf = String::new()
//         let _ = resp.read_to_string(&mut buf);
//         let parsed_comment = json::parse(&buf).unwrap();

//         let author = parsed_comment["user"]["login"];
//         let issue_url = if parsed_comment["issue_url"] == json::JsonValue::Null {
//                 parsed_comment["url"].as_str().unwrap()
//             } else {
//                 parsed_comment["issue_url"].as_str().unwrap()
//             };
//         let reply_url = format!("{}/comments", issue_url);

//         Reminder::new(author, reply_url)
//     }
// }

// fn authenticated_request(_url: String, _method: hyper::Method) -> String {
//     let method = match _method {
//         Method::Get => Client::get
//         Method::Post => Client:post
//         _ => unreachable!()
//     } 
// }

fn main() {
    // Setting up TLS
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    // Setting up GitHub authentication
    let tok = include_str!("../token.txt");
    let auth = Authorization(format!("token {}", tok));
    let agent = UserAgent("remindbot".to_string());

    // Send request and parse response containing notifications
    let mut resp = client.get("https://api.github.com/notifications")
        .header(auth.clone())
        .header(agent.clone())
        .send()
        .unwrap();
    let mut buf = String::new();
    let _ = resp.read_to_string(&mut buf);
    let parsed = json::parse(&buf).unwrap(); // <---------- Product


    // Notification -> Comment -> Issue url -> Append /comments -> reply url
    //                        |-> Comment Body
    //                        |-> Comment author 

    // Serialize mentions
    for n in parsed.members() {
        if n["reason"] == "mention" {
            // let body = format!("{{ \"body\": \"@{} I serve at your will, O Ineffable One.\"}}", parsed_comment["user"]["login"]);
            // println!("{}", body);
            // let mut resp = client.post(&reply_url).header(auth.clone()).header(agent.clone()).body(&body).send().unwrap();
            // // println!("{}", resp.headers);
        }
    }
}
