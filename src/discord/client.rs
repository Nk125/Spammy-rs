use super::error::{Error, ErrorCategory};
use reqwest::Client as HttpClient;
use reqwest::Response as ClientResponse;
use serde_json as json;

const DISCORD_BASE_API: &str = "https://discord.com/api/v10";

pub struct Client {
    client: HttpClient,
    app_id: String,
}

impl Client {
    fn from_base_api(endpoint: &str) -> String {
        format!("{}/{}", DISCORD_BASE_API, endpoint)
    }

    fn load_json(data: &str) -> Result<json::Value, Error> {
        json::from_str(data).map_err(|e| Error::new(ErrorCategory::Conversion, Some(e.to_string())))
    }

    fn transform_result(
        res: Result<ClientResponse, reqwest::Error>,
    ) -> Result<ClientResponse, Error> {
        res.map_err(|e| Error::new(ErrorCategory::Transport, Some(e.to_string())))
    }

    async fn create_put_request(
        client: &HttpClient,
        endpoint: &str,
        body: String,
    ) -> Result<ClientResponse, Error> {
        Self::transform_result(
            client
                .put(Self::from_base_api(endpoint))
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await,
        )
    }

    async fn create_get_request(
        client: &HttpClient,
        endpoint: &str,
    ) -> Result<ClientResponse, Error> {
        Self::transform_result(client.get(Self::from_base_api(endpoint)).send().await)
    }

    async fn default_json_endpoint(
        client: &HttpClient,
        endpoint: &str,
        body: Option<json::Value>,
    ) -> Result<json::Value, Error> {
        let res = match body {
            Some(body) => Self::create_put_request(client, endpoint, body.to_string()).await,
            None => Self::create_get_request(client, endpoint).await,
        }?;

        if res.status().as_u16() == 401 {
            return Err(Error::new(ErrorCategory::Auth, None));
        }

        if res.headers().get("Content-Type").ok_or(Error::new(
            ErrorCategory::Transport,
            Some("response without content-type".into()),
        ))? != "application/json"
        {
            return Err(Error::new(
                ErrorCategory::ServerSide,
                Some("Not json response".into()),
            ));
        }

        if !res.status().is_success() {
            return Err(Error::new(
                ErrorCategory::Transport,
                Some(res.status().to_string()),
            ));
        }

        let body = res.text().await.map_err(|e| {
            log::error!("Error while getting request body: {:?}", e);
            Error::new(
                ErrorCategory::Transport,
                Some("fail at getting request body".into()),
            )
        })?;

        Self::load_json(&body).inspect_err(|e| {
            log::error!("Error while deserializing body: {:?}", e);

            if e.category == ErrorCategory::Conversion {
                log::error!("Body: {:?}", body);
            }
        })
    }

    pub async fn from_bot_token(token: &str) -> Result<Self, Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("Bot {}", token).parse().map_err(|e| {
                log::error!("Failed to create default header for builder: {}", e);

                Error::new(ErrorCategory::Lib, Some("Invalid header value".into()))
            })?,
        );

        let client = HttpClient::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| {
                log::error!("Failed to create client from builder: {}", e);
                Error::new(ErrorCategory::Lib, Some("Failed to create client".into()))
            })?;

        let json_body = Self::default_json_endpoint(&client, "applications/@me", None).await?;

        let app_id: String = json_body["id"]
            .as_str()
            .ok_or(Error::new(
                ErrorCategory::Conversion,
                Some("deserialize application id".into()),
            ))?
            .into();

        Ok(Self { app_id, client })
    }

    pub async fn get_app_info(&self) -> Result<json::Value, Error> {
        Self::default_json_endpoint(&self.client, &format!("applications/{}", self.app_id), None)
            .await
    }

    pub async fn submit_command(
        &self,
        name: &str,
        description: &str,
    ) -> Result<json::Value, Error> {
        // See schema: https://discord.com/developers/docs/interactions/application-commands#application-command-object
        Self::default_json_endpoint(
            &self.client,
            &format!("applications/{}/commands", self.app_id),
            Some(json::json!([{
                "name": name,
                "description": description,
                // Only allowed for user-installable apps
                "integration_types": [1],
                // Allowed in every context
                "contexts": [0, 1, 2],
                // Invoked command by typing /
                "type": 1
            }])),
        )
        .await
    }
}
