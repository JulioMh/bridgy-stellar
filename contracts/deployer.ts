import * as Client from '../packages/deployer/dist/index.js';
import { rpcUrl } from './util.ts';

export const get_client_deployer = (publicKey: string)=> new Client.Client({
  ...Client.networks.standalone,
  rpcUrl,
  allowHttp: true,
  publicKey,
});
