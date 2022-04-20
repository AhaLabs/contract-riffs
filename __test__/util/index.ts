import { Gas, NEAR } from "near-units";
import {
    Account,
    BN,
    createKeyPair,
    NearAccount,
    Workspace,
    randomAccountId,
    KeyPair,
    PublicKey,
    AccountManager,
} from "near-workspaces";
import { ONE_NEAR, TransactionResult } from "near-workspaces-ava";
import { binPath } from "./bin";
import { BalanceDelta, getDelta } from "./delta";

export * from "./bin";

// This will allow the contract account to be deleted since the size is reduced
export async function deployEmpty(account: NearAccount): Promise<void> {
    if (!Workspace.networkIsTestnet()) {
        return;
    }
    const empty = account.getFullAccount("empty.tn");
    const bytes = await empty.viewCode();
    await account.createTransaction(account).deployContract(bytes).signAndSend();
}

export function now() {
    return Date.now();
}

export const COST_PER_BYTE = NEAR.parse("10 μN");
export const SHA256_BYTES = 32;
export const EXTRA_RECORD_BYTES = 40;

export function cost_of_bytes(bytes: Uint8Array): NEAR {
    return COST_PER_BYTE.mul(NEAR.from(bytes.length + SHA256_BYTES + EXTRA_RECORD_BYTES));
}

export class ActualTestnet extends Account {
    constructor(private name: string) {
        super(null as any, null as any);
    }

    get accountId(): string {
        return this.name;
    }
}

export function get_gas_profile(res) {
    return res.result.receipts_outcome
        .map((outcome) => {
            const gas_profile = outcome.outcome["metadata"].gas_profile;
            return gas_profile.map((info) => {
                info.gas_used = Gas.parse(info.gas_used).toHuman();
                return JSON.stringify(info, null, 2);
            });
        })
        .join("\n");
}


export * from "./delta";

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
