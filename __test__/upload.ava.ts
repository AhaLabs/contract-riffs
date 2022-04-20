import { Workspace } from "near-workspaces-ava";
import { NEAR } from "near-units";
import {
    binPath, cost_of_bytes,
} from "./util";
import * as fs from "fs/promises";

const price = NEAR.parse("1 N");
// const min_cost = NEAR.parse("0.01 N");

const bin = binPath("contract_registry");

const runner = Workspace.init(
    { initialBalance: NEAR.parse("15 N").toString() },
    async ({ root }) => {
        let registry = await root.createAndDeploy("registry", bin);

        return { registry };
    }
);

runner.test("cover storage costs", async (t, { root, registry }) => {
    let bytes = await fs.readFile(bin);
    await root.call(registry, "upload", bytes, { attachedDeposit: cost_of_bytes(bytes).add(NEAR.parse("0.1 N")) });
});

runner.test("doesn't cover storage", async (t, { root, registry }) => {
    let bytes = await fs.readFile(bin);
    await t.throwsAsync(root.call(registry, "upload", bytes));
});
