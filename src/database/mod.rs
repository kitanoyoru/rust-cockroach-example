use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PoolError},
    Connection,
};

use crate::config::{Config, CONFIG};

#[serde(untagged)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum DatabaseConnection {
    Cockroach,
    Postgres,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;

pub type CockroachPool = Pool<PgConnection>;
pub type PostgresPool = Pool<PgConnection>;

#[cfg(feature = "cockroach")]
pub type PoolType = CockroachPool;

#[cfg(feature = "postgres")]
pub type PoolType = PostgresPool;

#[derive(Clone)]
pub enum InferPool {
    Cockroach(CockroachPool),
    Postgres(PostgresPool),
}

impl InferPool {
    pub fn init_pool(config: Config) -> Result<Self, r2d2::Error> {
        match config.database {
            DatabaseConnection::Cockroach => {
                init_pool::<PgConnection>(config).map(InferPool::Cockroach)
            }
            DatabaseConnection::Postgres => {
                init_pool::<PgConnection>(config).map(InferPool::Postgres)
            }
        }
    }
}

pub fn init_pool<T>(config: Config) -> Result<Pool<T>, PoolError>
where
    T: Connection + 'static,
{
    let manager = ConnectionManager::<T>::new(config.database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = InferPool::init_pool(CONFIG.clone()).expect("Failed to create database pool");
    match pool {
        InferPool::Cockroach(cockroach_pool) => cfg.data(cockroach_pool),
        InferPool::Postgres(postgres_pool) => cfg.data(postgres_pool),
    };
}
