use validator::validate_email;
#[derive(Debug,Clone)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("'{}'不是合法的邮件地址", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    // 获得共享引用，只有只读权限
    fn as_ref(&self) -> &str {
        &self.0
    }
}

mod tests{
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use crate::domain::SubscriberEmail;

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email=SafeEmail().fake();
        claim::assert_ok!(SubscriberEmail::parse(email));
    }
}
