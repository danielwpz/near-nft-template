#!/usr/bin/env node
const yargsInteractive = require("yargs-interactive");
const { init } = require("./helper");
const { Gas, NEAR } = require("near-units");

const options = {
  interactive: {
    default: true
  },
  network: {
    type: "input",
    describe: "Network ID. mainnet or testnet",
    default: "testnet"
  },
  contractId: {
    type: "input",
    describe: "NFT Contract Address"
  },
  owner: {
    type: "input",
    describe: "Token owner ID",
  },
  title: {
    type: "input",
    describe: "Token title",
  },
  description: {
    type: "input",
    describe: "Token description",
  },
  media: {
    type: "input",
    describe: "Token media URL",
  },
  copies: {
    type: "input",
    describe: "How many copies to mint",
  },
};

yargsInteractive()
  .usage("$0 <command> [args]")
  .interactive(options)
  .then(async result => {
    const { contractId, network, owner, title, description, media, copies } = result;
    const near = await init(network);
    const account = await near.account(contractId);

    const args = {
      token_owner_id: owner,
      token_metadata: {
        title,
        description,
        media,
      },
    };

    for (let i = 0; i < copies; i++) {
      await account.functionCall({
        contractId,
        methodName: 'nft_mint',
        args,
        gas: Gas.parse("100 Tgas"),
        attachedDeposit: NEAR.parse('0.01')
      });
      console.log('Minted 1 NFT');
    }

    console.log('done');
  });
