import "dotenv/config";

import {
  Address,
  Contract,
  Keypair,
  SorobanRpc,
  networks as deployerNet,
  nativeToScVal,
} from "../../packages/deployer/dist/index.js";
import { networks as poolNet } from "../../packages/pool/dist/index.js";
import { keccak256, toBuffer, ecsign } from "ethereumjs-utils";

import { readFileSync } from "fs";

export const server = new SorobanRpc.Server(
  process.env.PUBLIC_SOROBAN_RPC_URL,
  { allowHttp: true }
);
export const deployer_ct = new Contract(deployerNet.standalone.contractId);
export const pool = new Contract(poolNet.standalone.contractId);

export const poolWarm = keccak256(
  readFileSync("target/wasm32-unknown-unknown/release/pool.wasm")
);

export const submit_txn = async (txn: any, signer: Keypair) => {
  const simulate = await txn.simulate();
  const preparedTxn = await server.prepareTransaction(simulate.built);
  preparedTxn.sign(signer);
  try {
    let sendResponse = await server.sendTransaction(preparedTxn);
    console.log(`Sent transaction`);

    if (sendResponse.status === "PENDING") {
      let getResponse = await server.getTransaction(sendResponse.hash);
      // Poll `getTransaction` until the status is not "NOT_FOUND"
      while (getResponse.status === "NOT_FOUND") {
        console.log("Waiting for transaction confirmation...");
        // See if the transaction is complete
        getResponse = await server.getTransaction(sendResponse.hash);
        // Wait one second
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      if (getResponse.status === "SUCCESS") {
        // Make sure the transaction's resultMetaXDR is not empty
        if (!getResponse.resultMetaXdr) {
          throw "Empty resultMetaXDR in getTransaction response";
        }
        // Find the return value from the contract and return it
        let transactionMeta = getResponse.resultMetaXdr;
        let returnValue = transactionMeta.v3().sorobanMeta().returnValue();
        console.log(`Transaction confirmed`);
        return returnValue;
      } else {
        throw `Transaction failed: ${getResponse.resultXdr}`;
      }
    } else {
      throw sendResponse.errorResult;
    }
  } catch (err) {
    // Catch and report any errors we've thrown
    console.log("Sending transaction failed");
    console.log(JSON.stringify(err));
  }
};
