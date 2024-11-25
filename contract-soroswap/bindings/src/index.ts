import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CA23N7TEQAFRCG5HDKTXF77DYHANNXIAMLGQKILMYXTCS2PYPSAECHB7",
  }
} as const

export const Errors = {
  1: {message:"AlreadyInitialized"},

  2: {message:"NotInitialized"},

  3: {message:"InvalidAmount"},

  4: {message:"InvalidAddress"},

  5: {message:"SwapFailed"}
}

/**
 * Represents the configuration for a swap
 */
export interface SwapConfig {
  /**
 * The factory address for swap operations
 */
factory: string;
  /**
 * The router address for executing swaps
 */
router: string;
}

/**
 * Error types for swap-related operations
 */
export type SwapError = {tag: "NotInitialized", values: void} | {tag: "AmountTooLow", values: void} | {tag: "AmountTooHigh", values: void} | {tag: "InsufficientOutputBalance", values: void} | {tag: "SlippageTooHigh", values: void} | {tag: "Unauthorized", values: void};


export interface Client {
  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  initialize: ({router, factory}: {router: string, factory: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_expected_output transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_expected_output: ({amount_in, input_token, output_token}: {amount_in: i128, input_token: string, output_token: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<i128>>>

  /**
   * Construct and simulate a swap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  swap: ({customer, merchant, amount_in, input_token, output_token}: {customer: string, merchant: string, amount_in: i128, input_token: string, output_token: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<i128>>>

  /**
   * Construct and simulate a get_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_config: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<SwapConfig>>>

}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAgAAAAAAAAAGcm91dGVyAAAAAAATAAAAAAAAAAdmYWN0b3J5AAAAABMAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAATZ2V0X2V4cGVjdGVkX291dHB1dAAAAAADAAAAAAAAAAlhbW91bnRfaW4AAAAAAAALAAAAAAAAAAtpbnB1dF90b2tlbgAAAAATAAAAAAAAAAxvdXRwdXRfdG9rZW4AAAATAAAAAQAAA+kAAAALAAAAAw==",
        "AAAAAAAAAAAAAAAEc3dhcAAAAAUAAAAAAAAACGN1c3RvbWVyAAAAEwAAAAAAAAAIbWVyY2hhbnQAAAATAAAAAAAAAAlhbW91bnRfaW4AAAAAAAALAAAAAAAAAAtpbnB1dF90b2tlbgAAAAATAAAAAAAAAAxvdXRwdXRfdG9rZW4AAAATAAAAAQAAA+kAAAALAAAAAw==",
        "AAAAAAAAAAAAAAAKZ2V0X2NvbmZpZwAAAAAAAAAAAAEAAAPpAAAH0AAAAApTd2FwQ29uZmlnAAAAAAAD",
        "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAABQAAAAAAAAASQWxyZWFkeUluaXRpYWxpemVkAAAAAAABAAAAAAAAAA5Ob3RJbml0aWFsaXplZAAAAAAAAgAAAAAAAAANSW52YWxpZEFtb3VudAAAAAAAAAMAAAAAAAAADkludmFsaWRBZGRyZXNzAAAAAAAEAAAAAAAAAApTd2FwRmFpbGVkAAAAAAAF",
        "AAAAAQAAACdSZXByZXNlbnRzIHRoZSBjb25maWd1cmF0aW9uIGZvciBhIHN3YXAAAAAAAAAAAApTd2FwQ29uZmlnAAAAAAACAAAAJ1RoZSBmYWN0b3J5IGFkZHJlc3MgZm9yIHN3YXAgb3BlcmF0aW9ucwAAAAAHZmFjdG9yeQAAAAATAAAAJlRoZSByb3V0ZXIgYWRkcmVzcyBmb3IgZXhlY3V0aW5nIHN3YXBzAAAAAAAGcm91dGVyAAAAAAAT",
        "AAAAAgAAACdFcnJvciB0eXBlcyBmb3Igc3dhcC1yZWxhdGVkIG9wZXJhdGlvbnMAAAAAAAAAAAlTd2FwRXJyb3IAAAAAAAAGAAAAAAAAABhDb250cmFjdCBub3QgaW5pdGlhbGl6ZWQAAAAOTm90SW5pdGlhbGl6ZWQAAAAAAAAAAAAmU3dhcCBhbW91bnQgaXMgYmVsb3cgbWluaW11bSB0aHJlc2hvbGQAAAAAAAxBbW91bnRUb29Mb3cAAAAAAAAAIVN3YXAgYW1vdW50IGV4Y2VlZHMgbWF4aW11bSBsaW1pdAAAAAAAAA1BbW91bnRUb29IaWdoAAAAAAAAAAAAACFJbnN1ZmZpY2llbnQgb3V0cHV0IHRva2VuIGJhbGFuY2UAAAAAAAAZSW5zdWZmaWNpZW50T3V0cHV0QmFsYW5jZQAAAAAAAAAAAAAbU3dhcCBmYWlsZWQgZHVlIHRvIHNsaXBwYWdlAAAAAA9TbGlwcGFnZVRvb0hpZ2gAAAAAAAAAABlVbmF1dGhvcml6ZWQgc3dhcCBhdHRlbXB0AAAAAAAADFVuYXV0aG9yaXplZA==" ]),
      options
    )
  }
  public readonly fromJSON = {
    initialize: this.txFromJSON<Result<void>>,
        get_expected_output: this.txFromJSON<Result<i128>>,
        swap: this.txFromJSON<Result<i128>>,
        get_config: this.txFromJSON<Result<SwapConfig>>
  }
}