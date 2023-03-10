use pallet_evm::{Precompile, PrecompileHandle, PrecompileResult, PrecompileSet};
use sp_core::H160;
use sp_runtime::{traits::AccountIdLookup, AccountId32};
use sp_std::{fmt::Debug, marker::PhantomData};

use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_dispatch::Dispatch;
use pallet_evm_precompile_ed25519::Ed25519Verify;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_template_precompile::PalletTemplatePrecompile;
use pallet_uvm_precompile::PalletUvmPrecompile;

/// The PrecompileSet installed in the runtime.
#[derive(Debug, Default, Clone, Copy)]
pub struct Precompiles<R>(PhantomData<R>);

impl<R> Precompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}

	pub fn used_address() -> impl Iterator<Item = H160> {
		sp_std::vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1024, 1025, 1026, 1027, 2048, 2049]
			.into_iter()
			.map(hash)
	}
}

impl<R> PrecompileSet for Precompiles<R>
where
	R: pallet_evm::Config
		+ pallet_template::Config
		+ pallet_contracts::Config
		+ frame_system::Config<Lookup = AccountIdLookup<AccountId32, ()>>,
	Dispatch<R>: Precompile,
	PalletTemplatePrecompile<R>: Precompile,
	PalletUvmPrecompile<R>: Precompile,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// Ethereum precompiles :
			addr if addr == hash(1) => Some(ECRecover::execute(handle)),
			addr if addr == hash(2) => Some(Sha256::execute(handle)),
			addr if addr == hash(3) => Some(Ripemd160::execute(handle)),
			addr if addr == hash(4) => Some(Identity::execute(handle)),
			addr if addr == hash(5) => Some(Modexp::execute(handle)),
			addr if addr == hash(6) => Some(Bn128Add::execute(handle)),
			addr if addr == hash(7) => Some(Bn128Mul::execute(handle)),
			addr if addr == hash(8) => Some(Bn128Pairing::execute(handle)),
			addr if addr == hash(9) => Some(Blake2F::execute(handle)),
			// nor Ethereum precompiles :
			addr if addr == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			addr if addr == hash(1025) => Some(Dispatch::<R>::execute(handle)),
			addr if addr == hash(1026) => Some(ECRecoverPublicKey::execute(handle)),
			addr if addr == hash(1027) => Some(Ed25519Verify::execute(handle)),
			//0x0000000000000000000000000000000000000800 in hexadecimal
			addr if addr == hash(2048) => Some(PalletUvmPrecompile::<R>::execute(handle)),
			//0x0000000000000000000000000000000000000801 in hexadecimal
			addr if addr == hash(2049) => Some(PalletTemplatePrecompile::<R>::execute(handle)),
			_ => None,
		}
	}
	fn is_precompile(&self, address: H160) -> bool {
		Self::used_address().any(|precompile_addr| precompile_addr == address)
	}
}

fn hash(addr: u64) -> H160 {
	H160::from_low_u64_be(addr)
}
