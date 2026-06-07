//! # Thermal Equilibrium
//!
//! Thermal metaphor for multi-agent load balancing.
//!
//! Models agents as thermal bodies with workload represented as temperature,
//! heat transfer as work redistribution, and system balance as thermal equilibrium.

/// Agent workload as temperature.
pub mod temperature {
    /// Temperature unit representation.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Temperature(f64);

    impl Temperature {
        /// Create a new temperature value.
        pub fn new(value: f64) -> Self {
            Self(value.max(0.0))
        }

        /// Get the raw value.
        pub fn value(&self) -> f64 {
            self.0
        }

        /// Check if the temperature is zero (idle).
        pub fn is_idle(&self) -> bool {
            self.0 < 1e-10
        }

        /// Check if overheated (above threshold).
        pub fn is_overheated(&self, threshold: f64) -> bool {
            self.0 > threshold
        }

        /// Add heat (increase workload).
        pub fn heat(&self, amount: f64) -> Temperature {
            Temperature::new(self.0 + amount)
        }

        /// Cool down (decrease workload).
        pub fn cool(&self, amount: f64) -> Temperature {
            Temperature::new((self.0 - amount).max(0.0))
        }

        /// Mix two temperatures (average).
        pub fn mix(&self, other: &Temperature) -> Temperature {
            Temperature::new((self.0 + other.0) / 2.0)
        }

        /// Weighted mix.
        pub fn weighted_mix(&self, other: &Temperature, self_weight: f64, other_weight: f64) -> Temperature {
            let total = self_weight + other_weight;
            Temperature::new((self.0 * self_weight + other.0 * other_weight) / total)
        }

        /// Difference between temperatures.
        pub fn difference(&self, other: &Temperature) -> f64 {
            (self.0 - other.0).abs()
        }
    }

    impl std::ops::Add for Temperature {
        type Output = Temperature;
        fn add(self, other: Temperature) -> Temperature {
            Temperature::new(self.0 + other.0)
        }
    }

    impl std::ops::Sub for Temperature {
        type Output = Temperature;
        fn sub(self, other: Temperature) -> Temperature {
            Temperature::new((self.0 - other.0).max(0.0))
        }
    }

    impl PartialOrd for Temperature {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    /// An agent with a temperature (workload).
    #[derive(Debug, Clone)]
    pub struct ThermalAgent {
        id: String,
        temperature: Temperature,
        capacity: f64,
    }

    impl ThermalAgent {
        /// Create a new thermal agent.
        pub fn new(id: &str, temperature: f64, capacity: f64) -> Self {
            Self {
                id: id.to_string(),
                temperature: Temperature::new(temperature),
                capacity,
            }
        }

        /// Get agent ID.
        pub fn id(&self) -> &str {
            &self.id
        }

        /// Get temperature.
        pub fn temperature(&self) -> Temperature {
            self.temperature
        }

        /// Set temperature.
        pub fn set_temperature(&mut self, temp: Temperature) {
            self.temperature = temp;
        }

        /// Get capacity.
        pub fn capacity(&self) -> f64 {
            self.capacity
        }

        /// Check if agent is overloaded.
        pub fn is_overloaded(&self) -> bool {
            self.temperature.is_overheated(self.capacity)
        }

        /// Utilization ratio (0.0 to potentially > 1.0 if overloaded).
        pub fn utilization(&self) -> f64 {
            if self.capacity == 0.0 {
                return 0.0;
            }
            self.temperature.value() / self.capacity
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_temperature_creation() {
            let t = Temperature::new(100.0);
            assert!((t.value() - 100.0).abs() < 1e-10);
        }

        #[test]
        fn test_temperature_negative_clamped() {
            let t = Temperature::new(-5.0);
            assert!((t.value() - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_is_idle() {
            let t = Temperature::new(0.0);
            assert!(t.is_idle());
        }

        #[test]
        fn test_is_not_idle() {
            let t = Temperature::new(1.0);
            assert!(!t.is_idle());
        }

        #[test]
        fn test_is_overheated() {
            let t = Temperature::new(150.0);
            assert!(t.is_overheated(100.0));
        }

        #[test]
        fn test_is_not_overheated() {
            let t = Temperature::new(50.0);
            assert!(!t.is_overheated(100.0));
        }

        #[test]
        fn test_heat() {
            let t = Temperature::new(50.0);
            let heated = t.heat(30.0);
            assert!((heated.value() - 80.0).abs() < 1e-10);
        }

        #[test]
        fn test_cool() {
            let t = Temperature::new(50.0);
            let cooled = t.cool(30.0);
            assert!((cooled.value() - 20.0).abs() < 1e-10);
        }

        #[test]
        fn test_cool_below_zero() {
            let t = Temperature::new(10.0);
            let cooled = t.cool(20.0);
            assert!((cooled.value() - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_mix() {
            let a = Temperature::new(100.0);
            let b = Temperature::new(0.0);
            let mixed = a.mix(&b);
            assert!((mixed.value() - 50.0).abs() < 1e-10);
        }

        #[test]
        fn test_weighted_mix() {
            let a = Temperature::new(100.0);
            let b = Temperature::new(0.0);
            let mixed = a.weighted_mix(&b, 3.0, 1.0);
            assert!((mixed.value() - 75.0).abs() < 1e-10);
        }

        #[test]
        fn test_difference() {
            let a = Temperature::new(80.0);
            let b = Temperature::new(30.0);
            assert!((a.difference(&b) - 50.0).abs() < 1e-10);
        }

        #[test]
        fn test_add_temperatures() {
            let a = Temperature::new(10.0);
            let b = Temperature::new(20.0);
            let c = a + b;
            assert!((c.value() - 30.0).abs() < 1e-10);
        }

        #[test]
        fn test_sub_temperatures() {
            let a = Temperature::new(30.0);
            let b = Temperature::new(10.0);
            let c = a - b;
            assert!((c.value() - 20.0).abs() < 1e-10);
        }

        #[test]
        fn test_partial_ord() {
            let a = Temperature::new(10.0);
            let b = Temperature::new(20.0);
            assert!(a < b);
            assert!(b > a);
        }

        #[test]
        fn test_thermal_agent_creation() {
            let agent = ThermalAgent::new("a1", 50.0, 100.0);
            assert_eq!(agent.id(), "a1");
            assert!((agent.temperature().value() - 50.0).abs() < 1e-10);
            assert!((agent.capacity() - 100.0).abs() < 1e-10);
        }

        #[test]
        fn test_agent_overloaded() {
            let agent = ThermalAgent::new("a1", 150.0, 100.0);
            assert!(agent.is_overloaded());
        }

        #[test]
        fn test_agent_not_overloaded() {
            let agent = ThermalAgent::new("a1", 50.0, 100.0);
            assert!(!agent.is_overloaded());
        }

        #[test]
        fn test_agent_utilization() {
            let agent = ThermalAgent::new("a1", 75.0, 100.0);
            assert!((agent.utilization() - 0.75).abs() < 1e-10);
        }

        #[test]
        fn test_agent_set_temperature() {
            let mut agent = ThermalAgent::new("a1", 50.0, 100.0);
            agent.set_temperature(Temperature::new(80.0));
            assert!((agent.temperature().value() - 80.0).abs() < 1e-10);
        }
    }
}

/// Heat transfer between agents.
pub mod conductor {
    use super::temperature::{Temperature, ThermalAgent};

    /// A conductor linking two agents for heat transfer.
    #[derive(Debug, Clone)]
    pub struct Conductor {
        agent_a: String,
        agent_b: String,
        conductivity: f64,
    }

    impl Conductor {
        /// Create a new conductor.
        pub fn new(agent_a: &str, agent_b: &str, conductivity: f64) -> Self {
            Self {
                agent_a: agent_a.to_string(),
                agent_b: agent_b.to_string(),
                conductivity: conductivity.clamp(0.0, 1.0),
            }
        }

        /// Get agent A.
        pub fn agent_a(&self) -> &str {
            &self.agent_a
        }

        /// Get agent B.
        pub fn agent_b(&self) -> &str {
            &self.agent_b
        }

        /// Get conductivity.
        pub fn conductivity(&self) -> f64 {
            self.conductivity
        }

        /// Compute heat transfer between two temperatures.
        pub fn heat_transfer(&self, temp_a: Temperature, temp_b: Temperature) -> f64 {
            self.conductivity * (temp_a.value() - temp_b.value())
        }

        /// Apply one step of heat transfer to two agents.
        pub fn transfer_step(&self, agent_a: &mut ThermalAgent, agent_b: &mut ThermalAgent) {
            let transfer = self.heat_transfer(agent_a.temperature(), agent_b.temperature());
            let new_a = agent_a.temperature().cool(transfer.max(0.0)).heat((-transfer).max(0.0));
            let new_b = agent_b.temperature().heat(transfer.max(0.0)).cool((-transfer).max(0.0));
            agent_a.set_temperature(new_a);
            agent_b.set_temperature(new_b);
        }
    }

    /// A network of conductors.
    #[derive(Debug, Clone)]
    pub struct ConductorNetwork {
        conductors: Vec<Conductor>,
    }

    impl ConductorNetwork {
        /// Create an empty network.
        pub fn new() -> Self {
            Self { conductors: Vec::new() }
        }

        /// Add a conductor.
        pub fn add(&mut self, conductor: Conductor) {
            self.conductors.push(conductor);
        }

        /// Get conductors.
        pub fn conductors(&self) -> &[Conductor] {
            &self.conductors
        }

        /// Perform one transfer step across all conductors.
        pub fn global_step(&self, agents: &mut [&mut ThermalAgent]) {
            // First compute all transfers
            let transfers: Vec<(usize, usize, f64)> = self.conductors.iter()
                .filter_map(|conductor| {
                    let a_idx = agents.iter().position(|a| a.id() == conductor.agent_a)?;
                    let b_idx = agents.iter().position(|a| a.id() == conductor.agent_b)?;
                    if a_idx == b_idx { return None; }
                    let transfer = conductor.heat_transfer(agents[a_idx].temperature(), agents[b_idx].temperature());
                    Some((a_idx, b_idx, transfer))
                })
                .collect();
            // Then apply them
            for (ai, bi, transfer) in transfers {
                let new_a = agents[ai].temperature().cool(transfer.max(0.0)).heat((-transfer).max(0.0));
                let new_b = agents[bi].temperature().heat(transfer.max(0.0)).cool((-transfer).max(0.0));
                agents[ai].set_temperature(new_a);
                agents[bi].set_temperature(new_b);
            }
        }

        /// Number of conductors.
        pub fn len(&self) -> usize {
            self.conductors.len()
        }

        /// Check if empty.
        pub fn is_empty(&self) -> bool {
            self.conductors.is_empty()
        }
    }

    impl Default for ConductorNetwork {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_conductor_creation() {
            let c = Conductor::new("a", "b", 0.5);
            assert_eq!(c.agent_a(), "a");
            assert_eq!(c.agent_b(), "b");
            assert!((c.conductivity() - 0.5).abs() < 1e-10);
        }

        #[test]
        fn test_conductivity_clamped_high() {
            let c = Conductor::new("a", "b", 2.0);
            assert!((c.conductivity() - 1.0).abs() < 1e-10);
        }

        #[test]
        fn test_conductivity_clamped_low() {
            let c = Conductor::new("a", "b", -1.0);
            assert!((c.conductivity() - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_heat_transfer_hot_to_cold() {
            let c = Conductor::new("a", "b", 0.5);
            let hot = Temperature::new(100.0);
            let cold = Temperature::new(0.0);
            let transfer = c.heat_transfer(hot, cold);
            assert!(transfer > 0.0);
            assert!((transfer - 50.0).abs() < 1e-10);
        }

        #[test]
        fn test_heat_transfer_equal() {
            let c = Conductor::new("a", "b", 0.5);
            let t = Temperature::new(50.0);
            let transfer = c.heat_transfer(t, t);
            assert!((transfer - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_transfer_step() {
            let c = Conductor::new("a", "b", 1.0);
            let mut a = ThermalAgent::new("a", 100.0, 200.0);
            let mut b = ThermalAgent::new("b", 0.0, 200.0);
            c.transfer_step(&mut a, &mut b);
            assert!(a.temperature().value() < 100.0);
            assert!(b.temperature().value() > 0.0);
        }

        #[test]
        fn test_network_creation() {
            let net = ConductorNetwork::new();
            assert!(net.is_empty());
        }

        #[test]
        fn test_network_add() {
            let mut net = ConductorNetwork::new();
            net.add(Conductor::new("a", "b", 0.5));
            assert_eq!(net.len(), 1);
        }

        #[test]
        fn test_network_global_step() {
            let mut net = ConductorNetwork::new();
            net.add(Conductor::new("a", "b", 1.0));
            let mut a = ThermalAgent::new("a", 100.0, 200.0);
            let mut b = ThermalAgent::new("b", 0.0, 200.0);
            net.global_step(&mut [&mut a, &mut b]);
            assert!(a.temperature().value() < 100.0);
            assert!(b.temperature().value() > 0.0);
        }

        #[test]
        fn test_network_default() {
            let net = ConductorNetwork::default();
            assert!(net.is_empty());
        }
    }
}

/// System disorder measure.
pub mod entropy {
    use super::temperature::ThermalAgent;

    /// Compute the Shannon entropy of a workload distribution.
    pub fn workload_entropy(agents: &[ThermalAgent]) -> f64 {
        let total: f64 = agents.iter().map(|a| a.temperature().value()).sum();
        if total == 0.0 {
            return 0.0;
        }
        agents
            .iter()
            .map(|a| {
                let p = a.temperature().value() / total;
                if p > 0.0 {
                    -p * p.log2()
                } else {
                    0.0
                }
            })
            .sum()
    }

    /// Compute the maximum possible entropy (uniform distribution).
    pub fn max_entropy(n: usize) -> f64 {
        if n == 0 {
            return 0.0;
        }
        (n as f64).log2()
    }

    /// Compute the normalized entropy (0.0 to 1.0).
    pub fn normalized_entropy(agents: &[ThermalAgent]) -> f64 {
        let h = workload_entropy(agents);
        let h_max = max_entropy(agents.len());
        if h_max == 0.0 {
            return 0.0;
        }
        h / h_max
    }

    /// Compute the Gini coefficient of workload distribution.
    pub fn gini_coefficient(agents: &[ThermalAgent]) -> f64 {
        if agents.is_empty() {
            return 0.0;
        }
        let mut values: Vec<f64> = agents.iter().map(|a| a.temperature().value()).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n;
        if mean == 0.0 {
            return 0.0;
        }
        let sum_diffs: f64 = values
            .iter()
            .flat_map(|vi| values.iter().map(move |vj| (vi - vj).abs()))
            .sum();
        sum_diffs / (2.0 * n * n * mean)
    }

    /// Compute the variance of workload distribution.
    pub fn workload_variance(agents: &[ThermalAgent]) -> f64 {
        if agents.is_empty() {
            return 0.0;
        }
        let n = agents.len() as f64;
        let mean: f64 = agents.iter().map(|a| a.temperature().value()).sum::<f64>() / n;
        agents
            .iter()
            .map(|a| (a.temperature().value() - mean).powi(2))
            .sum::<f64>()
            / n
    }

    /// Compute the standard deviation of workload.
    pub fn workload_stddev(agents: &[ThermalAgent]) -> f64 {
        workload_variance(agents).sqrt()
    }

    /// Compute the coefficient of variation.
    pub fn coefficient_of_variation(agents: &[ThermalAgent]) -> f64 {
        let mean: f64 = agents.iter().map(|a| a.temperature().value()).sum::<f64>() / agents.len() as f64;
        if mean == 0.0 {
            return 0.0;
        }
        workload_stddev(agents) / mean
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn make_agents(temps: &[f64]) -> Vec<ThermalAgent> {
            temps
                .iter()
                .enumerate()
                .map(|(i, &t)| ThermalAgent::new(&format!("a{}", i), t, 100.0))
                .collect()
        }

        #[test]
        fn test_entropy_uniform() {
            let agents = make_agents(&[50.0, 50.0, 50.0, 50.0]);
            let h = workload_entropy(&agents);
            let h_max = max_entropy(4);
            assert!((h - h_max).abs() < 1e-10);
        }

        #[test]
        fn test_entropy_concentrated() {
            let agents = make_agents(&[100.0, 0.0, 0.0, 0.0]);
            let h = workload_entropy(&agents);
            assert!(h < 1.0);
        }

        #[test]
        fn test_entropy_empty() {
            let agents: Vec<ThermalAgent> = vec![];
            assert!((workload_entropy(&agents) - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_entropy_all_zero() {
            let agents = make_agents(&[0.0, 0.0, 0.0]);
            assert!((workload_entropy(&agents) - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_max_entropy() {
            assert!((max_entropy(2) - 1.0).abs() < 1e-10);
            assert!((max_entropy(4) - 2.0).abs() < 1e-10);
        }

        #[test]
        fn test_max_entropy_zero() {
            assert!((max_entropy(0) - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_normalized_entropy_uniform() {
            let agents = make_agents(&[50.0, 50.0]);
            assert!((normalized_entropy(&agents) - 1.0).abs() < 1e-10);
        }

        #[test]
        fn test_normalized_entropy_concentrated() {
            let agents = make_agents(&[100.0, 0.0]);
            assert!(normalized_entropy(&agents) < 0.5);
        }

        #[test]
        fn test_gini_equal() {
            let agents = make_agents(&[50.0, 50.0, 50.0]);
            assert!(gini_coefficient(&agents) < 0.01);
        }

        #[test]
        fn test_gini_unequal() {
            let agents = make_agents(&[100.0, 0.0]);
            assert!(gini_coefficient(&agents) > 0.3);
        }

        #[test]
        fn test_gini_empty() {
            let agents: Vec<ThermalAgent> = vec![];
            assert!((gini_coefficient(&agents) - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_variance_uniform() {
            let agents = make_agents(&[50.0, 50.0, 50.0]);
            assert!(workload_variance(&agents) < 1e-10);
        }

        #[test]
        fn test_variance_spread() {
            let agents = make_agents(&[0.0, 100.0]);
            assert!(workload_variance(&agents) > 1000.0);
        }

        #[test]
        fn test_stddev() {
            let agents = make_agents(&[0.0, 100.0]);
            let sd = workload_stddev(&agents);
            assert!(sd > 30.0);
        }

        #[test]
        fn test_coefficient_of_variation() {
            let agents = make_agents(&[50.0, 50.0]);
            assert!(coefficient_of_variation(&agents) < 0.01);
        }
    }
}

/// Convergence to balanced state.
pub mod equilibrium {
    use super::temperature::{Temperature, ThermalAgent};
    use super::conductor::{Conductor, ConductorNetwork};
    use super::entropy;

    /// Result of an equilibrium computation.
    #[derive(Debug, Clone)]
    pub struct EquilibriumResult {
        pub iterations: usize,
        pub converged: bool,
        pub final_temperatures: Vec<(String, f64)>,
    }

    /// Find thermal equilibrium by iterating heat transfer.
    pub fn find_equilibrium(
        agents: &mut [&mut ThermalAgent],
        network: &ConductorNetwork,
        max_iterations: usize,
        tolerance: f64,
    ) -> EquilibriumResult {
        let mut iter = 0;
        for i in 0..max_iterations {
            let before: Vec<f64> = agents.iter().map(|a| a.temperature().value()).collect();
            network.global_step(agents);
            let after: Vec<f64> = agents.iter().map(|a| a.temperature().value()).collect();
            let max_change = before
                .iter()
                .zip(after.iter())
                .map(|(b, a)| (b - a).abs())
                .fold(0.0_f64, f64::max);
            iter = i + 1;
            if max_change < tolerance {
                return EquilibriumResult {
                    iterations: iter,
                    converged: true,
                    final_temperatures: agents
                        .iter()
                        .map(|a| (a.id().to_string(), a.temperature().value()))
                        .collect(),
                };
            }
        }
        EquilibriumResult {
            iterations: iter,
            converged: false,
            final_temperatures: agents
                .iter()
                .map(|a| (a.id().to_string(), a.temperature().value()))
                .collect(),
        }
    }

    /// Compute the equilibrium temperature for fully connected agents.
    pub fn compute_equilibrium_temp(agents: &[ThermalAgent]) -> Temperature {
        if agents.is_empty() {
            return Temperature::new(0.0);
        }
        let total: f64 = agents.iter().map(|a| a.temperature().value()).sum();
        Temperature::new(total / agents.len() as f64)
    }

    /// Check if agents are in equilibrium (all within tolerance).
    pub fn is_equilibrium(agents: &[ThermalAgent], tolerance: f64) -> bool {
        if agents.len() < 2 {
            return true;
        }
        let avg = compute_equilibrium_temp(agents).value();
        agents
            .iter()
            .all(|a| (a.temperature().value() - avg).abs() < tolerance)
    }

    /// Measure how far from equilibrium the system is.
    pub fn distance_from_equilibrium(agents: &[ThermalAgent]) -> f64 {
        entropy::workload_stddev(agents)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_equilibrium_two_agents() {
            let mut a = ThermalAgent::new("a", 100.0, 200.0);
            let mut b = ThermalAgent::new("b", 0.0, 200.0);
            let mut net = ConductorNetwork::new();
            net.add(Conductor::new("a", "b", 0.5));
            let result = find_equilibrium(&mut [&mut a, &mut b], &net, 10000, 0.01);
            assert!(result.converged);
            assert!((a.temperature().value() - 50.0).abs() < 1.0);
            assert!((b.temperature().value() - 50.0).abs() < 1.0);
        }

        #[test]
        fn test_equilibrium_three_agents() {
            let mut a = ThermalAgent::new("a", 90.0, 200.0);
            let mut b = ThermalAgent::new("b", 30.0, 200.0);
            let mut c = ThermalAgent::new("c", 0.0, 200.0);
            let mut net = ConductorNetwork::new();
            net.add(Conductor::new("a", "b", 0.5));
            net.add(Conductor::new("b", "c", 0.5));
            let result = find_equilibrium(&mut [&mut a, &mut b, &mut c], &net, 10000, 0.01);
            assert!(result.converged);
        }

        #[test]
        fn test_compute_equilibrium_temp() {
            let agents = vec![
                ThermalAgent::new("a", 100.0, 200.0),
                ThermalAgent::new("b", 0.0, 200.0),
            ];
            let eq = compute_equilibrium_temp(&agents);
            assert!((eq.value() - 50.0).abs() < 1e-10);
        }

        #[test]
        fn test_compute_equilibrium_temp_empty() {
            let agents: Vec<ThermalAgent> = vec![];
            let eq = compute_equilibrium_temp(&agents);
            assert!((eq.value() - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_is_equilibrium_true() {
            let agents = vec![
                ThermalAgent::new("a", 50.0, 100.0),
                ThermalAgent::new("b", 50.0, 100.0),
            ];
            assert!(is_equilibrium(&agents, 1.0));
        }

        #[test]
        fn test_is_equilibrium_false() {
            let agents = vec![
                ThermalAgent::new("a", 100.0, 200.0),
                ThermalAgent::new("b", 0.0, 200.0),
            ];
            assert!(!is_equilibrium(&agents, 1.0));
        }

        #[test]
        fn test_is_equilibrium_single() {
            let agents = vec![ThermalAgent::new("a", 50.0, 100.0)];
            assert!(is_equilibrium(&agents, 0.01));
        }

        #[test]
        fn test_distance_from_equilibrium() {
            let agents = vec![
                ThermalAgent::new("a", 50.0, 100.0),
                ThermalAgent::new("b", 50.0, 100.0),
            ];
            assert!(distance_from_equilibrium(&agents) < 1e-10);
        }

        #[test]
        fn test_distance_from_equilibrium_nonzero() {
            let agents = vec![
                ThermalAgent::new("a", 100.0, 200.0),
                ThermalAgent::new("b", 0.0, 200.0),
            ];
            assert!(distance_from_equilibrium(&agents) > 0.0);
        }
    }
}

/// Isolated agent behavior.
pub mod insulation {
    use super::temperature::{Temperature, ThermalAgent};

    /// An insulated agent that does not exchange heat.
    #[derive(Debug, Clone)]
    pub struct InsulatedAgent {
        agent: ThermalAgent,
        insulation_factor: f64, // 0.0 = fully insulated, 1.0 = no insulation
    }

    impl InsulatedAgent {
        /// Create a new insulated agent.
        pub fn new(agent: ThermalAgent, insulation_factor: f64) -> Self {
            Self {
                agent,
                insulation_factor: insulation_factor.clamp(0.0, 1.0),
            }
        }

        /// Get the underlying agent.
        pub fn agent(&self) -> &ThermalAgent {
            &self.agent
        }

        /// Get mutable reference to agent.
        pub fn agent_mut(&mut self) -> &mut ThermalAgent {
            &mut self.agent
        }

        /// Get insulation factor.
        pub fn insulation_factor(&self) -> f64 {
            self.insulation_factor
        }

        /// Effective temperature after insulation.
        pub fn effective_temperature(&self, external_temp: Temperature) -> Temperature {
            let my_temp = self.agent.temperature();
            my_temp.weighted_mix(
                &external_temp,
                self.insulation_factor,
                1.0 - self.insulation_factor,
            )
        }

        /// Check if fully insulated.
        pub fn is_fully_insulated(&self) -> bool {
            self.insulation_factor == 0.0
        }

        /// Apply external temperature influence.
        pub fn apply_external(&mut self, external_temp: Temperature) {
            let new_temp = self.effective_temperature(external_temp);
            self.agent.set_temperature(new_temp);
        }
    }

    /// An insulated group of agents.
    #[derive(Debug, Clone)]
    pub struct InsulatedGroup {
        agents: Vec<InsulatedAgent>,
    }

    impl InsulatedGroup {
        /// Create a new insulated group.
        pub fn new() -> Self {
            Self { agents: Vec::new() }
        }

        /// Add an agent.
        pub fn add(&mut self, agent: InsulatedAgent) {
            self.agents.push(agent);
        }

        /// Get agents.
        pub fn agents(&self) -> &[InsulatedAgent] {
            &self.agents
        }

        /// Apply external temperature to all agents.
        pub fn apply_external_all(&mut self, external_temp: Temperature) {
            for agent in &mut self.agents {
                agent.apply_external(external_temp);
            }
        }

        /// Number of agents.
        pub fn len(&self) -> usize {
            self.agents.len()
        }

        /// Check if empty.
        pub fn is_empty(&self) -> bool {
            self.agents.is_empty()
        }

        /// Average temperature of the group.
        pub fn average_temperature(&self) -> Temperature {
            if self.agents.is_empty() {
                return Temperature::new(0.0);
            }
            let total: f64 = self.agents.iter().map(|a| a.agent().temperature().value()).sum();
            Temperature::new(total / self.agents.len() as f64)
        }
    }

    impl Default for InsulatedGroup {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_insulated_agent_creation() {
            let agent = ThermalAgent::new("a", 50.0, 100.0);
            let insulated = InsulatedAgent::new(agent, 0.5);
            assert_eq!(insulated.agent().id(), "a");
            assert!((insulated.insulation_factor() - 0.5).abs() < 1e-10);
        }

        #[test]
        fn test_fully_insulated() {
            let agent = ThermalAgent::new("a", 50.0, 100.0);
            let insulated = InsulatedAgent::new(agent, 0.0);
            assert!(insulated.is_fully_insulated());
        }

        #[test]
        fn test_effective_temperature_no_insulation() {
            let agent = ThermalAgent::new("a", 100.0, 200.0);
            let insulated = InsulatedAgent::new(agent, 1.0);
            let external = Temperature::new(0.0);
            let effective = insulated.effective_temperature(external);
            // No insulation: should keep own temp
            assert!((effective.value() - 100.0).abs() < 1e-10);
        }

        #[test]
        fn test_effective_temperature_fully_insulated() {
            let agent = ThermalAgent::new("a", 100.0, 200.0);
            let insulated = InsulatedAgent::new(agent, 0.0);
            let external = Temperature::new(0.0);
            let effective = insulated.effective_temperature(external);
            // Fully insulated: takes external temp
            assert!((effective.value() - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_apply_external() {
            let agent = ThermalAgent::new("a", 100.0, 200.0);
            let mut insulated = InsulatedAgent::new(agent, 0.5);
            insulated.apply_external(Temperature::new(0.0));
            assert!((insulated.agent().temperature().value() - 50.0).abs() < 1e-10);
        }

        #[test]
        fn test_insulation_factor_clamped() {
            let agent = ThermalAgent::new("a", 50.0, 100.0);
            let insulated = InsulatedAgent::new(agent, 2.0);
            assert!((insulated.insulation_factor() - 1.0).abs() < 1e-10);
        }

        #[test]
        fn test_insulated_group_creation() {
            let group = InsulatedGroup::new();
            assert!(group.is_empty());
        }

        #[test]
        fn test_insulated_group_add() {
            let mut group = InsulatedGroup::new();
            group.add(InsulatedAgent::new(
                ThermalAgent::new("a", 50.0, 100.0),
                0.5,
            ));
            assert_eq!(group.len(), 1);
        }

        #[test]
        fn test_insulated_group_apply_all() {
            let mut group = InsulatedGroup::new();
            group.add(InsulatedAgent::new(ThermalAgent::new("a", 100.0, 200.0), 0.5));
            group.add(InsulatedAgent::new(ThermalAgent::new("b", 0.0, 200.0), 0.5));
            group.apply_external_all(Temperature::new(50.0));
            // Both should move toward 50.0
            assert!(group.agents()[0].agent().temperature().value() < 100.0);
            assert!(group.agents()[1].agent().temperature().value() > 0.0);
        }

        #[test]
        fn test_insulated_group_avg_temp() {
            let mut group = InsulatedGroup::new();
            group.add(InsulatedAgent::new(ThermalAgent::new("a", 100.0, 200.0), 0.5));
            group.add(InsulatedAgent::new(ThermalAgent::new("b", 0.0, 200.0), 0.5));
            let avg = group.average_temperature();
            assert!((avg.value() - 50.0).abs() < 1e-10);
        }

        #[test]
        fn test_insulated_group_default() {
            let group = InsulatedGroup::default();
            assert!(group.is_empty());
        }
    }
}

pub use temperature::{Temperature, ThermalAgent};
pub use conductor::{Conductor, ConductorNetwork};
pub use equilibrium::EquilibriumResult;
