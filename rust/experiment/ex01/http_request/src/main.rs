use std::collections::HashMap;

// fn main() -> Result<(), Box<std::error::Error>> {
// let mut map = HashMap::new();
// map.insert("text", "from-rust_to-slack");
// let params = [("payload", &map)];

// let mut payload = HashMap::new();
// payload.insert("payload", map);
// println!("{:?}", params);

// let client = reqwest::Client::new();
// let result = client
//     .post("http://httpbin.org/post")
//     .body("the exact body that is sent")
//     .send();
// Ok(())
//     let text = "Hello, World!";
//     let req_body = format!("{{\"text\": {} }}", text);

// let url = "https://httpbin.org/anything";
//     let result = client
//         .post(url)
//         .header(reqwest::header::CONTENT_TYPE, "application_json")
//         .body(req_body)
//         .send();
//     Ok(())
// }

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let url = "https://httpbin.org/anything";

    let text = "Hello, World!";
    let req_body = format!("{{\"text\": \"{}\" }}", text);

    let res = client
        .post(url)
        .body(req_body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let t = res.text().await?;

    println!("{}", t);
    // {
    //   "args": {},
    //   "data": "arbitrary text",
    //   "files": {},
    //   "form": {},
    //   "headers": {
    //     "Accept": "*/*",
    //     "Content-Length": "14",
    //     "Host": "httpbin.org",
    //     "X-Amzn-Trace-Id": "Root=1-604538df-218a5bb97264e7130c298b23",
    //     "X-Person-First": "Foo!",
    //     "X-Person-Last": "Bar!!"
    //   },
    //   "json": null,
    //   "method": "POST",
    //   "origin": "49.206.4.160",
    //   "url": "https://httpbin.org/anything"
    // }

    Ok(())
}
