use actix_web::web;
use crate::{auth, media, social, streaming};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::configure)
            .configure(media::configure)
            .configure(social::configure)
            .configure(streaming::configure)
    );
}
