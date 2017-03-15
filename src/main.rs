extern crate hyper;
extern crate hyper_native_tls;
extern crate json;

use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, UserAgent};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;


// Webhooks for notifications?
// A second mention within one thread overrides the previous one

// TODO: Use Modified-Since headers to limit API polling exhaustion
// TODO: Identify reply address for Issue-threads
// TODO: Generalize issue workflow to PR's


fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let tok = include_str!("../token.txt");
    let auth = Authorization(format!("token {}", tok));
    let agent = UserAgent("remindbot".to_string());

    let mut resp = client.get("https://api.github.com/notifications")
        .header(auth.clone())
        .header(agent.clone())
        .send()
        .unwrap();

    // println!("{}", resp.headers);
    let mut buf = String::new();

    let _ = resp.read_to_string(&mut buf);
    // println!("{}", buf);
    let parsed = json::parse(&buf).unwrap();

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
