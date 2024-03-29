use futures::executor;

struct User {}

struct UserId(u32);

struct Db {}

impl Db {
    async fn find_by_user_id(&self, user_id: UserId) -> Option<User> {}
}

async fn find_by_user_id(db: Db, user_id: UserId) -> Option<User> {
    db.find_by_user_id(user_id).await
}

fn run_sync() {
    executor::block_on(find_by_user_id(Db {}, UserId(1)));
}
