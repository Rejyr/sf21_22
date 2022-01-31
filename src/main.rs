use std::{fs::File, io::{Write, BufWriter}, path::Path, fmt::Debug};

use board_game::{
    ai::{mcts::MCTSBot, minimax::MiniMaxBot, simple::RandomBot, solver::SolverHeuristic}, util::bot_game::{run, BotGameResult}, wdl::WDL,
};
use rand::thread_rng;
use sf21_22::{
    board::Board,
    bot::{
        heuristic::{AdvancementHeuristic, AlwaysCaptureBot, AlwaysPushBot, MaterialHeuristic},
        mcts_heuristic_bot::MCTSHeuristicBot,
    },
};

fn main() {
    const MIN_MAX_DEPTH: u32 = 10;
    const MCTS_ITERATIONS: u64 = 10_000;
    const MCTS_EXPLORATION: f32 = 2.0;

    const GAMES_PER_SIDE: u32 = 500;
    const BOTH_SIDES: bool = true;

    let file = File::create(Path::new("/home/rejyr/Rust Projects/sf21_22/output")).unwrap();
    let mut buf = BufWriter::new(file);
    let board_size = [3, 4, 5, 6, 7, 8];
    // let board_size = [3, 4];
    for size in board_size {
        println!("size: {size}");
        buf.write_fmt(format_args!("size: {size}\n\n\n")).unwrap();


        buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || RandomBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysPushBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || AlwaysCaptureBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();

buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || RandomBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();
buf.write_fmt(format_args!("{:?}", x(run(|| Board::new(size), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), GAMES_PER_SIDE, BOTH_SIDES, |_, _| {})))).unwrap();


    }
    buf.flush().unwrap()
}

pub struct BotResult {
    wdl_l: WDL<u32>,
    debug_l: String,
    debug_r: String,
    time_l: f32,
    time_r: f32,
}

impl Debug for BotResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "L: {} W{}|D{}|L{} R: {}", self.debug_l, self.wdl_l.win, self.wdl_l.draw, self.wdl_l.loss, self.debug_r)
        writeln!(f, "L: {} (t: {:.4}) | W:{},D:{},L:{} | R: {} (t: {:.4})", self.debug_l, self.time_l, self.wdl_l.win, self.wdl_l.draw, self.wdl_l.loss, self.debug_r, self.time_r)
    }
}
    pub fn x(result: BotGameResult<Board>) -> BotResult {
        BotResult {
            wdl_l: result.wdl_l,
            debug_l: result.debug_l,
            debug_r: result.debug_r,
            time_l: result.time_l,
            time_r: result.time_r,
        }
    }
