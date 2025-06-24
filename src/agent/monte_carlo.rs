use core::f64;
use good_web_game::timer;
use rand::seq::IndexedRandom;

use crate::{agent::random, core::{board::Board, game::Game, move_generator::get_all_possible_moves, position::Position}};

const EXPLORATION_C: f64 = 1.5;

#[derive(Clone)]
pub struct Tree {
    pub root_node_index: usize,
    pub nodes: Vec<Node>,
    pub color: bool
}

impl Tree {
    pub fn new() -> Tree {
        let root_node: Node = Node::new_root();
        Tree {
            root_node_index: 0,
            nodes: vec![root_node],
            color: false
        }
    }

    pub fn get_node(&self, index: usize) -> &Node {
        self.nodes.get(index).unwrap()
    }

    pub fn get_node_mut(&mut self, index: usize) -> &mut Node {
        self.nodes.get_mut(index).unwrap()
    }

    pub fn get_root_node(&self) -> &Node {
        self.get_node(self.root_node_index)
    }
    
    pub fn add_new_node(&mut self, new_node: Node) -> usize {
        self.nodes.push(new_node);
        self.nodes.len() - 1
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub edge_to_this_node: Option<(Position, Position)>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub termination_node: bool,
    pub total_visits: usize,
    pub score: f64
}

impl Node {
    pub fn new(edge: (Position, Position), parent_index: usize) -> Node {
        Node {
            edge_to_this_node: Some(edge),
            parent: Some(parent_index),
            children: vec![],
            termination_node: false,
            total_visits: 1,
            score: 0.0
        }
    }

    pub fn new_root() -> Node {
        Node {
            edge_to_this_node: None,
            parent: None,
            children: vec![],
            termination_node: false,
            total_visits: 1,
            score: 0.0
        }
    }
}

pub fn get_turn(initial_game: &Game, tree: &mut Tree, max_compute_time: f64) -> (Position, Position) {
    if initial_game.player_turn != tree.color || tree.get_root_node().children.is_empty() {
        tree.root_node_index = 0;
        tree.nodes = vec![Node::new_root()];
        tree.color = initial_game.player_turn;
    }

    let playing_for: bool = tree.color;
    let time_for_stop: f64 = timer::time() + max_compute_time;

    while timer::time() < time_for_stop {
        let mut node_index: usize = tree.root_node_index;
        let mut simulation: Game = initial_game.clone();

        /* Selection */
        while !tree.get_node(node_index).children.is_empty() && !tree.get_node(node_index).termination_node {
            node_index = *tree.get_node(node_index).children.get(ucb_selection(tree, node_index)).unwrap();

            simulation.perform_move(
                &tree.get_node(node_index).edge_to_this_node.clone().unwrap().0,
                &tree.get_node(node_index).edge_to_this_node.clone().unwrap().1
            );
        }

        /* Expansion */
        if simulation.get_winner().is_some() {
            tree.get_node_mut(node_index).termination_node = true;
        }

        if !tree.get_node(node_index).termination_node {
            tree.get_node_mut(node_index).children = get_all_possible_moves(&simulation.board, simulation.player_turn, true)
                .into_iter()
                .map(|edge| {
                    let new_node: Node = Node::new(edge, node_index);
                    tree.add_new_node(new_node)
                }).collect::<Vec<usize>>()
        }

        /* Simulation */
        const MAX_STEPS: usize = 35;
        let mut current_step: usize = 0;
        
        while current_step < MAX_STEPS && simulation.get_winner().is_none() {
            let (from_pos, to_pos) = random::get_turn(&simulation);
            simulation.perform_move(&from_pos, &to_pos);
            current_step += 1;
        }

        /* Backpropagation */
        let reward: f64 = evaluate_simulation(&simulation, playing_for);
        let mut propagation_node: Option<usize> = Some(node_index);
        
        while let Some(propagation_node_index) = propagation_node {
            tree.get_node_mut(propagation_node_index).score += reward;
            tree.get_node_mut(propagation_node_index).total_visits += 1;
            propagation_node = tree.get_node(propagation_node_index).parent;
        }
    }

    let children_to_choose: &Vec<usize> = &tree.get_root_node().children;
    let greedy_selection: usize = greedy_selection(tree, tree.root_node_index);
    let greedy_node_index_in_tree: usize = *children_to_choose.get(greedy_selection).unwrap();
    let greedy_node: &Node = tree.get_node(greedy_node_index_in_tree);

    println!("\nMonte-Carlo:\n > Execution time {:.3?}\n > best score {}\n > nodes simulated: {}",
        timer::time() - time_for_stop + max_compute_time,
        greedy_score(greedy_node),
        tree.get_root_node().total_visits
    );
    greedy_node.edge_to_this_node.clone().unwrap()
}

fn ucb_score(node: &Node, total_tree_visits: usize) -> f64 {
    node.score / node.total_visits as f64
    + EXPLORATION_C * ((total_tree_visits as f64).ln() / node.total_visits as f64).sqrt()
}

fn greedy_score(node: &Node) -> f64 {
    node.score / node.total_visits as f64
}

fn ucb_selection(tree: &Tree, index_of_parent: usize) -> usize {
    tree.get_node(index_of_parent).children
        .iter()
        .map(|child: &usize| {
            tree.get_node(*child)
        }).enumerate()
        .map(|(index, child)| (index, ucb_score(child, tree.get_root_node().total_visits)))
        .max_by(|(_, score1), (_, score2)| {
            score1.partial_cmp(score2).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn greedy_selection(tree: &Tree, index_of_parent: usize) -> usize {
    tree.get_node(index_of_parent).children
        .iter()
        .map(|child: &usize| {
            tree.get_node(*child)
        }).enumerate()
        .map(|(index, child)| (index, greedy_score(child)))
        .max_by(|(_, score1), (_, score2)| {
            score1.partial_cmp(score2).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn evaluate_simulation(game: &Game, playing_for: bool) -> f64 {
    match game.get_winner() {
        Some(2) => 0.0,
        Some(1) if playing_for => 1.0,
        Some(0) if !playing_for => 1.0,
        Some(1) => -1.0,
        Some(0) => -1.0,
        _ => evalutate_game(&game.board, playing_for)
    }
}

fn evalutate_game(board: &Board, playing_for: bool) -> f64 {
    let mut score: f64 = 0.0;
    let playing_color_layer: u64 = if playing_for {
        board.layer_color
    } else {
        !board.layer_color
    };

    score += (board.layer_pawn & playing_color_layer).count_ones() as f64;
    score += (board.layer_knight & playing_color_layer).count_ones() as f64 * 3.0;
    score += (board.layer_bishop & playing_color_layer).count_ones() as f64 * 3.0;
    score += (board.layer_rook & playing_color_layer).count_ones() as f64 * 5.0;
    score += (board.layer_queen & playing_color_layer).count_ones() as f64 * 7.0;

    score -= (board.layer_pawn & !playing_color_layer).count_ones() as f64;
    score -= (board.layer_knight & !playing_color_layer).count_ones() as f64 * 3.0;
    score -= (board.layer_bishop & !playing_color_layer).count_ones() as f64 * 3.0;
    score -= (board.layer_rook & !playing_color_layer).count_ones() as f64 * 5.0;
    score -= (board.layer_queen & !playing_color_layer).count_ones() as f64 * 7.0;

    score / 37.0
}
