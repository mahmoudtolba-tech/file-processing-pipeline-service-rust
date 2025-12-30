use super::models;
use super::repositories;
use super::schema;

pub async fn process_file(file_path: String) -> Result<(), std::io::Error> {
    let file_repository = repositories::FileRepository::new();
    let new_file = file_repository.create_file(file_path)?;
    Ok(())
}