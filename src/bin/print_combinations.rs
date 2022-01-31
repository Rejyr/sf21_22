fn main() {
    // let mut file = File::create(Path::new("/home/rejyr/Rust Projects/sf21_22/src/run")).unwrap();
    let bots = [
        "RandomBot::new(thread_rng())",
        "AlwaysPushBot::new(thread_rng())",
        "AlwaysCaptureBot::new(thread_rng())",
        "MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristic, thread_rng())",
        "MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng())",
        "MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng())",
        "MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng())",
        "MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng())",
        "MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng())",
    ];
    for bot_l in bots {
        for bot_r in bots {
            println!(
                r#"r!(buf, size, || {}, || {});"#,
                bot_l, bot_r
            );
        }
        println!()
    }

    // for (i, bot) in bots.iter().enumerate() {
    //     println!("fn i{}() -> impl Bot<Board> {{ {} }}", i, bot);
    // }
}
