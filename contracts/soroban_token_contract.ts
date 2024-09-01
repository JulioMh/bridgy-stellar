import * as Client from '../packages/soroban_token_contract/dist/index.js';
import { rpcUrl } from './util.ts';

export const get_client_soroban_token_contract = (publicKey: string)=> new Client.Client({
  ...Client.networks.standalone,
  rpcUrl,
  allowHttp: true,
  publicKey,
});
