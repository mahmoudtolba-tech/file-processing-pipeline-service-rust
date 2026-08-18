use actix_web::web;

mod file_handler;

/// Register all routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(file_handler::init_routes),
    );
}