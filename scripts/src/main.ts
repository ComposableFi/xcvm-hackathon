import "./types/interfaces";
import "./types/interfaces/lookup";
import "./types/interfaces/types-lookup";
import "./types/interfaces/augment-api";
import "./types/interfaces/augment-types";
import * as definitions from "./types/interfaces/definitions";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ApiOptions } from "@polkadot/api/types";
import { KeyringPair } from "@polkadot/keyring/types";
import { randomInt } from "crypto";
import { readFileSync } from "fs";
import {
  sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess
} from "./tx";
import { AccountId, CodeHash } from "@polkadot/types/interfaces";
import { AnyTuple } from "@polkadot/types-codec/types";
import { IEvent } from "@polkadot/types/types";
import { blake2AsHex } from "@polkadot/util-crypto";

const devWallets = (keyring: Keyring): {
  devWalletAlice: KeyringPair,
  devWalletBob: KeyringPair,
  devWalletCharlie: KeyringPair,
  devWalletDave: KeyringPair,
  devWalletEve: KeyringPair,
  devWalletFerdie: KeyringPair
} => ({
  devWalletAlice: <KeyringPair>keyring.addFromUri("//Alice"),
  devWalletBob: <KeyringPair>keyring.addFromUri("//Bob"),
  devWalletCharlie: <KeyringPair>keyring.addFromUri("//Charlie"),
  devWalletDave: <KeyringPair>keyring.addFromUri("//Dave"),
  devWalletEve: <KeyringPair>keyring.addFromUri("//Eve"),
  devWalletFerdie: <KeyringPair>keyring.addFromUri("//Ferdie")
});

const connect = async () => {
  const rpc = Object.keys(definitions)
    .filter(k => Object.keys(definitions[k].rpc).length > 0)
    .reduce((accumulator, key) => ({ ...accumulator, [key]: definitions[key].rpc }), {});
  const types = Object.values(definitions).reduce((accumulator, { types }) => ({ ...accumulator, ...types }), {});
  const endpoint = "ws://127.0.0.1:9988";
  const provider = new WsProvider(endpoint);
  const apiOptions: ApiOptions = {
    provider, types, rpc
  };
  const api = await ApiPromise.create(apiOptions);
  const keyring = new Keyring({ type: "sr25519" });
  return { api, keyring };
};

const XCVM_NETWORK_PICASSO = 1;
const XCVM_NETWORK_ETH = 2;

const XCVM_ASSET_PICA = 1;
const XCVM_ASSET_WETH = 2;
const XCVM_ASSET_USDT = 3;
const XCVM_ASSET_USDC = 4;

const COMPOSABLE_ASSET_PICA = 1;
const COMPOSABLE_ASSET_ETH = 130;
const COMPOSABLE_ASSET_USDT = 131;
const COMPOSABLE_ASSET_USDC = 132;

const setupInitialState = async (api: ApiPromise, sudoKey: KeyringPair, account: KeyringPair) => {
  const satellite = await api.query.xcvm.satellite.entries();
  if (satellite.length != 0) {
    console.log("Initial state already setup.");
    return;
  }

  console.log("Funding account...");
  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(api.createType("CurrencyId", 1), account.address, "100000000000000000000000000000000")
    )
  );

  const relayer = sudoKey;
  const mosaicEthereumNetworkId = api.createType("u128", XCVM_NETWORK_ETH);
  const maxTransferSize = 1000000000000000;
  const decayer = api.createType("PalletMosaicDecayBudgetPenaltyDecayer", {
    Linear: api.createType("PalletMosaicDecayLinearDecay", {
      factor: api.createType("u128", 5)
    })
  });
  const remoteAssetId = api.createType("CommonMosaicRemoteAssetId", {
    EthereumTokenAddress: api.createType("[u8; 20]", "0x0000000000000000000000000000000000000001")
  });
  const budget = maxTransferSize;

  console.log("Setting up mosaic relayer...");
  const localAssetId = api.createType("u128", 1);
  await sendWithBatchAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is, [
    api.tx.sudo.sudo(api.tx.mosaic.setRelayer(relayer.address)),
    api.tx.sudo.sudo(api.tx.mosaic.setBudget(localAssetId, budget, decayer)),
  ],
  );

  console.log("Configuring ethereum network on mosaic...");
  const networkInfo = api.createType("PalletMosaicNetworkInfo", {
    enabled: api.createType("bool", true),
    minTransferSize: api.createType("u128", 0),
    maxTransferSize: api.createType("u128", maxTransferSize)
  });
  await sendAndWaitForSuccess(
    api,
    relayer,
    api.events.mosaic.NetworksUpdated.is,
    api.tx.mosaic.setNetwork(mosaicEthereumNetworkId, networkInfo),
  );

  console.log("Updating asset mapping...");
  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.mosaic.updateAssetMapping(localAssetId, mosaicEthereumNetworkId, remoteAssetId))
  );

  console.log("Setting up XCVM satellite...");
  await sendAndWaitForSuccess(api, sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.xcvm.setSatellite(
        XCVM_NETWORK_ETH,
        [mosaicEthereumNetworkId, api.createType("ComposableSupportEthereumAddress", "0x")]
      )
    )
  );
  console.log("Initial state setup completed successfully.");
};

const uploadContract = async (api: ApiPromise, account: KeyringPair, code: Buffer): Promise<CodeHash> => {
  console.log("Uploading contract... Size: " + code.length);
  const hash = blake2AsHex(Uint8Array.from(code.values()));
  const storage = await api.query.cosmwasm.codeStorage(hash);
  if(storage.isSome) {
    console.log("Code already uploaded.");
    return api.createType("CodeHash", hash);
  }
  let {
    data: [codeHash]
  } = await sendAndWaitForSuccess(
    api,
    account,
    api.events.cosmwasm.CodeStored.is,
    api.tx.cosmwasm.uploadCode(
      api.createType("Bytes", "0x" + code.toString("hex")),
      null
    )
  );
  return codeHash;
};

const encodeCosmwasmMsg = (api: ApiPromise, msg: Object) =>
  api.createType("Bytes", JSON.stringify(msg));

const instantiateContract = async (
  api: ApiPromise,
  account: KeyringPair,
  funds: Object,
  salt: number,
  codeHash: CodeHash,
  message: Object,
): Promise<AccountId> => {
  console.log("Instantiating contract with hash: ", codeHash.toHuman());
  const gas = api.createType("u64", 300000000000);
  let {
    data: [_, contractAddress]
  } = await sendAndWaitForSuccess(
    api,
    account,
    api.events.cosmwasm.Instantiated.is,
    api.tx.cosmwasm.instantiate(
      api.createType("BTreeMap<u128, u128>", funds),
      gas,
      null,
      codeHash,
      encodeCosmwasmMsg(api, message),
      api.createType("Bytes", salt),
    )
  );
  console.log("Contract instantiated at: ", contractAddress.toHuman());
  return contractAddress;
};

const executeContract = async <T extends AnyTuple>(
  api: ApiPromise,
  account: KeyringPair,
  contractAddress: AccountId,
  funds: Object,
  message: Object,
  eventFilter: (event: IEvent<AnyTuple>) => event is IEvent<T>
): Promise<IEvent<T>> => {
  const gas = api.createType("u64", 300000000000);
  return await sendAndWaitForSuccess(
    api,
    account,
    eventFilter,
    api.tx.cosmwasm.call(
      contractAddress,
      api.createType("BTreeMap<u128, u128>", funds),
      gas,
      null,
      encodeCosmwasmMsg(api, message))
  );
};

const etphonehome = async (api: ApiPromise, admin: KeyringPair, account: KeyringPair) => {

  await setupInitialState(api, admin, account);

  const code = readFileSync("xcvm.wasm");

  const codeHash = await uploadContract(api, account, code);

  // Instantiate the contract without funds and a random salt to avoid collision while testing.
  const instantiateMsg = {};
  const contractAddress = await instantiateContract(api, account, {}, randomInt(0xCAFEBABE), codeHash, instantiateMsg);

  // The message amounts are string as Cosmwasm use string for u128 repr in JSON
  const executeMsg = {
    et_phone_home: {
      amount_in: "1000000000000",
      amount_out: "5000000000000",
    }
  };
  const funds = { [COMPOSABLE_ASSET_PICA]: 9000000000000 };
  const { data: [network]} = await executeContract(
    api,
    account,
    contractAddress,
    funds,
    executeMsg,
    api.events.xcvm.Spawn.is
  );
  console.log("XCVM program spawned on network: ", network);
};

const main = async () => {
  const { api, keyring } = await connect();
  const { devWalletAlice } = devWallets(keyring);
  await etphonehome(
    api,
    devWalletAlice,
    keyring.addFromMnemonic("chunk silent below help stem crew reduce canvas grant desert raven century")
  )
};


main();
