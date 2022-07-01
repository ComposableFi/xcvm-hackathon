import type { Null, Struct, Text, u128 } from '@polkadot/types-codec';
/** @name AssetsBalance */
export interface AssetsBalance extends u128 {
}
/** @name ComposableTraitsDefiCurrencyPairCurrencyId */
export interface ComposableTraitsDefiCurrencyPairCurrencyId extends Struct {
    readonly base: CurrencyId;
    readonly quote: CurrencyId;
}
/** @name ComposableTraitsDefiSellCurrencyId */
export interface ComposableTraitsDefiSellCurrencyId extends CurrencyId {
}
/** @name ComposableTraitsXcmCumulusMethodId */
export interface ComposableTraitsXcmCumulusMethodId extends Null {
}
/** @name ComposableTraitsXcmXcmSellRequest */
export interface ComposableTraitsXcmXcmSellRequest extends Null {
}
/** @name CurrencyId */
export interface CurrencyId extends u128 {
}
/** @name CustomRpcBalance */
export interface CustomRpcBalance extends SafeRpcWrapper {
}
/** @name CustomRpcCurrencyId */
export interface CustomRpcCurrencyId extends SafeRpcWrapper {
}
/** @name PalletContractsSchedule */
export interface PalletContractsSchedule extends Null {
}
/** @name PalletContractsStorageDeletedContract */
export interface PalletContractsStorageDeletedContract extends Null {
}
/** @name PalletContractsStorageRawContractInfo */
export interface PalletContractsStorageRawContractInfo extends Null {
}
/** @name PalletContractsWasmOwnerInfo */
export interface PalletContractsWasmOwnerInfo extends Null {
}
/** @name PalletContractsWasmPrefabWasmModule */
export interface PalletContractsWasmPrefabWasmModule extends Null {
}
/** @name SafeRpcWrapper */
export interface SafeRpcWrapper extends Text {
}
/** @name XcvmCoreAssetXcvmTransferDisplayed */
export interface XcvmCoreAssetXcvmTransferDisplayed extends Null {
}
/** @name XcvmCoreProgramXcvmProgram */
export interface XcvmCoreProgramXcvmProgram extends Null {
}
export declare type PHANTOM_COMMON = 'common';
