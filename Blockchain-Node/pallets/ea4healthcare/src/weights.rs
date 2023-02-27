#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn add_info() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl <T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_info() -> Weight {
		(Weight::from_ref_time(13_129_000))
			.saturating_add((Weight::from_ref_time(19_026_000)))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64).saturating_mul(2_64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64).saturating_mul(2_u64))
	}
}

impl WeightInfo for () {
	fn add_info() -> Weight {
		(Weight::from_ref_time(13_129_000))
			.saturating_add((Weight::from_ref_time(19_026_000)))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64).saturating_mul(2_64))
			.saturating_add(RocksDbWeight::get().writes(2_u64).saturating_mul(2_64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}