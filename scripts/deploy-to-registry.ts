import { Gas } from "near-units";
import { readFile } from "fs/promises";
import { Context } from "near-cli/context";
import * as registry from "../contracts/registry/dist";

export async function main({ account, nearAPI, argv, near }: Context) {
    if (argv.length < 2) {
        console.error("<contract> <file>")
        return;
    }
    if (!account) {
        console.error("requires --accountId");
        return;
    }
    let [contractId, fileName] = argv;
    const contractBytes = await readFile(fileName);
    const contract = new registry.Contract(account, contractId);

    console.log(Buffer.from(await contract.upload(contractBytes, { gas: Gas.parse("250 TGas") })).toString("hex"))
}
