use repositories::file_repository::FileRepository;
use models::FileModel;
use models::FileInput;

pub struct FileService {
    repository: FileRepository,
}

impl FileService {
    pub fn new(repository: FileRepository) -> Self {
        FileService { repository }
    }

    pub async fn create_file(&self, req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
        let file: FileInput = serde_json::from_str(&hyper::body::to_string(req.into_body()).await?).unwrap();
        let result = self.repository.create_file(file).await;
        match result {
            Ok(file) => Ok(hyper::Response::new(hyper::Body::from(serde_json::to_string(&file).unwrap()))),
            Err(err) => Ok(hyper::Response::new(hyper::Body::from(format!("Error: {}", err)))),
        }
    }

    pub async fn get_all_files(&self, _req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
        let files = self.repository.get_all_files().await;
        match files {
            Ok(files) => Ok(hyper::Response::new(hyper::Body::from(serde_json::to_string(&files).unwrap()))),
            Err(err) => Ok(hyper::Response::new(hyper::Body::from(format!("Error: {}", err)))),
        }
    }
}