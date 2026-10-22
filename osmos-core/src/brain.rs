pub const SENSOR_INPUTS: usize = 55;
pub const HIDDEN_NEURONS: usize = 8;
pub const CONTROL_OUTPUTS: usize = 2;
pub const GENOME_LENGTH: usize =
    (SENSOR_INPUTS + 1) * HIDDEN_NEURONS + (HIDDEN_NEURONS + 1) * CONTROL_OUTPUTS;

#[derive(Clone, Debug, PartialEq)]
pub struct Genome {
    values: Vec<f32>,
}

impl Genome {
    pub fn new(values: Vec<f32>) -> Self {
        assert_eq!(values.len(), GENOME_LENGTH, "unexpected genome length");
        Self { values }
    }

    pub fn zeros() -> Self {
        Self {
            values: vec![0.0; GENOME_LENGTH],
        }
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.values
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct ControlNetwork {
    hidden_weights: Vec<f32>,
    hidden_bias: [f32; HIDDEN_NEURONS],
    output_weights: Vec<f32>,
    output_bias: [f32; CONTROL_OUTPUTS],
}

impl ControlNetwork {
    pub fn from_genome(genome: &Genome) -> Self {
        let values = genome.as_slice();
        let mut cursor = 0;
        let mut hidden_bias = [0.0; HIDDEN_NEURONS];
        let mut hidden_weights = Vec::with_capacity(HIDDEN_NEURONS * SENSOR_INPUTS);
        let mut output_bias = [0.0; CONTROL_OUTPUTS];
        let mut output_weights = Vec::with_capacity(CONTROL_OUTPUTS * HIDDEN_NEURONS);

        for bias in &mut hidden_bias {
            *bias = values[cursor];
            cursor += 1;
            hidden_weights.extend_from_slice(&values[cursor..cursor + SENSOR_INPUTS]);
            cursor += SENSOR_INPUTS;
        }

        for bias in &mut output_bias {
            *bias = values[cursor];
            cursor += 1;
            output_weights.extend_from_slice(&values[cursor..cursor + HIDDEN_NEURONS]);
            cursor += HIDDEN_NEURONS;
        }

        assert_eq!(cursor, GENOME_LENGTH);
        Self {
            hidden_weights,
            hidden_bias,
            output_weights,
            output_bias,
        }
    }

    pub fn to_genome(&self) -> Genome {
        let mut values = Vec::with_capacity(GENOME_LENGTH);
        for (neuron, bias) in self.hidden_bias.iter().enumerate() {
            values.push(*bias);
            let start = neuron * SENSOR_INPUTS;
            values.extend_from_slice(&self.hidden_weights[start..start + SENSOR_INPUTS]);
        }
        for (neuron, bias) in self.output_bias.iter().enumerate() {
            values.push(*bias);
            let start = neuron * HIDDEN_NEURONS;
            values.extend_from_slice(&self.output_weights[start..start + HIDDEN_NEURONS]);
        }
        Genome::new(values)
    }

    pub fn feed(&self, input: &[f32; SENSOR_INPUTS]) -> [f32; CONTROL_OUTPUTS] {
        let mut hidden = [0.0_f32; HIDDEN_NEURONS];
        for (neuron, value) in hidden.iter_mut().enumerate() {
            let start = neuron * SENSOR_INPUTS;
            let mut sum = self.hidden_bias[neuron];
            for (index, value) in input.iter().enumerate() {
                sum += value * self.hidden_weights[start + index];
            }
            *value = sum.tanh();
        }

        let mut output = [0.0_f32; CONTROL_OUTPUTS];
        for (neuron, value) in output.iter_mut().enumerate() {
            let start = neuron * HIDDEN_NEURONS;
            let mut sum = self.output_bias[neuron];
            for (index, value) in hidden.iter().enumerate() {
                sum += value * self.output_weights[start + index];
            }
            *value = sum.tanh();
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genome_round_trip_is_stable() {
        let genome = Genome::new(
            (0..GENOME_LENGTH)
                .map(|index| index as f32 / 1000.0)
                .collect(),
        );
        let network = ControlNetwork::from_genome(&genome);
        assert_eq!(network.to_genome(), genome);
    }

    #[test]
    fn outputs_stay_inside_tanh_range() {
        let genome = Genome::new(
            (0..GENOME_LENGTH)
                .map(|index| (index as f32).sin())
                .collect(),
        );
        let network = ControlNetwork::from_genome(&genome);
        let output = network.feed(&[0.25; SENSOR_INPUTS]);
        assert!(output.iter().all(|value| value.is_finite()));
        assert!(output.iter().all(|value| (-1.0..=1.0).contains(value)));
    }

    #[test]
    fn zero_genome_produces_zero_output() {
        let network = ControlNetwork::from_genome(&Genome::zeros());
        assert_eq!(network.feed(&[0.5; SENSOR_INPUTS]), [0.0, 0.0]);
    }
}
