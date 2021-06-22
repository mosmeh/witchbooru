mod request;
mod resource;

use witchbooru::Classifier;

use netlify_lambda_http::{
    http::{self, HeaderValue},
    lambda, Request, Response,
};
use serde_json::{json, Value};
use tokio::sync::OnceCell;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

static CLASSIFIER: OnceCell<Classifier> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let classifier = async {
        CLASSIFIER
            .get_or_try_init(resource::create_classifier)
            .await
            .map_err(Into::into)
    };

    let runtime = lambda::run(netlify_lambda_http::handler(|req, _| async {
        let mut response = match handler(req).await {
            Ok(mut value) => {
                let obj = value
                    .as_object_mut()
                    .expect("Tried to return non-object JSON value");
                obj.insert("ok".to_string(), Value::Bool(true));

                Response::new(value.to_string())
            }
            Err(err) => {
                let value = json!({
                    "ok": false,
                    "error": err.to_string()
                });
                Response::builder()
                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(value.to_string())?
            }
        };

        let headers = response.headers_mut();
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );

        Ok(response)
    }));

    let _ = futures::try_join!(classifier, runtime)?;
    Ok(())
}

async fn handler(req: Request) -> anyhow::Result<Value> {
    let classifier = CLASSIFIER.get_or_try_init(resource::create_classifier);
    let img = request::extract_image(&req);

    let (classifier, img) = futures::try_join!(classifier, img)?;
    let prediction = tokio::task::spawn_blocking(move || classifier.predict(img)).await??;
    log::info!("Finished inference");

    let general: Vec<_> = prediction.general().iter().collect();
    let character: Vec<_> = prediction.character().iter().collect();

    Ok(json!({
        "general": general,
        "character": character,
    }))
}
