use rand::Rng;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NeuralNetInputNode {
    AlwaysOff,
    AlwaysOn,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NeuralNetOutputNode {
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
}

#[derive(Debug, Clone)]
pub struct NeuralNetConnection {
    pub input_node: NeuralNetInputNode,
    pub output_node: NeuralNetOutputNode,
}

impl NeuralNetConnection {
    pub fn random() -> Self {
        let mut thread_rng = rand::thread_rng();

        let input_node = match thread_rng.gen_range(0..2) {
            0 => NeuralNetInputNode::AlwaysOff,
            1 => NeuralNetInputNode::AlwaysOn,
            _ => unreachable!(),
        };

        let output_node = match thread_rng.gen_range(0..4) {
            0 => NeuralNetOutputNode::MoveNorth,
            1 => NeuralNetOutputNode::MoveEast,
            2 => NeuralNetOutputNode::MoveSouth,
            3 => NeuralNetOutputNode::MoveWest,
            _ => unreachable!(),
        };

        Self {
            input_node,
            output_node,
        }
    }
}
