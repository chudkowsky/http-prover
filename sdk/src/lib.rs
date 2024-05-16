mod access_key;
mod load;
pub mod prove_sdk_builder;
pub mod prover_sdk;

pub mod errors;

pub use access_key::ProverAccessKey;
pub use load::{load_cairo0, load_cairo1};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::errors::ProverSdkErrors;
    use crate::load::{load_cairo0, load_cairo1};
    use crate::prover_sdk::ProverSDK;
    use crate::ProverAccessKey;
    use prover::server::start;
    use prover::Args;
    use tokio::task::JoinHandle;
    use url::Url;

    fn get_signing_key() -> ProverAccessKey {
        ProverAccessKey::from_hex_string(
            "f91350db1ca372b54376b519be8bf73a7bbbbefc4ffe169797bc3f5ea2dec740",
        )
        .unwrap()
    }

    async fn spawn_prover() -> (JoinHandle<()>, ProverAccessKey) {
        let key = ProverAccessKey::generate();
        let encoded_key = hex::encode(key.0.verifying_key().to_bytes());

        let args = Args {
            host: "0.0.0.0".to_string(),
            port: 3000,
            jwt_secret_key: "placeholder".to_string(),
            message_expiration_time: 60,
            session_expiration_time: 3600,
            private_key: "f91350db1ca372b54376b519be8bf73a7bbbbefc4ffe169797bc3f5ea2dec740".into(),
            authorized_keys: Some(vec![encoded_key]),
            authorized_keys_path: None,
        };

        let handle = tokio::spawn(async move {
            start(args).await.unwrap();
        });

        (handle, key)
    }

    #[tokio::test]
    async fn test_prover_cairo1_spawn_prover() -> Result<(), ProverSdkErrors> {
        let (_handle, key) = spawn_prover().await;

        let url_auth = Url::parse("http://localhost:3000/auth").unwrap();
        let url_prover = Url::parse("http://localhost:3000/prove/cairo1").unwrap();
        let sdk = ProverSDK::new(key, url_auth, url_prover).await?;

        let data = load_cairo1(PathBuf::from("../prover/resources/input_cairo1.json")).await?;
        let proof = sdk.prove(data).await;

        // If authentication fails, print out the error message
        assert!(proof.is_ok(), "Failed to prove with invalid url");
        // If authentication fails, print out the error message for debugging purposes
        if let Err(err) = proof {
            println!(" error: {}", err);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_prover_cairo0() -> Result<(), ProverSdkErrors> {
        let url_auth = Url::parse("http://localhost:3000/auth").unwrap(); // Provide an invalid URL for authentication
        let url_prover = Url::parse("http://localhost:3000/prove/cairo0").unwrap();

        // Act: Attempt to authenticate with the valid private key and invalid URL for authentication
        let sdk = ProverSDK::new(get_signing_key(), url_auth, url_prover).await?;

        let data = load_cairo0(PathBuf::from("../prover/resources/input_cairo0.json")).await?;
        let proof = sdk.prove(data).await;
        // If authentication fails, print out the error message
        assert!(proof.is_ok(), "Failed to prove with invalid url");
        // If authentication fails, print out the error message for debugging purposes
        if let Err(err) = proof {
            println!(" error: {}", err);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_prover_cairo1() -> Result<(), ProverSdkErrors> {
        let url_auth = Url::parse("http://localhost:3000/auth").unwrap(); // Provide an invalid URL for authentication
        let url_prover = Url::parse("http://localhost:3000/prove/cairo1").unwrap();

        // Act: Attempt to authenticate with the valid private key and invalid URL for authentication
        let sdk = ProverSDK::new(get_signing_key(), url_auth, url_prover).await?;

        let data = load_cairo1(PathBuf::from("../prover/resources/input_cairo1.json")).await?;
        let proof = sdk.prove(data).await;
        // If authentication fails, print out the error message
        assert!(proof.is_ok(), "Failed to prove with invalid url");
        // If authentication fails, print out the error message for debugging purposes
        if let Err(err) = proof {
            println!(" error: {}", err);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_valid_private_key_auth() -> Result<(), ProverSdkErrors> {
        // Arrange: Set up any necessary data or dependencies
        let url_auth = Url::parse("http://localhost:3000/auth").unwrap();
        let url_prover = Url::parse("http://localhost:3000/prove/cairo0").unwrap();

        // Act: Attempt to authenticate with the valid private key
        let result = ProverSDK::new(get_signing_key(), url_auth, url_prover).await;

        // Assert: Check that authentication succeeds
        assert!(
            result.is_ok(),
            "Expected authentication to succeed with valid private key"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_url_auth() -> Result<(), ProverSdkErrors> {
        // Arrange: Set up any necessary data or dependencies
        let url_auth = Url::parse("http://localhost:3000/notauth").unwrap(); // Provide an invalid URL for authentication
        let url_prover = Url::parse("http://localhost:3000/prove/cairo0").unwrap();

        // Act: Attempt to authenticate with the valid private key and invalid URL for authentication
        let result = ProverSDK::new(get_signing_key(), url_auth, url_prover).await;
        // Assert: Check that authentication fails due to invalid URL
        assert!(
            result.is_err(),
            "Expected authentication to fail with invalid URL for authentication"
        );
        // If authentication fails, print out the error message
        if let Err(err) = result {
            println!("Error message: {}", err);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_url_prover() -> Result<(), ProverSdkErrors> {
        // Arrange: Set up any necessary data or dependencies

        let url_auth = Url::parse("http://localhost:3000/auth").unwrap();
        let url_prover = Url::parse("http://localhost:3000/notprove/cairo1").unwrap();

        // Act: Attempt to authenticate with the valid private key and invalid URL for authentication
        let sdk = ProverSDK::new(get_signing_key(), url_auth, url_prover).await?;

        let data = load_cairo1(PathBuf::from("../prover/resources/input_cairo1.json")).await?;

        let proof = sdk.prove(data).await;
        // If authentication fails, print out the error message
        assert!(proof.is_err(), "Failed to prove with invalid url");

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_url_without_base_prover() -> Result<(), ProverSdkErrors> {
        let url_auth = Url::parse("http://localhost:3000/notauth").unwrap();
        let url_prover = Url::parse("http://localhost:3000/prove/cairo0").unwrap(); // Provide an invalid URL for authentication

        // Act: Attempt to authenticate with the valid private key and invalid URL for authentication
        let sdk = ProverSDK::new(get_signing_key(), url_auth, url_prover).await;

        assert!(sdk.is_err(), "Failed to parse url without base to url");

        Ok(())
    }

    #[tokio::test]
    async fn test_register() -> Result<(), ProverSdkErrors> {
        let url_auth = Url::parse("http://localhost:3000/auth").unwrap();
        let url_prover = Url::parse("http://localhost:3000/prove/cairo0").unwrap();
        let url_register = Url::parse("http://localhost:3000/register").unwrap();

        let new_key = ProverAccessKey::generate();

        // Act: Attempt to authenticate with the valid private key
        let mut sdk = ProverSDK::new(get_signing_key(), url_auth, url_prover).await?;
        sdk.register(new_key.0.verifying_key(), url_register)
            .await?;
        // If authentication fails, print out the error message

        Ok(())
    }
}
