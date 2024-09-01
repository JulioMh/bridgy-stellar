import "dotenv/config";
import { mkdirSync, writeFileSync, rmSync, readFileSync } from "fs";
import { execSync } from "child_process";
import path from "path";
import { fileURLToPath } from "url";
import { sync as glob } from "glob";

// Load environment variables starting with PUBLIC_ into the environment,
// so we don't need to specify duplicate variables in .env
for (const key in process.env) {
  if (key.startsWith("PUBLIC_")) {
    process.env[key.substring(7)] = process.env[key];
  }
}

console.log("###################### Initializing ########################");

// Get dirname (equivalent to the Bash version)
const __filename = fileURLToPath(import.meta.url);
const dirname = path.dirname(__filename);

// variable for later setting pinned version of soroban in "$(dirname/target/bin/soroban)"
const cli = "stellar";

// Function to execute and log shell commands
function exe(command) {
  console.log(command);
  execSync(command, { stdio: "inherit" });
}

function fundAll() {
  for (const key in process.env) {
    if (key.endsWith("PRIV_KEY")) {
      const name = key.split("_")[0].toLowerCase();
      process.env.SOROBAN_SECRET_KEY = process.env[key];
      exe(`${cli} keys add ${name} --secret-key`);
      exe(`${cli} keys fund ${name}`);
    }
  }
}

function removeFiles(pattern) {
  console.log(`remove ${pattern}`);
  glob(pattern).forEach((a) => rmSync(a));
}

function buildAll() {
  removeFiles(`${dirname}/target/wasm32-unknown-unknown/release/*.wasm`);
  removeFiles(`${dirname}/target/wasm32-unknown-unknown/release/*.d`);
  exe(`${cli} contract build`);
}

function filenameNoExtension(filename) {
  return path.basename(filename, path.extname(filename));
}

function deploy(wasm) {
  exe(
    `${cli} contract deploy --source admin --wasm ${wasm} --ignore-checks --alias ${filenameNoExtension(
      wasm
    )}`
  );
}

function deployAll() {
  const contractsDir = `${dirname}/.soroban/contract-ids`;
  mkdirSync(contractsDir, { recursive: true });

  const wasmFiles = glob(
    `${dirname}/target/wasm32-unknown-unknown/release/*.wasm`
  );

  wasmFiles.forEach(deploy);
}

function contracts() {
  const contractFiles = glob(`${dirname}/.soroban/contract-ids/*.json`);

  return contractFiles
    .map((path) => ({
      alias: filenameNoExtension(path),
      ...JSON.parse(readFileSync(path)),
    }))
    .filter((data) => data.ids[process.env.SOROBAN_NETWORK_PASSPHRASE])
    .map((data) => ({
      alias: data.alias,
      id: data.ids[process.env.SOROBAN_NETWORK_PASSPHRASE],
    }));
}

function bind({ alias, id }) {
  exe(
    `${cli} contract bindings typescript --contract-id ${id} --output-dir ${dirname}/packages/${alias} --overwrite`
  );
}

function bindAll() {
  contracts().forEach(bind);
}

function importContract({ id, alias }) {
  const outputDir = `${dirname}/contracts/`;

  mkdirSync(outputDir, { recursive: true });

  const importContent =
    `import * as Client from '../packages/${alias}/dist/index.js';\n` +
    `import { rpcUrl } from './util.ts';\n\n` +
    `export const get_client_${alias} = (publicKey: string)=> new Client.Client({\n` +
    `  ...Client.networks.${process.env.SOROBAN_NETWORK},\n` +
    `  rpcUrl,\n` +
    `${
      process.env.SOROBAN_NETWORK === "local" || "standalone"
        ? `  allowHttp: true,\n`
        : null
    }` +
    `  publicKey,\n` +
    `});\n`;

  const outputPath = `${outputDir}/${alias}.ts`;

  writeFileSync(outputPath, importContent);

  console.log(`Created import for ${alias}`);
}

function importAll() {
  contracts().forEach(importContract);
}

// Calling the functions (equivalent to the last part of your bash script)
fundAll();
buildAll();
deployAll();
bindAll();
importAll();
