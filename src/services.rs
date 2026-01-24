use crate::models::File;
use crate::repositories::FileRepository;

pub struct FileService {
    repository: FileRepository,
}

impl FileService {
    pub fn new(repository: FileRepository) -> Self {
        Self { repository }
    }

    pub async fn process_file(&self, file: File) -> Result<File, sqlx::Error> {
        self.repository.save_file(file).await
    }
}

#### repositories.rs