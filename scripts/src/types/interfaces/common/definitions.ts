export default {
  rpc: {},
  types: {
    SafeRpcWrapper: "String",
    CustomRpcCurrencyId: "SafeRpcWrapper",
    CustomRpcBalance: "SafeRpcWrapper",
    CurrencyId: "u128",
    AssetsBalance: "u128",
    ComposableTraitsDefiSellCurrencyId: "CurrencyId",
    ComposableTraitsDefiCurrencyPairCurrencyId: {
      base: "CurrencyId",
      quote: "CurrencyId"
    },
    ComposableTraitsXcmCumulusMethodId: "Null",
    ComposableTraitsXcmXcmSellRequest: "Null",
    PalletContractsSchedule: "Null",
    XcvmCoreProgramXcvmProgram: "Null",
    PalletContractsWasmPrefabWasmModule: "Null",
    PalletContractsStorageRawContractInfo: "Null",
    PalletContractsStorageDeletedContract: "Null",
    PalletContractsWasmOwnerInfo: "Null",
    XcvmCoreAssetXcvmTransferDisplayed: "Null",
  },
};
