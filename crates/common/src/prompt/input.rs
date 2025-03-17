use std::str::FromStr;

use cliclack::{Input, Validate};

pub struct Prompt {
    inner: Input,
    default: Option<String>,
}

impl Prompt {
    pub fn new(question: &str) -> Self {
        let inner = Input::new(question);
        Self {
            inner,
            default: None,
        }
    }

    pub fn allow_empty(mut self) -> Self {
        self.inner = self.inner.required(false);
        self
    }

    pub fn default(mut self, default: &str) -> Self {
        self.inner = self.inner.default_input(default);
        self.default = Some(default.into());
        self
    }

    pub fn validate_with<F>(mut self, f: F) -> Self
    where
        F: Validate<String> + 'static,
        F::Err: ToString,
    {
        self.inner = self.inner.validate(f);
        self
    }

    pub fn validate_interactively<F>(mut self, f: F) -> Self
    where
        F: Validate<String> + 'static,
        F::Err: ToString,
    {
        self.inner = self.inner.validate_interactively(f);
        self
    }

    pub fn ask<T>(mut self) -> T
    where
        T: FromStr,
    {
        self.inner.interact().unwrap()
    }

    pub fn default_or_ask<T>(self, use_default: bool) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        if use_default {
            return T::from_str(&self.default.unwrap_or_default())
                .expect("Cannot convert default value fromStr");
        }
        self.ask()
    }
}
