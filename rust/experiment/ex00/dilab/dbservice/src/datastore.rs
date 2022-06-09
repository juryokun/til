pub trait FireBaseDao {
    fn a(&self) -> String;
}

pub struct FireBase {
    auth: String
}

pub trait DbService {
    fn data_update(&self) -> String;
}

pub trait Dao: FireBaseDao {}

impl<T: FireBase> FireBaseDao for T {
    fn prd_data_update(&self) -> String {
        self.auth
    }
}

impl<T: Dao> DbService for T {
    fn data_update(&self) -> String {
        format!("{}{}", self.prd_data_update(), "svc-b")
    }
}

pub fn update<B: DbService>(service: B) -> String {
    format!("[got] {}", service.data_update())
}


fn logic() {
    let dbconection = Firebase { auth: "connection".to_string()}
}
