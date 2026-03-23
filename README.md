# Neuromod - Reward-Modulated Spiking Neural Networks

[![Crates.io](https://img.shields.io/crates/v/neuromod.svg)](https://crates.io/crates/neuromod)
[![Docs.rs](https://img.shields.io/badge/docs.rs-neuromod-blue.svg)](https://docs.rs/neuromod)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL_3.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![GitHub](https://img.shields.io/badge/GitHub-rmems/neuromod-black.svg)](https://github.com/rmems/neuromod)

A lightweight, focused Rust crate for neuromorphic computing with reward-modulated spiking neural networks. Designed for high-frequency trading (HFT) applications and FPGA deployment.

## Features

- **LIF Neurons**: Fast, reactive leaky integrate-and-fire neurons
- **Izhikevich Neurons**: Complex, adaptive neuron dynamics with rich firing patterns
- **STDP Learning**: Spike-timing-dependent plasticity with reward modulation
- **Neuromodulators**: Dopamine, cortisol, acetylcholine, tempo control, and mining efficiency rewards
- **HFT Optimized**: Built for real-time trading applications with microsecond latency
- **FPGA Ready**: Architecture supports hardware acceleration deployment
- **Mining Integration**: Lean mining efficiency reward signals without bloat

## Quick Start

```rust
use neuromod::{SpikingNetwork, NeuroModulators};

// Create network
let mut network = SpikingNetwork::new();

// Create input stimuli (16 channels)
let stimuli = [0.5, 0.3, 0.8, 0.2, 0.1, 0.9, 0.4, 0.7,
               0.6, 0.2, 0.8, 0.3, 0.5, 0.1, 0.9, 0.4];

// Create neuromodulators from telemetry
let modulators = NeuroModulators::from_telemetry(
    75.0,  // GPU temp
    300.0, // Power (W)
    0.05,  // Hashrate (MH/s)
    2640.0 // GPU clock (MHz)
);

// Step the network
let spikes = network.step(&stimuli, &modulators);
println!("Neurons that spiked: {:?}", spikes);

// Get membrane potentials
let potentials = network.get_membrane_potentials();
println!("Membrane potentials: {:?}", potentials);
```

## Architecture

### Neuron Banks

1. **LIF Neurons (Bank 1)**: 16 fast, reactive neurons organized as 8 bear/bull pairs
   - N0-N1: Asset pair 0 (DNX)
   - N2-N3: Asset pair 1 (QUAI)
   - N4-N5: Asset pair 2 (QUBIC)
   - N6-N7: Asset pair 3 (KASPA)
   - N8-N9: Asset pair 4 (XMR)
   - N10-N11: Asset pair 5 (OCEAN)
   - N12-N13: Asset pair 6 (VERUS)
   - N14: Coincidence detector (fires when ≥3 pairs spike together)
   - N15: Global inhibitory interneuron

2. **Izhikevich Neurons (Bank 2)**: 5 complex adaptive neurons for hardware telemetry

### Neuromodulator System

- **Dopamine**: Reward signal based on hashrate performance
- **Cortisol**: Stress signal from temperature and power
- **Acetylcholine**: Focus signal from voltage stability
- **Tempo**: Timing scale based on GPU clock speed

### Learning Mechanisms

- **STDP**: Spike-timing-dependent plasticity with exponential learning windows
- **Reward Modulation**: Dopamine scales learning rate
- **Synaptic Scaling**: L1 weight normalization prevents runaway excitation
- **Competitive Inhibition**: Bear/bull pairs compete for activation

## Use Cases

### High-Frequency Trading
```rust
// Real-time market processing
let market_data = get_market_data();
let stimuli = normalize_market_data(&market_data);
let spikes = network.step(&stimuli, &modulators);

// Execute trades based on neural spikes
for &neuron_id in &spikes {
    if neuron_id < 14 { // Trading neurons only
        execute_trade(neuron_id);
    }
}
```

### Hardware Monitoring
```rust
// Create modulators from GPU telemetry
let modulators = NeuroModulators::from_telemetry(
    gpu_temp,
    power_draw,
    hashrate,
    gpu_clock
);

// Network adapts to hardware conditions
let spikes = network.step(&stimuli, &modulators);

// Check stress levels
if modulators.is_stressed() {
    reduce_mining_intensity();
}
```

## Performance

- **Latency**: < 1μs per network step
- **Memory**: ~2KB for full 16-neuron network
- **Throughput**: > 1M steps/second on single core
- **Deterministic**: No allocations in hot path

## FPGA Integration

The architecture is designed for FPGA deployment with:
- Fixed-point arithmetic support
- Parallel neuron evaluation
- Hardware STDP implementation
- Low-latency spike propagation

## License

Licensed under the GNU General Public License, Version 3.0 ([GPL-3.0](LICENSE) or https://www.gnu.org/licenses/gpl-3.0)

## Contribution

Contributions are welcome! Please feel free to submit a Pull Request.

## Repository

- **GitHub**: https://github.com/rmems/neuromod
- **Crates.io**: https://crates.io/crates/neuromod

---

*Built for the Spikenaut HFT system - neuromorphic computing for real-time trading*
