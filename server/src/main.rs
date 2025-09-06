use database::connection::init_db_pool;

mod database;

fn main() {
    let _ = init_db_pool();
}
