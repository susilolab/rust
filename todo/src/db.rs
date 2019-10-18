use std::ops::Deref;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use crate::model::{NewTask, Task};

pub type MyPool = Pool<ConnectionManager<MysqlConnection>>;
type MyPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn init_pool(database_url: &str) -> Result<MyPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &MyPool) -> Result<MyPooledConnection, &'static str> {
    pool.get().map_err(|_| "Tidak dapat terhubung")
}

pub fn get_all_tasks(pool: &MyPool) -> Result<Vec<Task>, &'static str> {
    Task::all(get_conn(pool)?.deref()).map_err(|_| "Tidak dapat insert task")
}

pub fn create_task(todo: String, pool: &MyPool) -> Result<(), &'static str> {
    let new_task = NewTask { description: todo };
    Task::insert(new_task, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error insert task")
}

pub fn toggle_task(id: i32, pool: &MyPool) -> Result<(), &'static str> {
    Task::toggle_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error insert task")
}

pub fn delete_task(id: i32, pool: &MyPool) -> Result<(), &'static str> {
    Task::delete_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error delete task")
}
