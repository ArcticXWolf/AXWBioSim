use rand::{seq::SliceRandom, Rng};

use crate::{
    agent_neural_net::{NeuralNetConnection, NeuralNetInputNode, NeuralNetOutputNode},
    world::WorldSettings,
};

pub enum MoveIntention {
    North,
    East,
    South,
    West,
}

impl MoveIntention {
    pub fn from_neural_net_outputs(nodes: &[NeuralNetOutputNode]) -> Option<Self> {
        for node in nodes {
            match node {
                NeuralNetOutputNode::MoveNorth => return Some(Self::North),
                NeuralNetOutputNode::MoveEast => return Some(Self::East),
                NeuralNetOutputNode::MoveSouth => return Some(Self::South),
                NeuralNetOutputNode::MoveWest => return Some(Self::West),
                _ => {}
            }
        }
        None
    }
}

pub struct AgentIntention {
    pub move_intention: Option<MoveIntention>,
}

#[derive(Debug, Default, Clone)]
pub struct Agent {
    pub neural_net: Vec<NeuralNetConnection>,
    pub x: usize,
    pub y: usize,
}

impl Agent {
    pub fn new(neural_net: Vec<NeuralNetConnection>, x: usize, y: usize) -> Self {
        Self { neural_net, x, y }
    }

    pub fn spawn(world_settings: &WorldSettings) -> Self {
        let mut thread_rng = rand::thread_rng();
        Self {
            neural_net: vec![NeuralNetConnection::random()],
            x: thread_rng.gen_range(0..world_settings.width),
            y: thread_rng.gen_range(0..world_settings.height),
        }
    }

    pub fn spawn_from_parents(world_settings: &WorldSettings, parents: &[Self]) -> Self {
        let mut thread_rng = rand::thread_rng();
        let neural_net = parents.choose(&mut thread_rng).unwrap().neural_net.clone();
        Self {
            neural_net,
            x: thread_rng.gen_range(0..world_settings.width),
            y: thread_rng.gen_range(0..world_settings.height),
        }
    }

    fn get_neural_net_input_node_state(&self, node: NeuralNetInputNode) -> bool {
        match node {
            NeuralNetInputNode::AlwaysOff => false,
            NeuralNetInputNode::AlwaysOn => true,
        }
    }

    pub fn process_neural_net(&mut self) -> AgentIntention {
        let mut activated_output_nodes = vec![];

        for connection in &self.neural_net {
            if self.get_neural_net_input_node_state(connection.input_node) {
                activated_output_nodes.push(connection.output_node);
            }
        }

        AgentIntention {
            move_intention: MoveIntention::from_neural_net_outputs(&activated_output_nodes),
        }
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn color(&self) -> [u8; 3] {
        [0xFF, 0xFF, 0xFF]
    }
}
