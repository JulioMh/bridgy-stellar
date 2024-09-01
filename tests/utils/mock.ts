import "dotenv/config";
import {
  Address,
  Keypair,
  nativeToScVal,
  scValToNative,
  xdr,
} from "../../packages/deployer/dist/index.js";

export const XTAR = "GAORYJ3KBDGIM7FFSKVUJHJ5NEFWIRDIAGGBJBJS7TY6ECZS53257IG4";
export const XTAR_sol = "3B5wuUrMEi5yATD7on46hKfej3pfmd7t1RKgrsN3pump";
export const ADMIN = Keypair.fromSecret(process.env.ADMIN_PRIV_KEY);
export const ADMIN_ADDRESS_VAL = nativeToScVal(
  Address.fromString(ADMIN.publicKey())
);
export const OWNER = Keypair.fromSecret(process.env.OWNER_PRIV_KEY);
export const USER = Keypair.fromSecret(process.env.USER_PRIV_KEY);
export const BE = Buffer.from(process.env.BE_PUB_KEY, "hex");
export const BE_VAL = nativeToScVal(BE);

export function scValToAuthority(scVal: xdr.ScVal): Authority {
  const [val1, val2] = scVal.value() as unknown as [
    xdr.ScMapEntry,
    xdr.ScMapEntry
  ];

  return {
    [scValToNative(val1.key())]: scValToNative(val1.val()),
    [scValToNative(val2.key())]: scValToNative(val2.val()),
  } as unknown as Authority;
}

export interface Authority {
  signer: Address;
  fee_wallet: Address;
}
