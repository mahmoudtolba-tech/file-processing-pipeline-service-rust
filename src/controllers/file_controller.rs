use super::*;
use models::{FileModel, FileInput};
use repositories::file_repository::FileRepository;
use services::file_service::FileService;

pub struct FileController {
    service: FileService,
}

impl FileController {
    pub fn new(db_url: String) -> Self {
        let repository = FileRepository::new(db_url);
        let service = FileService::new(repository);
        FileController { service }
    }

    async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::POST, "/file") => {
                self.service.create_file(req).await
            }
            (&hyper::Method::GET, "/file") => {
                self.service.get_all_files(req).await
            }
            _ => {
                Ok(Response::new(Body::from("Not found")))
            }
        }
    }
}