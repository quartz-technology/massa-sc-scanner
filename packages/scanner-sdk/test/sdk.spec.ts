import { assert, expect } from "chai";
import { describe, it } from "mocha";
import { ScannerSDK } from "../src/sdk";

describe("Scanner SDK Tests", () => {
    it("should get bytecode for address #1", async () => {
        const expected_exported_function_names = [
            'constructor',
            'version',
            'name',
            'symbol',
            'totalSupply',
            'decimals',
            'balanceOf',
            'transfer',
            'allowance',
            'increaseAllowance',
            'decreaseAllowance',
            'transferFrom',
            'mint',
            'burn',
            'burnFrom'
        ];
        const expected_host_functions = [
            'assembly_script_caller_has_write_access',
            'assembly_script_get_call_stack',
            'assembly_script_set_data',
            'assembly_script_has_data',
            'assembly_script_get_data',
            'assembly_script_generate_event',
        ];
        const sdk = new ScannerSDK("https://buildnet.massa.net");
        const res = await sdk.scanSmartContract("AS1sKBEGsqtm8vQhQzi7KJ4YhyaKTSkhJrLkRc7mQtPqme3VcFHm");

        assert.isNotNull(res);
        assert.isArray(res.exported_function_names);
        assert.isArray(res.host_functions);

        assert.deepEqual(res.exported_function_names, expected_exported_function_names);
        assert.deepEqual(res.host_functions, expected_host_functions);
    });

    it("should get bytecode for address #2", async () => {
        const expected_exported_function_names = [
            'constructor',
            'createLBPair',
            'addLiquidity',
            'addLiquidityMAS',
            'removeLiquidity',
            'removeLiquidityMAS',
            'swapExactTokensForTokens',
            'swapExactTokensForMAS',
            'swapExactMASForTokens',
            'swapTokensForExactTokens',
            'swapTokensForExactMAS',
            'swapMASForExactTokens',
            'sweep',
            'sweepLBToken',
            'getSwapIn',
            'getSwapOut'
        ];
        const expected_host_functions = [
            'assembly_script_caller_has_write_access',
            'assembly_script_set_data',
            'assembly_script_get_data',
            'assembly_script_call',
            'assembly_script_get_data_for',
            'assembly_script_get_call_stack',
            'assembly_script_get_call_coins',
            'assembly_script_get_time',
            'assembly_script_transfer_coins',
            'assembly_script_get_balance_for'
        ];
        const sdk = new ScannerSDK("https://buildnet.massa.net");
        const res = await sdk.scanSmartContract("AS1qCXCY5AF7SpZUk2bwiHneYb5MLVgeWEiRs8BxorYArLWMavZ");

        assert.isNotNull(res);
        assert.isArray(res.exported_function_names);
        assert.isArray(res.host_functions);

        assert.deepEqual(res.exported_function_names, expected_exported_function_names);
        assert.deepEqual(res.host_functions, expected_host_functions);
    });

    it("should fail for inexisting smart contract", async () => {
        const sdk = new ScannerSDK("https://buildnet.massa.net");
        expect(sdk.scanSmartContract("0000000000000000000000000000000000000000000000000000")).to.throw;
    });
});