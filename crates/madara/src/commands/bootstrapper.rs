use madara_cli_common::{docker, logger};
use xshell::Shell;

use crate::config::{
    bootstrapper::BootstrapperConfiguration,
    global_config::{Config, EthWallet},
};

const BOOTSTRAPPER_REPO_PATH: &str = "deps/bootstrapper";
const BOOTSTRAPPER_DOCKER_IMAGE: &str = "bootstrapper";
const BOOTSTRAPPER_CONFIG_FILE: &str = "deps/bootstrapper/devnet.json";

pub fn build_image(shell: &Shell) -> anyhow::Result<()> {
    docker::build_image(
        shell,
        BOOTSTRAPPER_REPO_PATH.to_string(),
        BOOTSTRAPPER_DOCKER_IMAGE.to_string(),
    )?;

    Ok(())
}

pub fn process_params(global_config: &Config) -> anyhow::Result<()> {
    let global_config = global_config.clone();

    let mut config = BootstrapperConfiguration::load(BOOTSTRAPPER_CONFIG_FILE);

    let eth_wallet = EthWallet::new(
        global_config.eth_wallet.eth_priv_key,
        global_config.eth_wallet.l1_multisig_address,
    );

    assert_eq!(
        eth_wallet.l1_deployer_address, global_config.eth_wallet.l1_deployer_address,
        "L1 deployer address is not derived from provided private key"
    );

    config.eth_rpc = global_config.l1_config.eth_rpc;
    config.eth_priv_key = eth_wallet.eth_priv_key;
    config.l1_deployer_address = eth_wallet.l1_deployer_address;
    config.l1_multisig_address = eth_wallet.l1_multisig_address;
    config.operator_address = eth_wallet.operator_address;
    config.app_chain_id = global_config.madara.app_chain_id;

    config.save(BOOTSTRAPPER_CONFIG_FILE);

    Ok(())
}
