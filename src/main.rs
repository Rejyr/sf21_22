use std::{fs::File, io::{Write, BufWriter}, fmt::Debug};

use board_game::{
    ai::{minimax::MiniMaxBot, simple::RandomBot, mcts::MCTSBot}, util::bot_game::{run, BotGameResult}, wdl::WDL,
};
use rand::thread_rng;
use time::OffsetDateTime;
use sf21_22::{
    board::Board,
    bot::{
        heuristic::{AdvancementHeuristic, AlwaysCaptureBot, AlwaysPushBot, MaterialHeuristic, SolverHeuristicSimplified},
        mcts_heuristic_bot::MCTSHeuristicBot,
    }, SIZES, output_path,
};

fn main() {
    const MIN_MAX_DEPTH: u32 = 10;
    const MCTS_ITERATIONS: u64 = 10_000;
    const MCTS_EXPLORATION: f32 = 2.0;

    const TRIALS_PER: u32 = 1000;
    const GAMES_PER_SIDE: u32 = TRIALS_PER / 4;
    const BOTH_SIDES: bool = true;

    let file = File::create(output_path()).unwrap();
    let mut buf = BufWriter::new(file);
    for size in SIZES {
        println!("size: {size}");
        buf.write_fmt(format_args!("\n\nsize: {size}\n\n")).unwrap();

println!("Running (at {}): Random", OffsetDateTime::now_utc());
r!(buf, size, || RandomBot::new(thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || RandomBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): AlwaysPush", OffsetDateTime::now_utc());
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || AlwaysPushBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): AlwaysCapture", OffsetDateTime::now_utc());
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || AlwaysCaptureBot::new(thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): MiniMax", OffsetDateTime::now_utc());
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): MiniMaxAdvancement", OffsetDateTime::now_utc());
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): MiniMaxMaterial", OffsetDateTime::now_utc());
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): MCTS", OffsetDateTime::now_utc());
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): MCTSSolver", OffsetDateTime::now_utc());
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): MCTSAdvancement", OffsetDateTime::now_utc());
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));

println!("Running (at {}): MCTSMaterial", OffsetDateTime::now_utc());
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || RandomBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || AlwaysPushBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || AlwaysCaptureBot::new(thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng()));
r!(buf, size, || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()), || MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng()));



    }
    buf.flush().unwrap()
}

#[macro_export]
macro_rules! r {
    ($buf:expr, $size:expr, $l:expr, $r:expr) => {
        $buf.write_fmt(format_args!("{:?}", x(run(|| Board::new($size), $l, $r, GAMES_PER_SIDE, BOTH_SIDES, |_,_| {})))).unwrap();
    };
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
