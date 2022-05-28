use super::*;
use chrono::{DateTime, Local, TimeZone};
use firestore_db_and_auth::{documents, documents::List, dto, Credentials, ServiceSession};
use serde::{Deserialize, Serialize};

pub struct FirestoreConnection {
    auth: ServiceSession,
}

impl FirestoreConnection {
    pub fn new() -> Self {
        let cred = Credentials::from_file("firebase-key.json").unwrap();
        let auth = ServiceSession::new(cred).unwrap();
        Self { auth: auth }
    }
}

pub trait RecordDao {
    fn push_db(&self, record: Record);
}

pub struct RecordFirestoreDao(pub FirestoreConnection);
impl RecordDao for RecordFirestoreDao {
    fn push_db(&self, record: Record) {
        #[derive(Serialize, Deserialize, Debug)]
        struct RecordFirestore {
            record: String,
            timestamp: String,
        }
        let record_sec = record.record.as_secs();
        let record_mili = record.record.as_millis();
        let data = RecordFirestore {
            record: format!("{}.{:0>2}", record_sec, record_mili),
            timestamp: record.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
        };

        documents::write(
            &self.0.auth,
            "stop-watch",
            Some(data.timestamp.clone()),
            &data,
            documents::WriteOptions::default(),
        );
    }
}

struct RecordService<R: RecordDao>(R);
impl<R: RecordDao> RecordService<R> {
    fn push_record(&self, record: Record) {
        self.0.push_db(record);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Record {
    record: Duration,
    timestamp: DateTime<Local>,
}

impl Record {
    pub fn create(datetime: Duration) -> Self {
        Self {
            record: datetime,
            timestamp: Local::now(),
        }
    }

    // fn transform_for_db(&self) -> RecordFirebase {
    //     let format_string = "%Y-%m-%d %H:%M:%S";
    //     RecordFirebase {
    //         record: self.record.format(format_string).to_string(),
    //         timestamp: self.record.format(format_string).to_string(),
    //     }
    // }
}

pub fn update<R: RecordDao>(dao: R, time: Duration) {
    let data = Record::create(time);

    let record_service = RecordService(dao);
    record_service.push_record(data);
}

fn fetch_list() -> Vec<Record> {
    let cred = Credentials::from_file("firebase-key.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let documents: List<Record, ServiceSession> = documents::list(&auth, "stop-watch");

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
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read, Write};
    const TEST_FILE: &str = "record.json";

    struct RecordTestDao;
    impl RecordDao for RecordTestDao {
        fn push_db(&self, record: Record) {
            let mut writer = BufWriter::new(File::create(TEST_FILE).unwrap());
            let data = serde_json::to_string(&record).unwrap();
            writer.write(data.as_bytes());
        }
    }
    struct TestSuite;
    trait DoTest {
        fn init(&self) {
            std::fs::remove_file(TEST_FILE);
        }
        fn done(&self) {
            std::fs::remove_file(TEST_FILE);
        }
        fn run_test(&self) {
            self.init();
            self.test_logic();
            self.done();
        }
        fn test_logic(&self);
    }

    #[test]
    fn test_update() {
        impl DoTest for TestSuite {
            fn test_logic(&self) {
                let target = Duration::new(60, 0);

                let dao = RecordTestDao;
                // let dao = RecordFirestoreDao(FirestoreConnection::new());
                update(dao, target);

                let reader = BufReader::new(File::open("record.json").unwrap());
                let record: Record = serde_json::from_reader(reader).unwrap();

                assert_eq!(target, record.record);
            }
        }
        let test = TestSuite;
        test.run_test()
    }
}
