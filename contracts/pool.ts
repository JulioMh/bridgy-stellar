import * as Client from '../packages/pool/dist/index.js';
import { rpcUrl } from './util.ts';

export const get_client_pool = (publicKey: string)=> new Client.Client({
  ...Client.networks.standalone,
  rpcUrl,
  allowHttp: true,
  publicKey,
});
