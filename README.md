# Thermal Equilibrium

[![crates.io](https://img.shields.io/crates/v/thermal-equilibrium.svg)](https://crates.io/crates/thermal-equilibrium)
[![docs.rs](https://docs.rs/thermal-equilibrium/badge.svg)](https://docs.rs/thermal-equilibrium)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Thermal metaphor for multi-agent load balancing — workload as temperature, redistribution as heat transfer.**

---

## The Problem

Multi-agent systems need load balancing, but traditional approaches (round-robin, least-connections) are rigid. They don't model the natural tendency of systems to seek equilibrium, and they can't express nuanced policies like "preferentially transfer work to nearby agents."

## Why This Exists

Thermal Equilibrium models load balancing as thermodynamics:
- **Temperature** = workload (higher = busier)
- **Heat transfer** = work redistribution between agents
- **Thermal equilibrium** = balanced workload across the system
- **Overheated** = overloaded agent
- **Idle** = agent with no work

The thermal metaphor gives you natural mixing, weighted redistribution, and overload detection.

## Architecture

```
  Hot Agent (T=90)          Cold Agent (T=10)
  ┌──────────────┐          ┌──────────────┐
  │ ████████████ │── heat ─→│ ██           │
  │ overloaded   │  transfer│ underloaded  │
  └──────────────┘          └──────────────┘
       T=50                      T=50
       └──── Equilibrium ────────┘

  Temperature operations:
  • heat(amount) → increase workload
  • cool(amount) → decrease workload
  • mix(other)   → average temperatures
  • weighted_mix → capacity-aware mixing
```

## Installation

```toml
[dependencies]
thermal-equilibrium = "0.1"
```

## API Reference

### `Temperature`

Workload as a temperature value:

```rust
use thermal_equilibrium::temperature::Temperature;

let t = Temperature::new(100.0);
assert!(!t.is_idle());
assert!(t.is_overheated(80.0));

let heated = t.heat(50.0);
let cooled = t.cool(30.0);
let mixed = Temperature::new(80.0).mix(&Temperature::new(20.0));
// mixed = 50.0 (average)
```

### `ThermalAgent`

An agent with temperature and capacity:

```rust
use thermal_equilibrium::temperature::ThermalAgent;

let agent = ThermalAgent::new("worker-1", 75.0, 100.0);
assert!(agent.is_overloaded()); // 75/100 = 75%, not overloaded
assert!((agent.utilization() - 0.75).abs() < 0.01);
```

### Mixing & Transfer

```rust
use thermal_equilibrium::temperature::Temperature;

let hot = Temperature::new(90.0);
let cold = Temperature::new(10.0);

// Simple mix (average)
let balanced = hot.mix(&cold); // 50.0

// Weighted mix (capacity-aware)
let weighted = hot.weighted_mix(&cold, 1.0, 3.0); // weighted toward cold
```

## Usage Examples

### Example 1: Load Balancing

```rust
use thermal_equilibrium::temperature::*;

let overloaded = ThermalAgent::new("a", 95.0, 100.0);
let underloaded = ThermalAgent::new("b", 20.0, 100.0);

// Transfer heat from overloaded to underloaded
let transfer = 37.5; // half the difference
let new_hot = overloaded.temperature().cool(transfer);
let new_cold = underloaded.temperature().heat(transfer);
// Both now at ~57.5
```

### Example 2: Capacity-Aware Balancing

```rust
use thermal_equilibrium::temperature::*;

let small = Temperature::new(50.0);
let large = Temperature::new(50.0);

// Large agent should take more work
let balanced = small.weighted_mix(&large, 1.0, 4.0);
// Weighted average: (50*1 + 50*4) / 5 = 50.0 (same temp, different capacity)
```

## Performance

| Operation | Complexity |
|-----------|-----------|
| Temperature arithmetic | O(1) |
| Mix/weighted mix | O(1) |
| Agent overload check | O(1) |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests
4. Push and open a Pull Request

## Mathematical Background

**Newton's Law of Cooling**: The rate of heat transfer between two bodies is proportional to their temperature difference:

```
dT/dt = -k(T - T_env)
```

In the agent context, this becomes: the rate of workload transfer is proportional to the load imbalance.

**Thermal Equilibrium**: When all agents reach the same temperature (workload), no further transfer occurs. This is the desired balanced state.

**Weighted Mixing**: When agents have different capacities, the equilibrium temperature accounts for capacity:

```
T_eq = (T₁C₁ + T₂C₂) / (C₁ + C₂)
```

This is analogous to mixing water at different temperatures in containers of different sizes.

### Zeroth Law of Thermodynamics

If agent A is in thermal equilibrium with agent B, and B is in equilibrium with agent C, then A is in equilibrium with C. This transitive property enables cascading load balancing across the fleet.

### Entropy and Load Distribution

System entropy increases as workload distributes more evenly:

```
S = -Σ (T_i/T_total) × ln(T_i/T_total)
```

Maximum entropy = perfectly balanced load.

## API Reference

### `Temperature`

```rust
use thermal_equilibrium::temperature::Temperature;

let t = Temperature::new(100.0);
let heated = t.heat(50.0);    // → 150.0
let cooled = t.cool(30.0);    // → 70.0
let mixed = t.mix(&Temperature::new(50.0)); // → 75.0

// Arithmetic operations
let sum = Temperature::new(60.0) + Temperature::new(40.0); // 100.0
let diff = Temperature::new(60.0) - Temperature::new(20.0); // 40.0
```

### `ThermalAgent`

```rust
use thermal_equilibrium::temperature::ThermalAgent;

let agent = ThermalAgent::new("worker-1", 75.0, 100.0);
assert_eq!(agent.id(), "worker-1");
assert!((agent.temperature().value() - 75.0).abs() < 0.001);
assert!((agent.capacity() - 100.0).abs() < 0.001);
assert!(!agent.is_overloaded()); // 75/100 = 75%
```

## Modules

| Module | Description |
|--------|-------------|
| `temperature` | Temperature type, ThermalAgent, mixing operations |

## Usage Examples

### Example 3: Multi-Agent Equilibrium

```rust
use thermal_equilibrium::temperature::*;

let agents = vec![
    ThermalAgent::new("a", 95.0, 100.0),
    ThermalAgent::new("b", 30.0, 100.0),
    ThermalAgent::new("c", 50.0, 100.0),
];

let total_temp: f64 = agents.iter()
    .map(|a| a.temperature().value())
    .sum();
let equilibrium = total_temp / agents.len() as f64;
// All agents should converge to this temperature
```

## Comparison with Alternatives

| Feature | thermal-equilibrium | round-robin | least-conn |
|---------|-------------------|-------------|------------|
| Natural load balancing | ✅ | ❌ | Partial |
| Capacity-aware | ✅ | ❌ | ✅ |
| Overload detection | ✅ | ❌ | ✅ |
| Intuitive metaphor | ✅ | ❌ | ❌ |
| Continuous transfer | ✅ | ❌ | ❌ |
