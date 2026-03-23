# Changelog

## [0.2.1] - 2026-03-23

### Added
- **Mining dopamine reward** - New `mining_dopamine` field in NeuroModulators for mining efficiency signals
- **MiningReward struct** - Simple EMA-based mining reward calculation with thermal penalties
- **Extended HftReward trait** - Added `mining_efficiency_bonus()` method for mining-specific rewards
- **Lean mining integration** - Mining reward signals without bloating the core crate

### Changed
- **NeuroModulators structure** - Added mining_dopamine field while maintaining backward compatibility
- **Default values** - Updated NeuroModulators::default() to include mining_dopamine: 0.0
- **Decay method** - Extended natural decay to include mining_dopamine

### Fixed
- **Clean architecture** - Removed heavy mining telemetry dependencies that would bloat the crate
- **Performance preservation** - Maintained sub-1 µs modulator updates and < 2k SLoC footprint

### Performance
- **Zero bloat** - Mining integration adds minimal overhead (no new dependencies)
- **Sub-1 µs updates** - Mining reward computation maintains real-time performance
- **no_std compatible** - Core engine remains suitable for FPGA deployment

---

## [0.2.0] - 2026-03-23

### Added
- Full `HftReward` trait (`sync_bonus`, `price_reflex`, `thermal_pain`)
- jlrs zero-copy interop examples (Spikenaut HFT pipeline)
- `no_std` + FPGA `.mem` export utilities (Q8.8 fixed-point)
- Spikenaut-specific 16-channel neuron map + thermal LTD safeguard
- Proper GitHub repository link (rmems/neuromod)
- Modulator profiles (`profile_hft()`, `profile_fpga()`)

### Changed
- License to GPL-3.0-or-later (matches Spikenaut HF model)
- Keywords and categories for better crates.io discoverability

### Fixed
- Dead repo link from v0.1.0

### Performance
- <1 µs modulator update
- 1.6 KB footprint in HFT mode

---

*Built for Spikenaut-v2 — the only neuromorphic crypto HFT crate on crates.io*
