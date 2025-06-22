// use core::f64;

// use crate::{agent::random, core::{game::Game, move_generator::{get_all_possible_moves, get_possible_moves}, position::Position}};

// pub struct Node {
//     pub edge_to_this_node: Option<(Position, Position)>,
//     pub parent: Box<Option<Node>>,
//     pub children: Vec<Node>,
//     pub termination_node: bool,
//     pub total_visits: usize,
//     pub wins: usize
// }

// impl Node {
//     pub fn new(edge: (Position, Position), parent: Node) -> Node {
//         Node {
//             edge_to_this_node: Some(edge),
//             parent: Box::new(Some(parent)),
//             children: vec![],
//             termination_node: false,
//             total_visits: 1,
//             wins: 0
//         }
//     }
// }

// const EPSILON: f64 = 0.2;

// pub fn get_turn(mut game: &Game, root_node: &mut Node, max_compute_time: f64) -> (Position, Position) {
//     let node: &mut Node = root_node;

//     // choose a node to start evaluation
//     while node.children.is_empty() && !node.termination_node {
//         if random::get_random_float() < EPSILON {
//             node = node.children.get_mut(random_selection(&node.children)).unwrap();
//         } else {
//             node = node.children.get_mut(greedy_selection(&node.children)).unwrap();
//         }

//         game.perform_move(&node.edge_to_this_node.unwrap().0, &node.edge_to_this_node.unwrap().1);
//     }

//     if !node.termination_node {
//         node.children = get_all_possible_moves(&game.board, game.player_turn, true)
//             .into_iter()
//             .map(|edge| {
//                 Node::new(edge, node)
//             }).collect::<Vec<
//     }

//     // define a depth, where you stop and clearly define win or not

//     // backpropagate to the wins score

//     None
// }

// fn random_selection(children: &Vec<Node>) -> usize {
//     random::get_random_int(children.len())
// }

// fn greedy_selection(children: &Vec<Node>) -> usize {
//     let mut best_score: f64 = f64::MIN;
//     let mut best_index: usize = 0;

//     for (index, child) in children.iter().enumerate() {
//         let score: f64 = child.wins as f64 / child.total_visits as f64;

//         if score > best_score {
//             best_score = score;
//             best_index = index;
//         }
//     }

//     best_index
// }
