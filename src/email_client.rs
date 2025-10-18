use crate::configuration::EmailSettings;
use crate::domain::SubscriberEmail;
use secrecy::ExposeSecret;

#[derive(Clone)]
pub struct EmailClient {
    http_client: reqwest::Client,
    email_settings: EmailSettings,
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: SendEmailEntityRequest,
    to: SendEmailEntityRequest,
    subject: String,
    text: String,
    html: String,
}
#[derive(serde::Serialize)]
struct SendEmailEntityRequest {
    email: String,
    name: String,
}

impl EmailClient {
    pub fn new(email_settings: EmailSettings) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            email_settings,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.email_settings.base_url);
        let bearer_token = format!(
            "Bearer {}",
            self.email_settings.bearer_token.expose_secret()
        );
        let request_body = SendEmailRequest {
            from: SendEmailEntityRequest {
                email: self.email_settings.from_email.as_str().to_owned(),
                name: self.email_settings.from_email_name.as_str().to_owned(),
            },
            to: SendEmailEntityRequest {
                email: recipient.as_ref().to_owned(),
                name: recipient.as_ref().to_owned(),
            },
            subject: subject.to_owned(),
            text: text_content.to_owned(),
            html: html_content.to_owned(),
        };
        self.http_client
            .post(url.as_str())
            .header("Authorization", bearer_token)
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::get_configuration;
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use wiremock::matchers::header_exists;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let configuration = get_configuration().expect("配置失败");
        let mut email_settings = configuration.email;
        email_settings.base_url = mock_server.uri();
        let email_client = EmailClient::new(email_settings);
        Mock::given(header_exists("Authorization"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let subscriber_email =
            SubscriberEmail::parse("zrw1404644784@gmail.com".to_string()).unwrap();
        let _ = email_client
            .send_email(&subscriber_email, "标题测试", "123", "123")
            .await;
        mock_server.verify().await;
    }
}
