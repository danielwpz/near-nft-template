#!/usr/bin/env node
const yargsInteractive = require("yargs-interactive");
const { init } = require("./helper");
const fs = require("fs");

const options = {
  network: {
    type: "input",
    describe: "Network ID. mainnet or testnet",
    default: "testnet"
  },
  contractId: {
    type: "input",
    describe: "NFT Contract Address"
  },
  wasm: {
    type: "input",
    describe: "WASM file path",
    default: "res/nft.wasm"
  }
};

yargsInteractive()
  .usage("$0 <command> [args]")
  .interactive(options)
  .then(async result => {
    const { contractId, wasm, network } = result;
    const near = await init(network);
    const account = await near.account(contractId);

    await account.deployContract(fs.readFileSync(wasm));
    console.log(`Deployed NFT contract to ${contractId}`);
  });
