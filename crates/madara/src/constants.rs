// Madara constants
pub const MADARA_DOCKER_IMAGE: &str = "madara";
pub const MADARA_COMPOSE_FILE: &str = "compose.yaml";

// CLI messages
// pub(super) const MSG_STARTING_CONTAINERS_SPINNER: &str = "Starting containers...";
pub(super) const MSG_BUILDING_IMAGE_SPINNER: &str = "Building image...";

// Extra
pub const MADARA_REPO_PATH: &str = "deps/madara";
pub const MADARA_RUNNER_SCRIPT: &str = "madara-runner.sh";
pub const MADARA_RPC_API_KEY_FILE: &str = ".secrets/rpc_api.secret";

pub const DEPS_REPO_PATH: &str = "deps";

pub const DOCKERHUB_ORGANIZATION: &str = "gustavomoonsong/";

pub const DEFAULT_LOCAL_CONFIG_FILE: &str = "crates/madara/src/config/local.toml";

pub const DEFAULT_TMP_DATA_DIRECTORY: &str = "deps/data";

// Images version
pub const REMOTE_HELPER_IMAGE: &str = "1.0.0";
pub const REMOTE_BOOTSTRAPPER_IMAGE: &str = "1.1.0";
pub const REMOTE_MADARA_IMAGE: &str = "5bbb3a";
pub const REMOTE_PATHFINDER_IMAGE: &str = "1.0.0";
pub const REMOTE_ORCHESTRATOR_IMAGE: &str = "1.0.0";
