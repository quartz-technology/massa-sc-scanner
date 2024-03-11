export interface GetAddressesBytecodeRO {
    result?: number[][],
    error?: {
        code: number,
        message: string,
        data: string,
    },
}

export interface ScanSmartContractRO {
    exported_function_names: string[],
    host_functions: string[],
}
