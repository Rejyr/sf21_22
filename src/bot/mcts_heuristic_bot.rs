use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use board_game::ai::mcts::{IdxRange, Node, SNodeKind, Tree};
use board_game::ai::minimax::Heuristic;
use decorum::N32;
use internal_iterator::{InternalIterator, IteratorExt};
use rand::Rng;

use board_game::ai::Bot;
use board_game::board::{Board, Outcome};
use board_game::wdl::{Flip, OutcomeWDL, POV, WDL};
use rand::prelude::IteratorRandom;

// MCTSbot from board-game, with heuristics
//
fn new_node<M>(last_move: Option<M>, outcome: Option<OutcomeWDL>) -> Node<M> {
    let kind = match outcome {
        None => SNodeKind::Estimate(WDL::default()),
        Some(outcome) => SNodeKind::Solved(outcome),
    };

    Node {
        last_move,
        visits: 0,
        children: None,
        kind,
    }
}

fn random_playout<B: Board>(mut board: B, rng: &mut impl Rng) -> Outcome {
    assert!(
        !board.is_done(),
        "should never start random playout on a done board"
    );

    loop {
        board.play(board.random_available_move(rng));

        if let Some(outcome) = board.outcome() {
            return outcome;
        }
    }
}

fn uct_heuristic<M>(node: &Node<M>, parent_visits: i64, exploration_weight: f32, heuristic_val: f32) -> f32 {
    //TODO continue investigating this, what uct value to use for solved (in practice lost and drawn) nodes?
    // if exploration_weight < 0.0 {
    //     let value_unit = (self.wdl().value() + 1.0) / 2.0;
    //     let explore = ((parent_visits as f32).ln() / self.visits as f32).sqrt();
    //
    //     return value_unit - exploration_weight * explore;
    // }

    match node.kind {
        SNodeKind::Estimate(wdl) => {
            let visits = wdl.sum() as f32;
            let value = wdl.cast::<f32>().value() / visits;
            let value_unit = (value + 1.0) / 2.0;

            let explore = ((parent_visits as f32).ln() / visits).sqrt();

            value_unit + exploration_weight * explore + (heuristic_val / (visits + 1.0))
        }
        SNodeKind::Solved(outcome) => (outcome.sign::<f32>() + 1.0) / 2.0,
    }
}

/// Run a single MCTS step.
///
/// Returns `(result, proven)`, where
/// * `result` is from the pov of the player that just played on `curr_board`.
/// * `proven` is whether this result is fully proven
///
/// This function has already increments `curr_node` before it returns.
fn mcts_solver_step<B: Board>(
    tree: &mut Tree<B>,
    curr_node: usize,
    curr_board: &B,
    exploration_weight: f32,
    heuristic: impl Heuristic<B, V = impl Into<f32>> + Clone,
    rng: &mut impl Rng,
) -> (OutcomeWDL, bool) {
    //TODO should we decrement visit count? -> meh, then we're pulling search time towards partially solved branches
    //TODO should we backprop all previous backpropped losses and draws as wins now? -> meh, then we're overestimating this entire branch

    if let Some(outcome) = tree[curr_node].solution() {
        return (outcome, true);
    }

    // initialize children
    let children = match tree[curr_node].children {
        Some(children) => children,
        None => {
            let start = NonZeroUsize::new(tree.nodes.len()).unwrap();

            //  TODO: can use static eval for move choice, might not have large effect
            curr_board.available_moves().for_each(|mv: B::Move| {
                let next_board = curr_board.clone_and_play(mv);
                let outcome = next_board.outcome().pov(curr_board.next_player());
                let node = new_node(Some(mv), outcome);
                tree.nodes.push(node);
            });

            let length = tree.nodes.len() - start.get();
            let children = IdxRange { start, length };
            tree[curr_node].children = Some(children);

            //TODO maybe do this even earlier, and immediately stop pushing nodes -> but then children are inconsistent :(
            //  so what? who care about children somewhere deep in the tree!
            let outcome =
                OutcomeWDL::best_maybe(children.iter().map(|c| tree[c].solution()).into_internal());
            if let Some(outcome) = outcome.flip() {
                tree[curr_node].mark_solved(outcome);
                return (outcome, true);
            } else {
                children
            }
        }
    };

    // check if there are unvisited children
    let unvisited = children.iter().filter(|&c| tree[c].is_unvisited());
    let picked_unvisited = unvisited.choose(rng);

    // result is from the POV of curr_board.next_player
    let (result, proven) = if let Some(picked_child) = picked_unvisited {
        let picked_mv = tree[picked_child].last_move.unwrap();
        let next_board = curr_board.clone_and_play(picked_mv);

        let outcome = random_playout(next_board, rng).pov(curr_board.next_player().other());
        tree[picked_child].increment(outcome);

        (outcome.flip(), false)
    } else {
        //pick the max-uct child
        //TODO we're including lost and drawn nodes here, is there nothing better we can do?
        // at least this is what the paper seems to suggest
        let parent_visits = tree[curr_node].visits;

        let picked = children
            .iter()
            .max_by_key(|&c| {
                N32::from(uct_heuristic(&tree[c], parent_visits, exploration_weight, heuristic.value(curr_board, 0).into()))
            })
            .unwrap();

        //continue recursing
        let picked_mv = tree[picked].last_move.unwrap();
        let next_board = curr_board.clone_and_play(picked_mv);

        mcts_solver_step(
            tree,
            picked,
            &next_board,
            exploration_weight,
            heuristic,
            rng,
        )
    };

    let result = result.flip();

    if proven {
        //check if we can prove the current node as well
        let outcome =
            OutcomeWDL::best_maybe(children.iter().map(|c| tree[c].solution()).into_internal());
        if let Some(outcome) = outcome.flip() {
            tree[curr_node].mark_solved(outcome);
            return (outcome, true);
        }
    }

    tree[curr_node].increment(result);
    (result, false)
}

pub fn mcts_build_tree<B: Board>(
    root_board: &B,
    iterations: u64,
    exploration_weight: f32,
    heuristic: impl Heuristic<B, V = impl Into<f32>> + Clone,
    rng: &mut impl Rng,
) -> Tree<B> {
    assert!(iterations > 0);

    let mut tree = Tree::new(root_board.clone());

    let root_outcome = root_board
        .outcome()
        .map(|o| o.pov(root_board.next_player().other()));
    tree.nodes.push(new_node(None, root_outcome));

    for _ in 0..iterations {
        //we've solved the root node, so we're done
        if tree[0].solution().is_some() {
            break;
        }

        mcts_solver_step(
            &mut tree,
            0,
            root_board,
            exploration_weight,
            heuristic.clone(),
            rng,
        );
    }

    tree
}

pub struct MCTSBot<B: Board, H: Heuristic<B>, R: Rng> {
    iterations: u64,
    exploration_weight: f32,
    heuristic: H,
    rng: R,
    place_holder: PhantomData<B>,
}

impl<B: Board, H: Heuristic<B>, R: Rng> Debug for MCTSBot<B, H, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MCTSBot {{ iterations: {}, exploration_weight: {} }}",
            self.iterations, self.exploration_weight
        )
    }
}

impl<B: Board, H: Heuristic<B, V = impl Into<f32>> + Clone, R: Rng> MCTSBot<B, H, R> {
    pub fn new(iterations: u64, exploration_weight: f32, heuristic: H, rng: R) -> Self {
        assert!(iterations > 0);
        MCTSBot {
            iterations,
            exploration_weight,
            heuristic,
            rng,
            place_holder: PhantomData,
        }
    }

    pub fn build_tree(&mut self, board: &B) -> Tree<B> {
        mcts_build_tree(
            board,
            self.iterations,
            self.exploration_weight,
            self.heuristic.clone(),
            &mut self.rng,
        )
    }
}

impl<R: Rng, B: Board, H: Heuristic<B, V = impl Into<f32>> + Clone> Bot<B> for MCTSBot<B, H, R> {
    fn select_move(&mut self, board: &B) -> B::Move {
        assert!(!board.is_done());
        self.build_tree(board).best_move()
    }
}
