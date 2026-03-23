use serde::{Deserialize, Serialize};

/// This struct simulates a more complex, biologically plausible neuron model
/// developed by Dr. Eugene M. Izhikevich. It can reproduce many different
/// firing patterns (bursting, chattering, etc.) with only two equations and four parameters.
///
/// ANALOGY:
/// This is like a programmable oscillator. Changing `a,b,c,d` is like swapping
/// out different components to change the oscillation pattern.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IzhikevichNeuron {
    // State variables
    pub v: f32, // Membrane potential (mV)
    pub u: f32, // Membrane recovery variable

    // Parameters that define firing patterns
    pub a: f32, // Timescale of the recovery variable `u`
    pub b: f32, // Sensitivity of `u` to the subthreshold fluctuations of `v`
    pub c: f32, // After-spike reset value of `v`
    pub d: f32, // After-spike reset of `u`
}

impl IzhikevichNeuron {
    /// Creates a new neuron with parameters for "regular spiking" behavior.
    pub fn new_regular_spiking() -> Self {
        let a = 0.02;
        let b = 0.2;
        let c = -65.0;
        Self {
            v: c,     // Start at resting potential
            u: b * c, // Start recovery variable at its equilibrium
            a,
            b,
            c,
            d: 8.0,
        }
    }

    /// Simulates one time step (e.g., 1ms) of the neuron's dynamics,
    /// returning `true` if the neuron fired.
    /// The input `i` is the injected current.
    pub fn step(&mut self, i: f32) -> bool {
        // To improve the stability of the numerical simulation (Euler method),
        // the original paper suggests applying the update for `v` twice per time step.
        // This is equivalent to using a smaller time step (e.g., 0.5ms).
        for _ in 0..2 {
            self.v += 0.04 * self.v * self.v + 5.0 * self.v + 140.0 - self.u + i;
        }
        self.u += self.a * (self.b * self.v - self.u);

        // Check for spike
        if self.v >= 30.0 {
            self.v = self.c; // Reset potential
            self.u += self.d; // Reset recovery variable
            true // Spike occurred
        } else {
            false // No spike
        }
    }
}
