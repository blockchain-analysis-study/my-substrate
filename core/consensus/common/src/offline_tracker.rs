// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Tracks offline validators.

use std::collections::HashMap;
use std::time::{Instant, Duration};

// time before we report a validator.
const REPORT_TIME: Duration = Duration::from_secs(60 * 5);

struct Observed {
	last_round_end: Instant,
	offline_since: Instant,
}

impl Observed {
	fn new() -> Observed {
		let now = Instant::now();
		Observed {
			last_round_end: now,
			offline_since: now,
		}
	}

	fn note_round_end(&mut self, was_online: bool) {
		let now = Instant::now();

		self.last_round_end = now;
		if was_online {
			self.offline_since = now;
		}
	}

	fn is_active(&self) -> bool {
		// can happen if clocks are not monotonic
		if self.offline_since > self.last_round_end { return true }
		self.last_round_end.duration_since(self.offline_since) < REPORT_TIME
	}
}

/// Tracks offline validators and can issue a report for those offline.
pub struct OfflineTracker<AuthorityId> {
	observed: HashMap<AuthorityId, Observed>,
}

impl<AuthorityId: Eq + Clone + std::hash::Hash> OfflineTracker<AuthorityId> {
	/// Create a new tracker.
	pub fn new() -> Self {
		OfflineTracker { observed: HashMap::new() }
	}

	/// Note new consensus is starting with the given set of validators.
	pub fn note_new_block(&mut self, validators: &[AuthorityId]) {
		use std::collections::HashSet;

		let set: HashSet<_> = validators.iter().cloned().collect();
		self.observed.retain(|k, _| set.contains(k));
	}

	/// Note that a round has ended.
	pub fn note_round_end(&mut self, validator: AuthorityId, was_online: bool) {
		self.observed.entry(validator)
			.or_insert_with(Observed::new)
			.note_round_end(was_online);
	}

	/// Generate a vector of indices for offline account IDs.
	pub fn reports(&self, validators: &[AuthorityId]) -> Vec<u32> {
		validators.iter()
			.enumerate()
			.filter_map(|(i, v)| if self.is_online(v) {
				None
			} else {
				Some(i as u32)
			})
			.collect()
	}

	/// Whether reports on a validator set are consistent with our view of things.
	pub fn check_consistency(&self, validators: &[AuthorityId], reports: &[u32]) -> bool {
		reports.iter().cloned().all(|r| {
			let v = match validators.get(r as usize) {
				Some(v) => v,
				None => return false,
			};

			// we must think all validators reported externally are offline.
			let thinks_online = self.is_online(v);
			!thinks_online
		})
	}

	fn is_online(&self, v: &AuthorityId) -> bool {
		self.observed.get(v).map(Observed::is_active).unwrap_or(true)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use primitives::ed25519::Public as AuthorityId;

	#[test]
	fn validator_offline() {
		let mut tracker = OfflineTracker::<AuthorityId>::new();
		let v = AuthorityId::from_raw([0; 32]);
		let v2 = AuthorityId::from_raw([1; 32]);
		let v3 = AuthorityId::from_raw([2; 32]);
		tracker.note_round_end(v.clone(), true);
		tracker.note_round_end(v2.clone(), true);
		tracker.note_round_end(v3.clone(), true);

		let slash_time = REPORT_TIME + Duration::from_secs(5);
		tracker.observed.get_mut(&v).unwrap().offline_since -= slash_time;
		tracker.observed.get_mut(&v2).unwrap().offline_since -= slash_time;

		assert_eq!(tracker.reports(&[v.clone(), v2.clone(), v3.clone()]), vec![0, 1]);

		tracker.note_new_block(&[v.clone(), v3.clone()]);
		assert_eq!(tracker.reports(&[v, v2, v3]), vec![0]);
	}
}
