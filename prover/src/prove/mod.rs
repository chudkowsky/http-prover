use crate::server::AppState;
use axum::Json;
use axum::{routing::get, routing::post, Router};
pub mod errors;
pub mod models;
pub mod prove_input;
use crate::auth::jwt::Claims;
use crate::prove::errors::ProveError;
use podman::runner::Runner;
use self::prove_input::ProveInput;
pub fn auth(app_state: &AppState) -> Router {
    Router::new()
        .route("/auth", get(crate::auth::validation::generate_nonce))
        .route("/auth", post(crate::auth::validation::validate_signature))
        .with_state(app_state.clone())
}

pub async fn root(_claims: Claims, Json(program_input): Json<ProveInput>) -> Result<String, ProveError> {
    let runner = podman::runner::PodmanRunner::new("docker.io/chudas/stone5-poseidon3:latest");
    let v = serde_json::to_string(&program_input)?;
    let result: String = runner.run(&v).await?;
    let proof: serde_json::Value = serde_json::from_str(&result)?;
    let final_result = serde_json::to_string_pretty(&proof)?;
    Ok(final_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::Claims;
    use errors::ProveError;
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;
    #[tokio::test]
    async fn test_root_with_input_json() {
        // Read input data from resources/input.json
        let input_json = read_json_file("resources/input.json")
            .await
            .expect("Failed to read input JSON");

        // Call the root function with the input data and actual PodmanRunner
        let result = root(
            Claims {
                sub: "jwt_token".to_string(),
                exp: 3600,
            },
            Json(input_json)
        )
        .await;

        // Check if the result is as expected
        assert!(result.is_ok());
        // Add assertions based on the expected behavior of root function
    }

    async fn read_json_file(file_path: &str) -> Result<ProveInput, ProveError> {
        println!("{:?}", file_path);

        let mut file = File::open(file_path).await?;
        let mut json_string = String::new();
        file.read_to_string(&mut json_string).await?;

        let json_value: ProveInput = serde_json::from_str(&json_string)?;

        Ok(json_value)
    }
}
