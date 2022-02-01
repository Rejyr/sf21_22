fn main() {
    // let mut file = File::create(Path::new("/home/rejyr/Rust Projects/sf21_22/src/run")).unwrap();
    let bots = [
        "RandomBot::new(thread_rng())",
        "AlwaysPushBot::new(thread_rng())",
        "AlwaysCaptureBot::new(thread_rng())",
        "MiniMaxBot::new(MIN_MAX_DEPTH, SolverHeuristicSimplified, thread_rng())",
        "MiniMaxBot::new(MIN_MAX_DEPTH, MaterialHeuristic, thread_rng())",
        "MiniMaxBot::new(MIN_MAX_DEPTH, AdvancementHeuristic, thread_rng())",
        "MCTSBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, thread_rng())",
        "MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, SolverHeuristicSimplified, thread_rng())",
        "MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, MaterialHeuristic, thread_rng())",
        "MCTSHeuristicBot::new(MCTS_ITERATIONS, MCTS_EXPLORATION, AdvancementHeuristic, thread_rng())",
    ];
    let bot_names = [
        "Random",
        "AlwaysPush",
        "AlwaysCapture",
        "MiniMax",
        "MiniMaxAdvancement",
        "MiniMaxMaterial",
        "MCTS",
        "MCTSSolver",
        "MCTSAdvancement",
        "MCTSMaterial",
    ];
    for (bot_l, name) in bots.into_iter().zip(bot_names.into_iter()) {
        println!(r#"println!("Running (at {{}}): {}", OffsetDateTime::now_utc());"#, name);
        for bot_r in bots {
            println!(
                r#"r!(buf, size, || {}, || {});"#,
                bot_l, bot_r
            );
        }
        println!()
    }
}
