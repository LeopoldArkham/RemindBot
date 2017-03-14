extern crate hyper;
extern crate hyper_native_tls;
extern crate json;

use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, UserAgent};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

// PAT : 303328dd10a0e3b8f13ff3606f9af7e581eea283

// Webhooks for notifications?
// A second mention within one thread overrides the previous one

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let auth = Authorization("token 303328dd10a0e3b8f13ff3606f9af7e581eea283".to_string());
    let agent = UserAgent("remindbot".to_string());

    let mut resp = client.get("https://api.github.com/notifications")
        .header(auth.clone())
        .header(agent.clone())
        .send()
        .unwrap();

    // println!("{}", resp.headers);
    let mut buf = String::new();

    let _ = resp.read_to_string(&mut buf);

    let parsed = json::parse(&buf).unwrap();

    for n in parsed.members() {
        if n["reason"] == "mention" {
            let comment_url = n["subject"]["latest_comment_url"].as_str().unwrap();
            let mut resp =
                client.get(comment_url).header(auth.clone()).header(agent.clone()).send().unwrap();
            buf.drain(..);
            let _ = resp.read_to_string(&mut buf);
            let parsed_comment = json::parse(&buf).unwrap();
            println!("url: {}\nBody: {}\nAuthor: {}\n",
                     parsed_comment["url"],
                     parsed_comment["body"],
                     parsed_comment["user"]["login"]);
        }
    }
}
