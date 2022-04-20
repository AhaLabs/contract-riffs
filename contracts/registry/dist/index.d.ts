import { Account, transactions, providers, u64, ChangeMethodOptions, ViewFunctionOptions } from './helper';
/**
* StorageUsage is used to count the amount of storage used by a contract.
*/
export declare type StorageUsage = u64;
/**
* Balance is a type for storing amounts of tokens, specified in yoctoNEAR.
*/
export declare type Balance = U128;
/**
* Represents the amount of NEAR tokens in "gas units" which are used to fund transactions.
*/
export declare type Gas = u64;
/**
* base64 string.
*/
export declare type Base64VecU8 = string;
/**
* Raw type for duration in nanoseconds
*/
export declare type Duration = u64;
/**
* @minLength 2
* @maxLength 64
* @pattern ^(([a-z\d]+[-_])*[a-z\d]+\.)*([a-z\d]+[-_])*[a-z\d]+$
*/
export declare type AccountId = string;
/**
* String representation of a u128-bit integer
* @pattern ^[0-9]+$
*/
export declare type U128 = string;
/**
* Public key in a binary format with base58 string serialization with human-readable curve.
* The key types currently supported are `secp256k1` and `ed25519`.
*
* Ed25519 public keys accepted are 32 bytes and secp256k1 keys are the uncompressed 64 format.
*/
export declare type PublicKey = string;
/**
* Raw type for timestamp in nanoseconds
*/
export declare type Timestamp = u64;
export declare class Contract {
    account: Account;
    readonly contractId: string;
    constructor(account: Account, contractId: string);
    /**
    * Stores the bytes at its corresponding sha256 hash
    */
    upload(args: Uint8Array, options?: ChangeMethodOptions): Promise<Uint8Array>;
    /**
    * Stores the bytes at its corresponding sha256 hash
    */
    uploadRaw(args: Uint8Array, options?: ChangeMethodOptions): Promise<providers.FinalExecutionOutcome>;
    /**
    * Stores the bytes at its corresponding sha256 hash
    */
    uploadTx(args: Uint8Array, options?: ChangeMethodOptions): transactions.Action;
    /**
    * Fetch binary corresponding the sha256
    */
    fetch(args: Uint8Array, options?: ViewFunctionOptions): Promise<Uint8Array>;
}
/**
* Stores the bytes at its corresponding sha256 hash
*
* @contractMethod change
*/
export interface Upload {
    args: Uint8Array;
    options: {
        /** Units in gas
        * @pattern [0-9]+
        * @default "30000000000000"
        */
        gas?: string;
        /** Units in yoctoNear
        * @default "0"
        */
        attachedDeposit?: Balance;
    };
}
export declare type Upload__Result = Uint8Array;
/**
* Fetch binary corresponding the sha256
*
* @contractMethod view
*/
export interface Fetch {
    args: Uint8Array;
}
export declare type Fetch__Result = Uint8Array;
