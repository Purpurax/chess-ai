use core::f64;
use std::{sync::{Arc, Mutex}, thread};
use good_web_game::timer;
use rand::seq::IteratorRandom;

use crate::{agent::random, core::{board::Board, game::Game, move_generator::get_all_possible_moves, piece::PieceType, position::Position}};

const EXPLORATION_C: f64 = 0.7;
const SIMULATION_DEPTH_LIMIT: usize = 25;
const THREAD_COUNT: usize = 6;
const EPSILON_SIMULATION: f64 = 0.5;

#[derive(Clone)]
pub struct Tree {
    tree_state: Arc<Mutex<TreeState>>
}

pub struct TreeState {
    pub nodes: Vec<Node>,
    pub color: bool
}

impl Tree {
    pub fn blank() -> Tree {
        Tree {
            tree_state: Arc::new(Mutex::new(TreeState {
                nodes: vec![],
                color: false
            }))
        }
    }

    pub fn new() -> Tree {
        let root_node: Node = Node::new_root();
        Tree {
            tree_state: Arc::new(Mutex::new(TreeState {
                nodes: vec![root_node],
                color: false
            }))
        }
    }

    pub fn walk_edge_permanently(&mut self, from_pos: &Position, to_pos: &Position) {
        let index_root_self_op: Option<usize> = self.get_root_node().children
            .into_iter()
            .find(|child| {
                self.tree_state.lock().unwrap().nodes.get(*child).unwrap().edge_to_this_node
                == Some((from_pos.clone(), to_pos.clone()))
            });
        
        if let Some(index_root_self) = index_root_self_op {
            let mut tree: Tree = Tree::blank();
            self.copy_rec_nodes(&mut tree, index_root_self, None);
            *self = tree;
        } else {
            *self = Tree::new();
        }
    }

    fn copy_rec_nodes(&mut self, tree: &mut Tree, index_self: usize, parent_tree_index: Option<usize>) {
        let new_index: usize = tree.tree_state.lock().unwrap().nodes.len();

        let mut node: Node = self.get_node(index_self);
        node.parent = parent_tree_index;
        
        if let Some(parent_index) = parent_tree_index {
            let mutex = &mut tree.tree_state.lock().unwrap();
            let parent: &mut Node = mutex.nodes.get_mut(parent_index).unwrap();
            parent.children.iter_mut()
                .for_each(|parents_child| {
                    if *parents_child == index_self {
                        *parents_child = new_index;
                    }
                })
        }

        let nodes_children: Vec<usize> = node.children.clone();
        tree.tree_state.lock().unwrap().nodes.push(node);

        nodes_children.into_iter()
            .for_each(|child| {
                self.copy_rec_nodes(tree, child, Some(new_index));
            });
    }

    pub fn refresh(&mut self, player_turn: bool) {
        self.tree_state.lock().unwrap().nodes = vec![Node::new_root()];
        self.tree_state.lock().unwrap().color = player_turn;
    }

    pub fn get_node(&self, index: usize) -> Node {
        self.tree_state.lock().unwrap().nodes.get(index).unwrap().clone()
    }
    
    pub fn get_node_children(&self, index: usize) -> Vec<usize> {
        self.tree_state.lock().unwrap().nodes.get(index).unwrap().children.clone()
    }

    pub fn get_node_parent(&self, index: usize) -> Option<usize> {
        self.tree_state.lock().unwrap().nodes.get(index).unwrap().parent
    }

    pub fn get_node_termination_bool(&self, index: usize) -> bool {
        self.tree_state.lock().unwrap().nodes.get(index).unwrap().termination_node
    }

    pub fn get_node_edge(&self, index: usize) -> Option<(Position, Position)> {
        self.tree_state.lock().unwrap().nodes.get(index).unwrap().edge_to_this_node.clone()
    }

    pub fn modfiy_node(&mut self, index: usize, update_fn: impl FnOnce(&mut Node)) {
        update_fn(self.tree_state.lock().unwrap().nodes.get_mut(index).unwrap())
    }

    pub fn get_root_node(&self) -> Node {
        self.get_node(0)
    }
    
    pub fn add_new_node(&mut self, new_node: Node) -> usize {
        let node = &mut self.tree_state.lock().unwrap().nodes;
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
    if initial_game.player_turn != tree.tree_state.lock().unwrap().color || tree.get_root_node().children.is_empty() {
        tree.refresh(initial_game.player_turn);
    }

    let time_for_stop: f64 = timer::time() + max_compute_time;

    let mut handles = vec![];
    
    for _ in 0..THREAD_COUNT {
        let game_clone = initial_game.clone();
        let time_stop = time_for_stop;
        let mut tree_clone = tree.clone();
        
        handles.push(thread::spawn(move || {
            monte_carlo_iteration(game_clone, &mut tree_clone, time_stop)
        }));
    }
    
    for handle in handles {
        let _ = handle.join();
    }

    let children_to_choose: &Vec<usize> = &tree.get_root_node().children;
    let greedy_selection: usize = greedy_selection(tree, 0);
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
    let playing_for: bool = tree.tree_state.lock().unwrap().color;

    while timer::time() < time_for_stop {
        let mut node_index: usize = 0;
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
            let new_children: Vec<usize> = get_all_possible_moves(&simulation_game.board, simulation_game.player_turn)
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
        let (from_pos, to_pos) = get_heuristic_random_turn(&game);
        game.perform_move(&from_pos, &to_pos);
        current_step += 1;
    }
    
    evaluate_simulation(&game, playing_for)
}

fn get_heuristic_random_turn(game: &Game) -> (Position, Position) {
    if rand::random::<f64>() < EPSILON_SIMULATION {
        random::get_turn(game)
    } else {
        get_all_possible_moves(&game.board, game.player_turn)
            .into_iter()
            .filter(|(_, to_pos)|
                game.board.get_piece_at(to_pos).piece_type() != PieceType::Empty
            ).choose(&mut rand::rng())
            .unwrap_or(random::get_turn(game))
    }
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
            let score1: f64 = ucb_score(child1, root_node_total_visits);
            let score2: f64 = ucb_score(child2, root_node_total_visits);

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

// mod tests {
//     use super::*;

//     #[test]
//     fn test_walk_edge_permanently() {
//         // Create a tree with a known structure
//         let mut tree = Tree::new();
        
//         // First, build a small tree with depth 3
//         // Root (0) has 3 children: A (1), B (2), C (3)
//         // A has 2 children: D (4), E (5)
//         // B has 1 child: F (6)
//         // D has 1 child: G (7)
        
//         // Add the first level
//         let a_node = Node::new((Position::new(0, 0), Position::new(1, 0)), 0);
//         let b_node = Node::new((Position::new(0, 1), Position::new(1, 1)), 0);
//         let c_node = Node::new((Position::new(0, 2), Position::new(1, 2)), 0);
        
//         let a_index = tree.add_new_node(a_node);
//         let b_index = tree.add_new_node(b_node);
//         let c_index = tree.add_new_node(c_node);
        
//         // Update root's children
//         tree.modfiy_node(0, |node| {
//             node.children = vec![a_index, b_index, c_index];
//         });
        
//         // Add the second level
//         let d_node = Node::new((Position::new(1, 0), Position::new(2, 0)), a_index);
//         let e_node = Node::new((Position::new(1, 1), Position::new(2, 1)), a_index);
//         let f_node = Node::new((Position::new(1, 2), Position::new(2, 2)), b_index);
        
//         let d_index = tree.add_new_node(d_node);
//         let e_index = tree.add_new_node(e_node);
//         let f_index = tree.add_new_node(f_node);
        
//         // Update A and B's children
//         tree.modfiy_node(a_index, |node| {
//             node.children = vec![d_index, e_index];
//         });
        
//         tree.modfiy_node(b_index, |node| {
//             node.children = vec![f_index];
//         });
        
//         // Add the third level
//         let g_node = Node::new((Position::new(2, 0), Position::new(3, 0)), d_index);
//         let g_index = tree.add_new_node(g_node);
        
//         // Update D's children
//         tree.modfiy_node(d_index, |node| {
//             node.children = vec![g_index];
//         });
        
//         // Print initial tree state
//         println!("Initial tree structure:");
//         print_tree_structure(&tree);
        
//         // First walk: Select B's edge to keep
//         println!("\nWalking edge B (0,1)->(1,1)");
//         let from_pos_b = Position::new(0, 1);
//         let to_pos_b = Position::new(1, 1);
//         tree.walk_edge_permanently(&from_pos_b, &to_pos_b);
        
//         // Print tree state after first walk
//         println!("\nTree structure after first walk:");
//         print_tree_structure(&tree);
        
//         // Second walk: Select F's edge to keep
//         println!("\nWalking edge F (1,2)->(2,2)");
//         let from_pos_f = Position::new(1, 2);
//         let to_pos_f = Position::new(2, 2);
//         tree.walk_edge_permanently(&from_pos_f, &to_pos_f);
        
//         // Print final tree state
//         println!("\nFinal tree structure:");
//         print_tree_structure(&tree);
        
//         // Verify the structure is as expected
//         assert_eq!(tree.tree_state.lock().unwrap().nodes.len(), 2, 
//             "Tree should have 2 nodes: root and F");
            
//         // Check that the remaining child has the correct edge (F)
//         let root_node = tree.get_root_node();
//         assert_eq!(root_node.children.len(), 1, "Root should have 1 child");
//         let f_idx = root_node.children[0];
//         let f_node = tree.get_node(f_idx);
//         assert_eq!(f_node.edge_to_this_node, Some((Position::new(1, 2), Position::new(2, 2))),
//             "Remaining node should have F's edge");
//     }

//     // Helper function to print the tree structure for debugging
//     fn print_tree_structure(tree: &Tree) {
//         let nodes = tree.tree_state.lock().unwrap().nodes.clone();
//         println!("Tree has {} nodes", nodes.len());
        
//         for (i, node) in nodes.iter().enumerate() {
//             let edge_str = match &node.edge_to_this_node {
//                 Some((from, to)) => format!("({},{}) -> ({},{})", from.row, from.column, to.row, to.column),
//                 None => "ROOT".to_string()
//             };
            
//             let parent_str = match node.parent {
//                 Some(p) => format!("{}", p),
//                 None => "None".to_string()
//             };
            
//             println!("Node {}: Edge: {}, Parent: {}, Children: {:?}, Visits: {}, Score: {}",
//                 i, edge_str, parent_str, node.children, node.total_visits, node.score);
//         }
//     }
// }
