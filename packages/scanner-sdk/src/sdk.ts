import axios, { AxiosInstance } from "axios";
import { GetAddressesBytecodeRO, ScanSmartContractRO } from "./ro";
import { Scanner } from "@quartz-technology/massa-sc-scanner-core";

/**
 * This SDK provides a way to scan smart contracts bytecode deployed on the Massa blockchain.
 */
export class ScannerSDK {
    /**
     * Axios client instance.
     * @private
     */
    private client: AxiosInstance;

    /**
     * The URL of the RPC server.
     */
    private readonly rpcURL: string;

    /**
     * Creates an instance of the ScannerSDK.
     * @param rpcURL The URL of the RPC server.
     */
    constructor(rpcURL: string) {
        this.rpcURL = rpcURL;
        this.client = axios.create({
            baseURL: `${this.rpcURL}/api/v2`,
            timeout: 5000,
        });
    }

    /**
     * Scans a smart contract given its address.
     * @param address The address of the smart contract.
     * @returns The exported function names and host functions from the smart contract.
     */
    async scanSmartContract(address: string): Promise<ScanSmartContractRO> {
        const res = await this.client.post<GetAddressesBytecodeRO>("/", {
            jsonrpc: "2.0",
            id: 1,
            method: "get_addresses_bytecode",
            params: [[
                {
                    address: address,
                    is_final: false,
                }
            ]],
        });

        if (res.data.error) {
            throw new Error(JSON.stringify(res.data.error));
        }

        const scanner = new Scanner(Buffer.from(res.data.result![0]));

        return {
            exported_function_names: scanner.exported_function_names(),
            host_functions: scanner.host_functions(),
        } as ScanSmartContractRO;    
    }
}