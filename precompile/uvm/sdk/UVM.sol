// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

import "./IUVM.sol";

contract UVM is IUVM {
    // Invoke the WASM smart contract function deployed at `contract_address` with the given `input`.
    // The function hash 
    function uvmCall(bytes calldata contract_address, bytes calldata input) override external {
        
        bytes4 function_selector = calcFuncSelector("uvm_call(bytes,bytes)");
        
        bytes memory encoded_data = encode(function_selector, contract_address, input);
    
        assembly {
            // The first 32 bytes of dynamic memory array stores the length of the value.
            let len := mload(encoded_data)
            // The uvm precompile is stored at address: 0x800.
            if iszero(call(gas(), 0x800, 0, add(encoded_data, 0x20), len, 0x40, 0)) {
                revert(0, 0)
            }
        }
        
    }

    
    // Helper functions used to encode the call data.

    function calcFuncSelector(string memory func) private pure returns (bytes4) {
        return bytes4(keccak256(bytes(func)));
    }

    function encode(bytes4 selector, bytes memory contract_address, bytes memory input) public pure returns (bytes memory) {
        return abi.encodeWithSelector(selector, contract_address, input);
    }
}