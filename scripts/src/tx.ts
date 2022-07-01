import { ApiPromise } from "@polkadot/api";
import { AnyTuple, IEvent } from "@polkadot/types/types";
import { SubmittableExtrinsic, AddressOrPair } from "@polkadot/api/types";

export async function sendUnsignedAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return await sendUnsignedAndWaitFor(api, filter, call);
}

/**
 * Sends a signed extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export async function sendAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return await sendAndWaitFor(api, sender, filter, call);
}

/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export async function sendWithBatchAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">[],
): Promise<IEvent<T>> {
  return await sendAndWaitForWithBatch(api, sender, filter, call);
}

/**
 * Sends the given unsigned `call` and waits for an event that fits `filter`.
 * @param {ApiPromise} api api object
 * @param {IEvent} filter which event to filter for
 * @param {SubmittableExtrinsic<Promise>} call a call that can be submitted to the chain
 * @param {boolean} intendedToFail If true a failed submission will be counted as a success.
 * @returns event that fits the filter
 */
export function sendUnsignedAndWaitFor<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>(function(resolve, reject) {
    call
      .send(function(res) {
        const { dispatchError, status } = res;
        if (dispatchError) {
          if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          const envelope = res.events.find(e => filter(e.event));
          if (envelope === undefined)
            return reject(status.toString());
          if (filter(envelope.event)) {
            resolve(envelope.event);
          } else {
            reject(Error("Invalid event"));
          }
        }
      })
      .catch(function(e) {
        reject(Error(e.stack));
      });
  });
}

/**
 * Signs and sends the given `call` from `sender` and waits for an event that fits `filter`.
 * @param api api object
 * @param sender the sender of the transaction
 * @param filter which event to filter for
 * @param call a call that can be submitted to the chain
 * @param {boolean} intendedToFail If true a failed submission will be counted as a success.
 * @returns event that fits the filter
 */
export function sendAndWaitFor<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>(function(resolve, reject) {
    call
      .signAndSend(sender, { nonce: -1 }, function(res) {
        const { dispatchError, status } = res;
        if (dispatchError) {
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          const envelope = res.events.find(e => filter(e.event));
          if (envelope === undefined)
            return reject(status.toString());
          if (filter(envelope.event)) {
            resolve(envelope.event);
          } else {
            reject(Error("Invalid event"));
          }
        }
      })
      .catch(function(e) {
        reject(Error(e.stack));
      });
  });
}

/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export function sendAndWaitForWithBatch<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">[],
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>(function(resolve, reject) {
    api.tx.utility.batchAll(call)
      .signAndSend(sender, { nonce: -1 }, function(res) {
        const { dispatchError, status } = res;
        if (dispatchError) {
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          const envelope = res.events.find(e => filter(e.event));
          if (envelope === undefined)
            return reject(status.toString());
          if (filter(envelope.event)) {
            resolve(envelope.event);
          } else {
            reject(Error("Invalid event"));
          }
        }
      })
      .catch(function(e) {
        reject(Error(e.stack));
      });
  });
}
