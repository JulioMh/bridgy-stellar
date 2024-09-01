import { get_client_deployer } from "../contracts/deployer.ts";

import { ADMIN, BE, submit_txn, USER } from "./utils/index.ts";
import { get_client_soroban_token_contract } from "../contracts/soroban_token_contract.ts";
import * as PoolClient from "../packages/pool/dist/index.js";
import { get_client_pool } from "../contracts/pool.ts";

describe("deployer", () => {
  const admin_pub_key = ADMIN.publicKey();
  const user_pub_key = USER.publicKey();

  describe("success", () => {
    it("init", async () => {
      const deployer = get_client_deployer(admin_pub_key);
      const txn_deployer = await deployer.init(
        {
          admin: { signer: admin_pub_key, fee_wallet: user_pub_key },
          be: BE,
        },
        { simulate: false }
      );
      await submit_txn(txn_deployer, ADMIN);
      const token = get_client_soroban_token_contract(admin_pub_key);
      const txn_token = await token.initialize(
        {
          admin: admin_pub_key,
          decimal: 9,
          name: "xtar",
          symbol: "xtar",
        },
        { simulate: false }
      );
      await submit_txn(txn_token, ADMIN);

      const txn_mint = await token.mint(
        {
          to: admin_pub_key,

          amount: BigInt(100000),
        },
        { simulate: false }
      );
      await submit_txn(txn_mint, ADMIN);
      const a = await token.balance({ id: admin_pub_key }, { simulate: true });
      const b = await submit_txn(a, ADMIN);
      console.log(b.value());
      // const client = get_client_pool(
      //   admin_pub_key,
      //   "CDOWGLBOFVTR3YN7A27IMLOU3NI42PJIGOFPACDFFFCCPBKKOEJDMPHY"
      // );
      // const a = await token.allowance(
      //   {
      //     from: admin_pub_key,
      //     spender: "CAZ2VSH5WPQHHKHTCVZ3R4YRQ4ABJ6DSKBX4Q4NZB2ZRDUBTG24RJIPA",
      //   },
      //   { simulate: false, timeoutInSeconds: 60 }
      // );
      // const b = await submit_txn(a, ADMIN);
      // console.log(b.value());
      // const tx = await token.approve(
      //   {
      //     from: admin_pub_key,
      //     spender: "CAZ2VSH5WPQHHKHTCVZ3R4YRQ4ABJ6DSKBX4Q4NZB2ZRDUBTG24RJIPA",
      //     amount: BigInt(5000),
      //     expiration_ledger: 500,
      //   },
      //   { simulate: false, timeoutInSeconds: 60 }
      // );
      // await submit_txn(tx, ADMIN);
    });
  });
});
