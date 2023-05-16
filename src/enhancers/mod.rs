//! This module contains various functions that enhance / cleanup `Collections`

mod adjust_lines_names;
mod check_stop_times_order;
mod enhance_pickup_dropoff;
mod fill_co2;
mod memory_shrink;

pub(crate) use adjust_lines_names::adjust_lines_names;
pub(crate) use check_stop_times_order::check_stop_times_order;
pub(crate) use enhance_pickup_dropoff::enhance_pickup_dropoff;
pub(crate) use fill_co2::fill_co2;
pub(crate) use fill_co2::FALLBACK_PHYSICAL_MODES;
pub(crate) use memory_shrink::memory_shrink;
