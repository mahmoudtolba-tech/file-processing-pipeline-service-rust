use super::models;
use super::schema;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub struct FileRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl FileRepository {
    pub fn new() -> FileRepository {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        FileRepository {
            pool: Pool::builder()
                .build(manager)
                .expect("Failed to create pool"),
        }
    }

    pub fn get_file(&self, id: i32) -> Option<models::File> {
        use schema::files::dsl::*;
        let conn = self.pool.get().unwrap();
        files.find(id).first::<models::File>(&conn).ok()
    }

    pub fn create_file(&self, file_path: String) -> Result<models::File, diesel::result::Error> {
        use schema::files::dsl::*;
        let conn = self.pool.get().unwrap();
        let new_file = models::File {
            id: 0,
            file_path: file_path.clone(),
        };
        diesel::insert_into(files)
            .values(&new_file)
            .get_result::<models::File>(&conn)
    }
}