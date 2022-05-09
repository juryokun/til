use chrono::{Date, DateTime, Local, Utc};
use firestore_db_and_auth::{documents, documents::List, dto, Credentials, ServiceSession};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyDTD {
    record: DateTime<Local>,
    timestamp: DateTime<Local>,
}

fn update() {
    let cred = Credentials::from_file("firebase-key.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let da: MyDTD = MyDTD {
        record: Local::now(),
        timestamp: Local::now(),
    };
    let values = documents::write(
        &auth,
        "stop-watch",
        Some("AABBCCDDD"),
        &da,
        documents::WriteOptions::default(),
    );
}

fn fetch_list() -> Vec<MyDTD> {
    let cred = Credentials::from_file("firebase-key.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let documents: List<MyDTD, ServiceSession> = documents::list(&auth, "stop-watch");

    let mut rel_vec = vec![];
    for doc in documents {
        let (data, _document) = doc.unwrap();
        // println!("{:?}", data);
        // println!("{:}", data.record.format("%Y-%m-%d %H:%M:%S").to_string());
        rel_vec.push(data);
    }

    rel_vec
}

mod test {
    use super::*;

    #[test]
    fn test_list() {
        update();
        let list = fetch_list();
        for data in list {
            println!("{:?}", data);
            // println!("{:}", data.record.format("%Y-%m-%d %H:%M:%S").to_string());
        }
    }
}
