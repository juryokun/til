use super::*;
use chrono::{DateTime, Local};
use firestore_db_and_auth::{documents, documents::List, dto, Credentials, ServiceSession};
use serde::{Deserialize, Serialize};

pub struct Firestore {
    auth: ServiceSession,
}

impl Firestore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let cred = Credentials::from_file("firebase-key.json")?;
        let auth = ServiceSession::new(cred)?;
        Ok(Self { auth: auth })
    }
}

pub trait Repository<T> {
    fn push_db(&mut self, record: T) -> Result<(), Box<dyn std::error::Error>>;
}

impl Repository<Record> for Firestore {
    fn push_db(&mut self, record: Record) -> Result<(), Box<dyn std::error::Error>> {
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
            &self.auth,
            "stop-watch",
            Some(data.timestamp.clone()),
            &data,
            documents::WriteOptions::default(),
        )?;
        Ok(())
    }
}

pub struct Service<R: Repository<Record>> {
    repository: R,
}

impl<R: Repository<Record>> Service<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository: repository,
        }
    }
    pub fn push_record(&mut self, record: Record) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.push_db(record)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
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
}

fn fetch_list() -> Vec<Record> {
    let cred = Credentials::from_file("firebase-key.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();

    let documents: List<Record, ServiceSession> = documents::list(&auth, "stop-watch");

    let mut rel_vec = vec![];
    for doc in documents {
        let (data, _document) = doc.unwrap();
        rel_vec.push(data);
    }

    rel_vec
}

mod test {
    use super::*;
    struct MockDb {
        data: Vec<Record>,
    }
    impl Repository<Record> for MockDb {
        fn push_db(&mut self, record: Record) -> Result<(), Box<dyn std::error::Error>> {
            self.data.push(record);
            Ok(())
        }
    }

    #[test]
    fn test_update() {
        let record = Record {
            record: Duration::new(60, 0),
            timestamp: Local::now(),
        };

        let connection: MockDb = MockDb { data: Vec::new() };
        let mut service = Service::<MockDb>::new(connection);
        let result = service.push_record(record);

        match result {
            Ok(val) => assert_eq!(val, ()),
            Err(_) => assert!(false),
        }
        assert_eq!(service.repository.data[0], record.clone());
    }
}
