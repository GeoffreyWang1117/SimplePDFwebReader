use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpRequest, Result};
use actix_web::http::header::{ContentDisposition, DispositionType};
use std::path::PathBuf;

async fn serve_pdf(req: HttpRequest) -> Result<NamedFile> {
    let filename: String = req.match_info().query("filename").parse().unwrap();
    let path: PathBuf = format!("./pdfs/{}", filename).parse().unwrap();

    let file = NamedFile::open(path)?
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        });
    
    Ok(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/pdf/{filename}", web::get().to(serve_pdf))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}