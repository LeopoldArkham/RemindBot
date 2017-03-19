extern crate hyper;
extern crate hyper_native_tls;
extern crate json;

use std::io::Read;

// Hyper
use hyper::Client;
use hyper::header::{Authorization, UserAgent};
use hyper::net::HttpsConnector;

// hyper_native_tls
use hyper_native_tls::NativeTlsClient;


// A second mention within one thread overrides the previous one.
// Solution: Poll fast then mark thread as read.

// TODO: Use Modified-Since headers to limit API polling exhaustion
// TODO: Identify reply address for Issue-threads
// TODO: Generalize issue workflow to PR's
// TODO: Auto API polling every 2 seconds

// struct Mention {

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

    // Serialize mentions
    for n in parsed.members() {
        if n["reason"] == "mention" {
            let comment_url = n["subject"]["latest_comment_url"].as_str().unwrap();
            println!("{}", comment_url);
            // let mut resp =
            //     client.get(comment_url).header(auth.clone()).header(agent.clone()).send().unwrap();
            // buf.drain(..);
            // let _ = resp.read_to_string(&mut buf);
            // let parsed_comment = json::parse(&buf).unwrap();

            // let issue_url = if parsed_comment["issue_url"] == json::JsonValue::Null {
            //     parsed_comment["url"].as_str().unwrap()
            // } else {
            //     parsed_comment["issue_url"].as_str().unwrap()
            // };

            // let reply_url = format!("{}/comments", issue_url);

            // // println!("reply_url: {}\nBody: {}\nAuthor: {}\n",
            // //          reply_url,
            // //          parsed_comment["body"],
            // //          parsed_comment["user"]["login"]);

            // let body = format!("{{ \"body\": \"@{} I serve at your will, O Ineffable One.\"}}", parsed_comment["user"]["login"]);
            // println!("{}", body);
            // let mut resp = client.post(&reply_url).header(auth.clone()).header(agent.clone()).body(&body).send().unwrap();
            // // println!("{}", resp.headers);
        }
    }
}
