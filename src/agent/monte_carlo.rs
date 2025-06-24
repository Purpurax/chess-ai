use core::f64;
use std::{sync::{Arc, Mutex}, thread};
use good_web_game::timer;

use crate::{agent::random, core::{board::Board, game::Game, move_generator::get_all_possible_moves, position::Position}};

const EXPLORATION_C: f64 = 0.7;
const SIMULATION_DEPTH_LIMIT: usize = 20;
const THREAD_COUNT: usize = 6;

#[derive(Clone)]
pub struct Tree {
    pub root_node_index: usize,
    pub nodes: Arc<Mutex<Vec<Node>>>,
    pub color: bool
}

impl Tree {
    pub fn new() -> Tree {
        let root_node: Node = Node::new_root();
        Tree {
            root_node_index: 0,
            nodes: Arc::new(Mutex::new(vec![root_node])),
            color: false
        }
    }

    pub fn refresh(&mut self, player_turn: bool) {
        self.root_node_index = 0;
        self.nodes = Arc::new(Mutex::new(vec![Node::new_root()]));
        self.color = player_turn;
    }

    pub fn get_node(&self, index: usize) -> Node {
        self.nodes.lock().unwrap().get(index).unwrap().clone()
    }

    pub fn get_node_children(&self, index: usize) -> Vec<usize> {
        self.nodes.lock().unwrap().get(index).unwrap().children.clone()
    }

    pub fn get_node_parent(&self, index: usize) -> Option<usize> {
        self.nodes.lock().unwrap().get(index).unwrap().parent
    }

    pub fn get_node_termination_bool(&self, index: usize) -> bool {
        self.nodes.lock().unwrap().get(index).unwrap().termination_node
    }

    pub fn get_node_edge(&self, index: usize) -> Option<(Position, Position)> {
        self.nodes.lock().unwrap().get(index).unwrap().edge_to_this_node.clone()
    }

    pub fn modfiy_node(&mut self, index: usize, update_fn: impl FnOnce(&mut Node)) {
        update_fn(self.nodes.lock().unwrap().get_mut(index).unwrap())
    }

    pub fn get_root_node(&self) -> Node {
        self.get_node(self.root_node_index)
    }
    
    pub fn add_new_node(&mut self, new_node: Node) -> usize {
        let node = &mut self.nodes.lock().unwrap();
        node.push(new_node);
        node.len() - 1
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
        tree.refresh(initial_game.player_turn);
    }
    let time_for_stop: f64 = timer::time() + max_compute_time;

    let tree_arc = Arc::clone(&tree.nodes);
    let mut handles = vec![];
    
    for _ in 0..THREAD_COUNT {
        let game_clone = initial_game.clone();
        let mut tree_clone = Tree {
            root_node_index: tree.root_node_index,
            nodes: Arc::clone(&tree_arc),
            color: tree.color
        };
        let time_stop = time_for_stop;
        
        handles.push(thread::spawn(move || {
            monte_carlo_iteration(game_clone, &mut tree_clone, time_stop)
        }));
    }
    
    for handle in handles {
        let _ = handle.join();
    }

    let children_to_choose: &Vec<usize> = &tree.get_root_node().children;
    let greedy_selection: usize = greedy_selection(tree, tree.root_node_index);
    let greedy_node_index_in_tree: usize = *children_to_choose.get(greedy_selection).unwrap();
    let greedy_node: Node = tree.get_node(greedy_node_index_in_tree);

    println!("\nMonte-Carlo:\n > Execution time {:.3?}\n > best score {}\n > nodes simulated: {}",
        timer::time() - time_for_stop + max_compute_time,
        greedy_score(&greedy_node),
        tree.get_root_node().total_visits
    );
    greedy_node.edge_to_this_node.clone().unwrap()
}

fn monte_carlo_iteration(initial_game: Game, tree: &mut Tree, time_for_stop: f64) {
    let playing_for: bool = tree.color;

    while timer::time() < time_for_stop {
        let mut node_index: usize = tree.root_node_index;
        let mut simulation_game: Game = initial_game.clone();

        /* Selection */
        while !tree.get_node_children(node_index).is_empty()
        && !tree.get_node_termination_bool(node_index) {
            node_index = *tree.get_node_children(node_index).get(ucb_selection(tree, node_index)).unwrap();

            let edge: (Position, Position) = tree.get_node_edge(node_index).unwrap();
            simulation_game.perform_move(
                &edge.0,
                &edge.1
            );
        }

        /* Expansion */
        if simulation_game.get_winner().is_some() {
            tree.modfiy_node(node_index, |node| node.termination_node = true);
        }

        if !tree.get_node_termination_bool(node_index) {
            let new_children: Vec<usize> = get_all_possible_moves(&simulation_game.board, simulation_game.player_turn, true)
                .into_iter()
                .map(|edge| {
                    let new_node: Node = Node::new(edge, node_index);
                    tree.add_new_node(new_node)
                }).collect::<Vec<usize>>();
            tree.modfiy_node(node_index, |node| node.children = new_children)
        }

        /* Simulation */
        let reward: f64 = simulation(simulation_game, playing_for);

        /* Backpropagation */
        let mut propagation_node: Option<usize> = Some(node_index);
        
        while let Some(propagation_node_index) = propagation_node {
            tree.modfiy_node(propagation_node_index, |node| {
                node.score += reward;
                node.total_visits += 1;
            });
            propagation_node = tree.get_node_parent(propagation_node_index);
        }
    }
}

fn simulation(mut game: Game, playing_for: bool) -> f64 {
    let mut current_step: usize = 0;
    
    while current_step < SIMULATION_DEPTH_LIMIT && game.get_winner().is_none() {
        let (from_pos, to_pos) = random::get_turn(&game);
        game.perform_move(&from_pos, &to_pos);
        current_step += 1;
    }
    
    evaluate_simulation(&game, playing_for)
}

fn ucb_score(node: &Node, total_tree_visits: usize) -> f64 {
    node.score / node.total_visits as f64
    + EXPLORATION_C * ((total_tree_visits as f64).ln() / node.total_visits as f64).sqrt()
}

fn greedy_score(node: &Node) -> f64 {
    node.score / node.total_visits as f64
}

fn ucb_selection(tree: &Tree, index_of_parent: usize) -> usize {
    let root_node_total_visits: usize = tree.get_root_node().total_visits;
    tree.get_node(index_of_parent).children
        .iter()
        .map(|child: &usize| {
            tree.get_node(*child)
        }).enumerate()
        .max_by(|(_, child1), (_, child2)| {
            let score1: f64 = ucb_score(&child1, root_node_total_visits);
            let score2: f64 = ucb_score(&child2, root_node_total_visits);

            score1.partial_cmp(&score2).unwrap_or(std::cmp::Ordering::Equal)
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
        .map(|(index, child)| (index, greedy_score(&child)))
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

    let center_squares: u64 = 0x0000001818000000; // Center 4 squares
    let center_control = (center_squares & playing_color_layer).count_ones() as f64 * 0.2;
    score += center_control;

    score / 40.0
}
