#!/usr/bin/env node
const yargsInteractive = require("yargs-interactive");
const { init } = require("./helper");
const { Gas } = require("near-units");

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
  name: {
    type: "input",
    describe: "Contract Name",
  },
  symbol: {
    type: "input",
    describe: "Contract Symbol",
  },
  creator: {
    type: "input",
    describe: "Creator account ID",
  },
  creatorRoyalty: {
    type: "input",
    describe: "Creator royalty percentage",
  },
};

yargsInteractive()
  .usage("$0 <command> [args]")
  .interactive(options)
  .then(async result => {
    const { contractId, network, name, symbol, creator, creatorRoyalty } = result;
    const near = await init(network);
    const account = await near.account(contractId);
    const spec = "nft-1.0.0";

    const args = {
      owner_id: contractId,
      metadata: {
        spec,
        name,
        symbol,
      },
    };

    if (creator && creatorRoyalty) {
      args.creator_id = creator;
      args.creator_royalty_bp = parseInt(creatorRoyalty * 100);
    }

    await account.functionCall({
      contractId,
      methodName: 'new',
      args,
      gas: Gas.parse("100 Tgas")
    });

    console.log('done');
  });
