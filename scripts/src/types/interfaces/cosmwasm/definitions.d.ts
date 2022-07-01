declare const _default: {
    rpc: {
        instantiate: {
            description: string;
            params: ({
                name: string;
                type: string;
                isOptional?: undefined;
            } | {
                name: string;
                type: string;
                isOptional: boolean;
            })[];
            type: string;
        };
        call: {
            description: string;
            params: ({
                name: string;
                type: string;
                isOptional?: undefined;
            } | {
                name: string;
                type: string;
                isOptional: boolean;
            })[];
            type: string;
        };
        query: {
            description: string;
            params: ({
                name: string;
                type: string;
                isOptional?: undefined;
            } | {
                name: string;
                type: string;
                isOptional: boolean;
            })[];
            type: string;
        };
    };
    types: {
        Code: {
            _enum: {
                Upload: string;
                Existing: string;
            };
        };
        StorageDeposit: {
            _enum: {
                Refund: string;
                Charge: string;
            };
        };
        ContractExecResult: {
            gas_consumed: string;
            gas_required: string;
            storage_deposit: string;
            debug_message: string;
            result: string;
        };
        ContractInstantiateResult: {
            gas_consumed: string;
            gas_required: string;
            storage_deposit: string;
            debug_message: string;
            result: string;
        };
        Amount: {
            _enum: {
                Fixed: string;
                Ratio: string;
            };
        };
    };
};
export default _default;
