use sanitization::SecretString;

#[derive(Debug)]
pub struct RuntimeSecret {
    inner: SecretString,
}

impl RuntimeSecret {
    pub fn from_string(value: String) -> Self {
        Self {
            inner: SecretString::from_string(value),
        }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let _ = self.inner.try_with_secret(|secret| {
            length = secret.len();
            Ok::<(), core::convert::Infallible>(())
        });
        length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeSecret;

    #[test]
    fn stores_secret_without_exposing_value_in_debug() {
        let secret = RuntimeSecret::from_string("deployment-token".to_owned());
        assert_eq!(secret.len(), 16);
        assert!(!format!("{secret:?}").contains("deployment-token"));
    }
}
