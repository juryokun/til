use firestore_db_and_auth::{documents, documents::List, dto, Credentials, ServiceSession};
use serde::{Deserialize, Serialize};

// pub trait IsSvcA {
//     fn a(&self) -> String;
// }
pub trait FireBaseDao {
    fn prd_data_update(&self, data: Data);
}

// pub trait SvcA {}
pub struct FirebaseConnection {
    auth: ServiceSession,
}

// pub trait IsSvcB {
//     fn b(&self) -> String;
// }
pub trait DbService {
    fn data_update(&self, data: Data);
}

// pub trait SvcB: IsSvcA {}
pub trait Dao: FireBaseDao {}

// impl<T: SvcA> IsSvcA for T {
//     fn a(&self) -> String {
//         "svc-a".to_owned()
//     }
// }
impl FireBaseDao for FirebaseConnection {
    fn prd_data_update(&self, data: Data) {
        documents::write(
            &self.auth,
            "for-test",
            Some("AABBCCDD"),
            &data,
            documents::WriteOptions::default(),
        );
    }
}

// impl<T: SvcB> IsSvcB for T {
//     fn b(&self) -> String {
//         format!("{}{}", self.a(), "svc-b")
//     }
// }
impl<T: Dao> DbService for T {
    fn data_update(&self, data: Data) {
        self.prd_data_update(data);
    }
}

// pub fn use_b<B: IsSvcB>(b: B) -> String {
//     format!("[got] {}", b.b())
// }
pub fn update<B: DbService>(service: B, data: Data) {
    service.data_update(data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    id: String,
}

struct Hub{}
impl DbService for Hub
impl

fn main() {
    let service = DbService{};
}
