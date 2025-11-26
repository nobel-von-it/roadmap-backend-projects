pub mod user {
    pub mod get {
        use axum::{extract::State, http::StatusCode, response::IntoResponse};

        use crate::{
            html::HtmlManager,
            models::{Article, Articles},
            utils::parse_idx,
        };

        pub async fn articles(State(html_manager): State<HtmlManager>) -> impl IntoResponse {
            let articles = match Articles::from_db() {
                Ok(articles) => articles,
                Err(e) => {
                    eprintln!("articles not found in db: {}", e);
                    return (StatusCode::NOT_FOUND, "Not found").into_response();
                }
            };
            let html = match html_manager.home_html(&articles) {
                Ok(html) => html,
                Err(e) => {
                    eprintln!("articles format to html failed: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                        .into_response();
                }
            };
            html.into_response()
        }

        pub async fn article(
            url: axum::http::Uri,
            State(html_manager): State<HtmlManager>,
        ) -> impl IntoResponse {
            let idx = match parse_idx(url.path()) {
                Ok(idx) => idx,
                Err(e) => {
                    eprintln!("parsing error: {}", e);
                    return (StatusCode::NOT_FOUND, "Not found").into_response();
                }
            };
            let article = match Article::from_db(idx) {
                Ok(article) => article,
                Err(e) => {
                    eprintln!("article not found in db: {}", e);
                    return (StatusCode::NOT_FOUND, "Not found").into_response();
                }
            };
            let html = match html_manager.article_html(&article) {
                Ok(html) => html,
                Err(e) => {
                    eprintln!("article format to html failed: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                        .into_response();
                }
            };
            html.into_response()
        }
    }
}

pub mod admin {
    pub mod post {
        use axum::{Form, http::StatusCode, response::IntoResponse};
        use axum_auth::AuthBasic;

        use crate::{
            models::{Article, FormArticle},
            utils,
        };

        pub async fn new_article(
            AuthBasic((user, _)): AuthBasic,
            Form(new_article): Form<FormArticle>,
        ) -> impl IntoResponse {
            if user != "admin" {
                return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
            }
            if !Article::insert(new_article).is_ok() {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                    .into_response();
            }
            (StatusCode::SEE_OTHER, [("Location", "/admin")]).into_response()
        }

        pub async fn edit_article(
            url: axum::http::Uri,
            AuthBasic((user, _)): AuthBasic,
            Form(updated_article): Form<FormArticle>,
        ) -> impl IntoResponse {
            if user != "admin" {
                return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
            }
            let idx = match utils::parse_idx(url.path()) {
                Ok(idx) => idx,
                Err(e) => {
                    eprintln!("parsing error: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error: parsing",
                    )
                        .into_response();
                }
            };
            let article = Article::from((idx, updated_article));
            article.update().unwrap();

            (StatusCode::SEE_OTHER, [("Location", "/admin")]).into_response()
        }

        pub async fn delete_article(
            url: axum::http::Uri,
            AuthBasic((user, _)): AuthBasic,
        ) -> impl IntoResponse {
            if user != "admin" {
                return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
            }
            let idx = match utils::parse_idx(url.path()) {
                Ok(idx) => idx,
                Err(e) => {
                    eprintln!("parsing error: {}", e);
                    return (StatusCode::NOT_FOUND, "Not found").into_response();
                }
            };
            let article = match Article::from_db(idx) {
                Ok(article) => article,
                Err(e) => {
                    eprintln!("article not found in db: {}", e);
                    return (StatusCode::NOT_FOUND, "Not found").into_response();
                }
            };
            article.delete().unwrap();
            (StatusCode::SEE_OTHER, [("Location", "/admin")]).into_response()
        }
    }
    pub mod get {
        use axum::{
            body::Body,
            extract::{Request, State},
            http::StatusCode,
            response::IntoResponse,
        };
        use axum_auth::AuthBasic;

        use crate::{
            html::HtmlManager,
            models::{Article, Articles},
            utils,
        };

        pub async fn new_article(
            AuthBasic((user, _)): AuthBasic,
            State(html_manager): State<HtmlManager>,
        ) -> impl IntoResponse {
            if user != "admin" {
                return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
            }
            let html = match html_manager.new_article_html() {
                Ok(html) => html,
                Err(e) => {
                    eprintln!("article format to html failed: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                        .into_response();
                }
            };
            html.into_response()
        }

        pub async fn edit_article(
            url: axum::http::Uri,
            AuthBasic((user, _)): AuthBasic,
            State(html_manager): State<HtmlManager>,
        ) -> impl IntoResponse {
            if user != "admin" {
                return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
            }
            let idx = match utils::parse_idx(url.path()) {
                Ok(idx) => idx,
                Err(e) => {
                    eprintln!("parsing error: {}", e);
                    return (StatusCode::NOT_FOUND, "Not found").into_response();
                }
            };
            let article = match Article::from_db(idx) {
                Ok(article) => article,
                Err(e) => {
                    eprintln!("article not found in db: {}", e);
                    return (StatusCode::NOT_FOUND, "Not found").into_response();
                }
            };
            let html = match html_manager.edit_article_html(&article) {
                Ok(html) => html,
                Err(e) => {
                    eprintln!("article format to html failed: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                        .into_response();
                }
            };
            html.into_response()
        }
        pub async fn admin_panel(
            State(html_manager): State<HtmlManager>,
            req: Request<Body>,
        ) -> impl IntoResponse {
            if let Some((user, pass)) = utils::parse_basic_auth(req.headers()) {
                if user == "admin" && pass == "1234" {
                    let articles = match Articles::from_db() {
                        Ok(articles) => articles,
                        Err(e) => {
                            eprintln!("articles not found in db: {}", e);
                            return (StatusCode::NOT_FOUND, "Not found").into_response();
                        }
                    };
                    let html = match html_manager.admin_html(&articles) {
                        Ok(html) => html,
                        Err(e) => {
                            eprintln!("articles format to html failed: {}", e);
                            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                                .into_response();
                        }
                    };
                    return html.into_response();
                }
            }

            (
                StatusCode::UNAUTHORIZED,
                [(
                    axum::http::header::WWW_AUTHENTICATE,
                    r#"Basic realm="MyRealm", charset="UTF-8""#,
                )],
                "Unauthorized",
            )
                .into_response()
        }
    }
}
