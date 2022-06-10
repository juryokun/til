use firestore_db_and_auth::{documents, documents::List, dto, Credentials, ServiceSession};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct Data {
    id: u8,
}

struct Firestore {
    auth: ServiceSession,
}

trait Repository<T, U> {
    fn create(&mut self, new_params: T);
}

impl Repository<u8, Data> for Firestore {
    fn create(&mut self, id: u8) {
        let d = Data { id: id };
        documents::write(
            &self.auth,
            "for-test",
            Some("AABBCCDD"),
            &d,
            documents::WriteOptions::default(),
        );
    }
}

struct Service<U: Repository<u8, Data>> {
    repository: U,
}

impl<U: Repository<u8, Data>> Service<U> {
    pub fn create_data(&mut self, id: u8) {
        self.repository.create(id)
    }
}

fn main() {
    let cred = Credentials::from_file("firebase-key.json").unwrap();
    let auth = ServiceSession::new(cred).unwrap();
    let db: Firestore = Firestore { auth: auth };

    let mut service = Service::<Firestore> { repository: db };
    service.create_data(8);
}

mod test {
    use super::*;

    struct TestDb {
        data: Vec<Data>,
    }

    impl Repository<u8, Data> for TestDb {
        fn create(&mut self, id: u8) {
            let d = Data { id: id };
            self.data.push(d);
        }
    }

    #[test]
    fn test_create() {
        let db: TestDb = TestDb { data: Vec::new() };
        let mut service = Service::<TestDb> { repository: db };
        service.create_data(8);

        println!("{:?}", service.repository.data);
    }
}
