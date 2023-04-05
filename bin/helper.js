const nearAPI = require('near-api-js');

/**
 * Init NEAR instance
 * @param {'testnet' | 'mainnet'} network 
 * @returns {Promise<nearAPI.Near>}
 */
async function init(network) {
  const { keyStores } = nearAPI;
  const homedir = require("os").homedir();
  const CREDENTIALS_DIR = ".near-credentials";
  const credentialsPath = require("path").join(homedir, CREDENTIALS_DIR);
  const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

  let config;
  if (network === 'mainnet') {
    config = {
      networkId: "mainnet",
      nodeUrl: process.env.NEAR_CLI_MAINNET_RPC_SERVER_URL || "https://rpc.mainnet.near.org",
      walletUrl: "https://wallet.mainnet.near.org",
      helperUrl: "https://helper.mainnet.near.org",
      explorerUrl: "https://explorer.mainnet.near.org",
    };
  } else if (network === 'testnet') {
    config = {
      networkId: "testnet",
      nodeUrl: process.env.NEAR_CLI_TESTNET_RPC_SERVER_URL || "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
      explorerUrl: "https://explorer.testnet.near.org",
    }
  } else {
    throw new Error('bad network name');
  }

  config.keyStore = keyStore;
  return nearAPI.connect(config);
}

module.exports = {
  init,
}
