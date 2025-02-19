// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `runtime_parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a3dce7bd4066`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("spec-polkadot.json")`, DB CACHE: 1024

// Executed Command:
// /builds/polkadot-sdk/target/production/polkadot
// benchmark
// pallet
// --chain=spec-polkadot.json
// --pallet=runtime_parachains::hrmp
// --extrinsic=
// --output=/builds/runtimes/relay/polkadot/src/weights
// --header=/builds/bench/header.txt
// --no-median-slopes
// --no-min-squares

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_parachains::hrmp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::hrmp::WeightInfo for WeightInfo<T> {
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn hrmp_init_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `3848`
		// Minimum execution time: 32_938_000 picoseconds.
		Weight::from_parts(34_387_000, 0)
			.saturating_add(Weight::from_parts(0, 3848))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn hrmp_accept_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `373`
		//  Estimated: `3838`
		// Minimum execution time: 31_887_000 picoseconds.
		Weight::from_parts(32_788_000, 0)
			.saturating_add(Weight::from_parts(0, 3838))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpCloseChannelRequests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpCloseChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpCloseChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpCloseChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn hrmp_close_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `486`
		//  Estimated: `3951`
		// Minimum execution time: 33_051_000 picoseconds.
		Weight::from_parts(33_842_000, 0)
			.saturating_add(Weight::from_parts(0, 3951))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
	/// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
	/// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:254 w:254)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:0 w:1)
	/// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelContents` (r:0 w:254)
	/// Proof: `Hrmp::HrmpChannelContents` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:0 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 127]`.
	/// The range of component `e` is `[0, 127]`.
	fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `231 + e * (100 ±0) + i * (100 ±0)`
		//  Estimated: `3693 + e * (2575 ±0) + i * (2575 ±0)`
		// Minimum execution time: 1_170_406_000 picoseconds.
		Weight::from_parts(1_190_393_000, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			// Standard Error: 114_070
			.saturating_add(Weight::from_parts(3_669_481, 0).saturating_mul(i.into()))
			// Standard Error: 114_070
			.saturating_add(Weight::from_parts(3_565_988, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 2575).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2575).saturating_mul(i.into()))
	}
	/// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:128 w:128)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:256 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
	/// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
	/// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:128 w:128)
	/// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:128 w:128)
	/// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:0 w:128)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_open(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459 + c * (136 ±0)`
		//  Estimated: `1914 + c * (5086 ±0)`
		// Minimum execution time: 6_447_000 picoseconds.
		Weight::from_parts(6_522_000, 0)
			.saturating_add(Weight::from_parts(0, 1914))
			// Standard Error: 22_137
			.saturating_add(Weight::from_parts(20_773_045, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 5086).saturating_mul(c.into()))
	}
	/// Storage: `Hrmp::HrmpCloseChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpCloseChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:128 w:128)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
	/// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
	/// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpCloseChannelRequests` (r:0 w:128)
	/// Proof: `Hrmp::HrmpCloseChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelContents` (r:0 w:128)
	/// Proof: `Hrmp::HrmpChannelContents` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_close(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302 + c * (124 ±0)`
		//  Estimated: `1762 + c * (2600 ±0)`
		// Minimum execution time: 5_369_000 picoseconds.
		Weight::from_parts(5_532_000, 0)
			.saturating_add(Weight::from_parts(0, 1762))
			// Standard Error: 16_599
			.saturating_add(Weight::from_parts(12_975_587, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2600).saturating_mul(c.into()))
	}
	/// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 128]`.
	fn hrmp_cancel_open_request(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `993 + c * (13 ±0)`
		//  Estimated: `4262 + c * (15 ±0)`
		// Minimum execution time: 20_212_000 picoseconds.
		Weight::from_parts(29_660_461, 0)
			.saturating_add(Weight::from_parts(0, 4262))
			// Standard Error: 3_239
			.saturating_add(Weight::from_parts(251_998, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 15).saturating_mul(c.into()))
	}
	/// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:128 w:128)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 128]`.
	fn clean_open_channel_requests(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210 + c * (63 ±0)`
		//  Estimated: `1689 + c * (2538 ±0)`
		// Minimum execution time: 4_226_000 picoseconds.
		Weight::from_parts(4_418_000, 0)
			.saturating_add(Weight::from_parts(0, 1689))
			// Standard Error: 4_311
			.saturating_add(Weight::from_parts(3_525_389, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2538).saturating_mul(c.into()))
	}
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_open_hrmp_channel(_c: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `6558`
		// Minimum execution time: 58_716_000 picoseconds.
		Weight::from_parts(61_075_000, 0)
			.saturating_add(Weight::from_parts(0, 6558))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn establish_system_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `417`
		//  Estimated: `6357`
		// Minimum execution time: 629_674_000 picoseconds.
		Weight::from_parts(640_174_000, 0)
			.saturating_add(Weight::from_parts(0, 6357))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Hrmp::HrmpChannels` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn poke_channel_deposits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `263`
		//  Estimated: `3728`
		// Minimum execution time: 173_371_000 picoseconds.
		Weight::from_parts(175_860_000, 0)
			.saturating_add(Weight::from_parts(0, 3728))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
