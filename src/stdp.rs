/// STDP (Spike-Timing-Dependent Plasticity) parameters.
///
/// ANALOGY: This is the "learning rule" — like Hebb's Rule on a timer.
/// "Neurons that fire together wire together" but only if the timing is right.
pub const STDP_TAU_PLUS: f32 = 20.0;   // LTP time constant (ms / steps)
pub const STDP_TAU_MINUS: f32 = 20.0;  // LTD time constant (ms / steps)
pub const STDP_A_PLUS: f32 = 0.01;     // Max LTP amplitude
pub const STDP_A_MINUS: f32 = 0.012;   // Max LTD amplitude (slightly stronger → stability)
pub const STDP_W_MIN: f32 = 0.0;       // Minimum weight (no negative / inhibitory yet)
pub const STDP_W_MAX: f32 = 2.0;       // Maximum weight (prevents runaway excitation)
