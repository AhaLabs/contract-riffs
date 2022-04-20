import {
    Account,
    transactions,
    providers,
    DEFAULT_FUNCTION_CALL_GAS,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    f32,
    f64,
    BN,
    ChangeMethodOptions,
    ViewFunctionOptions,
} from './helper';

/**
* StorageUsage is used to count the amount of storage used by a contract.
*/
export type StorageUsage = u64;
/**
* Balance is a type for storing amounts of tokens, specified in yoctoNEAR.
*/
export type Balance = U128;
/**
* Represents the amount of NEAR tokens in "gas units" which are used to fund transactions.
*/
export type Gas = u64;
/**
* base64 string.
*/
export type Base64VecU8 = string;
/**
* Raw type for duration in nanoseconds
*/
export type Duration = u64;
/**
* @minLength 2
* @maxLength 64
* @pattern ^(([a-z\d]+[-_])*[a-z\d]+\.)*([a-z\d]+[-_])*[a-z\d]+$
*/
export type AccountId = string;
/**
* String representation of a u128-bit integer
* @pattern ^[0-9]+$
*/
export type U128 = string;
/**
* Public key in a binary format with base58 string serialization with human-readable curve.
* The key types currently supported are `secp256k1` and `ed25519`.
* 
* Ed25519 public keys accepted are 32 bytes and secp256k1 keys are the uncompressed 64 format.
*/
export type PublicKey = string;
/**
* Raw type for timestamp in nanoseconds
*/
export type Timestamp = u64;

export class Contract {

    constructor(public account: Account, public readonly contractId: string) { }

    /**
    * Stores the bytes at its corresponding sha256 hash
    */
    async upload(args: Uint8Array, options?: ChangeMethodOptions): Promise<Uint8Array> {
        return providers.getTransactionLastResult(await this.uploadRaw(args, options));
    }
    /**
    * Stores the bytes at its corresponding sha256 hash
    */
    uploadRaw(args: Uint8Array, options?: ChangeMethodOptions): Promise<providers.FinalExecutionOutcome> {
        return this.account.functionCall({ contractId: this.contractId, methodName: "upload", args, ...options });
    }
    /**
    * Stores the bytes at its corresponding sha256 hash
    */
    uploadTx(args: Uint8Array, options?: ChangeMethodOptions): transactions.Action {
        return transactions.functionCall("upload", args, options ?.gas ?? DEFAULT_FUNCTION_CALL_GAS, options ?.attachedDeposit ?? new BN(0))
    }
    /**
    * Fetch binary corresponding the sha256
    */
    fetch(args: Uint8Array, options?: ViewFunctionOptions): Promise<Uint8Array> {
        return this.account.viewFunction(this.contractId, "fetch", args, options);
    }
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
    }

}
export type Upload__Result = Uint8Array;
/**
* Fetch binary corresponding the sha256
* 
* @contractMethod view
*/
export interface Fetch {
    args: Uint8Array;
}
export type Fetch__Result = Uint8Array;
