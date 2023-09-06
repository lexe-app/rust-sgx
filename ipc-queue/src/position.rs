/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;
use std::sync::atomic::Ordering;

/// `PositionMonitor<T>` can be used to record the current read/write positions
/// of a queue. Even though a queue is comprised of a limited number of slots
/// arranged as a ring buffer, we can assign a position to each value written/
/// read to/from the queue. This is useful in case we want to know whether or
/// not a particular value written to the queue has been read.
pub struct PositionMonitor<T: 'static> {
    pub(crate) read_epoch: Arc<AtomicU64>,
    pub(crate) fifo: Fifo<T>,
}

/// A read position in a queue.
pub struct ReadPosition(u64);

/// A write position in a queue.
pub struct WritePosition(u64);

impl<T> PositionMonitor<T> {
    pub fn read_position(&self) -> ReadPosition {
        let current = self.fifo.current_offsets(Ordering::Relaxed);
        let read_epoch = self.read_epoch.load(Ordering::Relaxed);
        let read_epoch_shifted = read_epoch
            .checked_shl(32)
            .expect("Reading from position of over 2^32 (2 to the power of 32). This is unsupported.");
        ReadPosition(read_epoch_shifted | (current.read_offset() as u64))
    }

    pub fn write_position(&self) -> WritePosition {
        let current = self.fifo.current_offsets(Ordering::Relaxed);
        let mut write_epoch = self.read_epoch.load(Ordering::Relaxed);
        if current.read_high_bit() != current.write_high_bit() {
            write_epoch += 1;
        }
        let write_epoch_shifted = write_epoch
            .checked_shl(32)
            .expect("Writing to position of over 2^32 (2 to the power of 32). This is unsupported.");
        WritePosition(write_epoch_shifted | (current.write_offset() as u64))
    }
}

impl<T> Clone for PositionMonitor<T> {
    fn clone(&self) -> Self {
        Self {
            read_epoch: self.read_epoch.clone(),
            fifo: self.fifo.clone(),
        }
    }
}

impl ReadPosition {
    /// A `WritePosition` can be compared to a `ReadPosition` **correctly** if
    /// at most 2³¹ (2 to the power of 31) writes
    /// have occurred since the write position was recorded.
    pub fn is_past(&self, write: &WritePosition) -> bool {
        let (read, write) = (self.0, write.0);
        let hr = read & (1 << 63);
        let hw = write & (1 << 63);
        if hr == hw {
            return read > write;
        }
        true
    }
}