use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the entire configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Compose {
    /// The name of the runner.
    pub name: String,

    /// A map of service names to their configurations.
    pub services: HashMap<String, Service>,

    /// A map of secret names to their configurations.
    pub secrets: HashMap<String, Secret>,
}

/// Represents a single service configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    /// The Docker image to use for the service.
    pub image: String,

    /// Optional container name.
    #[serde(rename = "container_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    /// Optional CPU allocation (as a string to preserve formatting like "4.0").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,

    /// Optional memory limit (e.g., "16gb").
    #[serde(rename = "mem_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_limit: Option<String>,

    /// Optional list of port mappings (e.g., "9944:9944").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,

    /// Optional list of labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,

    /// Optional list of environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,

    /// Optional list of secrets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,

    /// Optional list of volume bindings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,

    /// Optional entrypoint command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,

    /// Optional healthcheck configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Healthcheck>,

    /// Optional restart policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,

    /// Optional TTY allocation (useful for services like `autoheal`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
}

/// Represents the healthcheck configuration for a service.
#[derive(Debug, Serialize, Deserialize)]
pub struct Healthcheck {
    /// The command to run for the healthcheck.
    pub test: Vec<String>,

    /// The interval between healthchecks (e.g., "10s").
    pub interval: String,

    /// The timeout for each healthcheck (e.g., "5s").
    pub timeout: String,

    /// The number of retries before marking the service as unhealthy.
    pub retries: u32,

    /// The startup period before starting healthchecks (e.g., "10s").
    pub start_period: String,
}

/// Represents a single secret configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    /// The file path to the secret.
    pub file: String,
}
