use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Result},
    ops::{Add, Neg},
    str::FromStr,
};

use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use sf21_22::{output_path, SIZES};

use self::BotType::*;

fn main() {
    let file = File::open(output_path()).unwrap();
    let read = BufReader::new(&file);

    let results = parse_results(read).unwrap();

    graph_data(&results);
}

pub fn graph_data(results: &Results) {
    let bot_types = [
        Random,
        AlwaysPush,
        AlwaysCapture,
        MiniMax,
        MiniMaxAdvance,
        MiniMaxCapture,
        MCTS,
        MCTSSolver,
        MCTSAdvance,
        MCTSCapture,
    ];

    print!("{:<14}: ", "size");
    for size in SIZES {
        print!("{:^9}|", size);
    }
    println!();
    for bot_type in bot_types {
        print!("{:<14}: ", format!("{:?}", bot_type));
        for size in SIZES {
            print!(
                "{:^9}|",
                results
                    .get_cumulative(size as u32, bot_type)
                    .unwrap()
                    .combined()
            );
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BotType {
    Random,
    AlwaysPush,
    AlwaysCapture,
    MiniMax,
    MiniMaxAdvance,
    MiniMaxCapture,
    MCTS,
    MCTSSolver,
    MCTSAdvance,
    MCTSCapture,
}

impl FromStr for BotType {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
"RandomBot" => Random,
"AlwaysPushBot" => AlwaysPush,
"AlwaysCaptureBot" => AlwaysCapture,
"MiniMaxBot { depth: 10, heuristic: SolverHeuristicSimplified }" => MiniMax,
"MiniMaxBot { depth: 10, heuristic: MaterialHeuristic }" => MiniMaxCapture,
"MiniMaxBot { depth: 10, heuristic: AdvancementHeuristic }" => MiniMaxAdvance,
"MCTSBot { iterations: 10000, exploration_weight: 2 }" => MCTS,
"MCTSHeuristicBot { iterations: 10000, exploration_weight: 2, heuristic: SolverHeuristicSimplified }" => MCTSSolver,
"MCTSHeuristicBot { iterations: 10000, exploration_weight: 2, heuristic: MaterialHeuristic }" => MCTSCapture,
"MCTSHeuristicBot { iterations: 10000, exploration_weight: 2, heuristic: AdvancementHeuristic }" => MCTSAdvance,
_ => return Err(io::Error::new(ErrorKind::InvalidData, format!("Expected bot Debug value, got {}", s)))
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Results {
    inner: HashMap<u32, HashMap<ResultKey, WDL>>,
}

impl Results {
    pub fn get(&self, size: u32, key: ResultKey) -> Option<WDL> {
        self.inner.get(&size).and_then(|map| map.get(&key)).cloned()
    }

    pub fn get_cumulative(&self, size: u32, key: BotType) -> Option<WDL> {
        let map = self.inner.get(&size)?;

        map.iter()
            .filter_map(|(k, v)| {
                if k.left == key {
                    Some(v.clone())
                } else if k.right == key {
                    Some(-v.clone())
                } else {
                    None
                }
            })
            .reduce(|acc, wdl| acc + wdl)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WDL {
    pub win: u32,
    pub draw: u32,
    pub loss: u32,
}

impl Add for WDL {
    type Output = WDL;

    fn add(self, rhs: Self) -> Self::Output {
        WDL::new(
            self.win + rhs.win,
            self.draw + rhs.draw,
            self.loss + rhs.draw,
        )
    }
}

impl Neg for WDL {
    type Output = WDL;

    fn neg(self) -> Self::Output {
        WDL::new(self.loss, self.draw, self.win)
    }
}

impl WDL {
    pub fn new(win: u32, draw: u32, loss: u32) -> Self {
        WDL { win, draw, loss }
    }

    pub fn combined(&self) -> f32 {
        self.win as f32 + self.draw as f32 * 0.5
    }
}

pub fn parse_results(read: impl BufRead) -> Result<Results> {
    let mut map = {
        let mut map = HashMap::new();
        for size in 3..=8 {
            map.insert(size, HashMap::new());
        }
        map
    };
    let mut selected_map = map.get_mut(&3).unwrap();

    for line in read.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"size: (?P<size>[3-8])"#).unwrap());

        if let Some(captures) = RE.captures(&line) {
            let size = capture_u32(&captures, "size").unwrap();
            selected_map = map.get_mut(&size).unwrap();
        } else {
            let (key, val) = parse_line(&line)?;
            selected_map.insert(key, val);
        }
    }

    Ok(Results { inner: map })
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ResultKey {
    left: BotType,
    right: BotType,
}

fn capture_u32(capture: &Captures, name: &str) -> Result<u32> {
    capture
        .name(name)
        .unwrap()
        .as_str()
        .parse()
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Expected a number"))
}

fn parse_line(s: &str) -> Result<(ResultKey, WDL)> {
    fn parse_wdl(s: &str) -> Result<WDL> {
        static RE_WDL: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"W:(?P<w>\d{1,4}),D:(?P<d>\d{1,4}),L:(?P<l>\d{1,4})"#).unwrap()
        });

        let captures = RE_WDL
            .captures(s)
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "Input did not match regex."))?;
        Ok(WDL::new(
            capture_u32(&captures, "w")?,
            capture_u32(&captures, "d")?,
            capture_u32(&captures, "l")?,
        ))
    }

    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"L: (?P<l>.+?) \(.+?\) \| (?P<wdl>.+?) \| R: (?P<r>.+?) \(.+?\)"#).unwrap()
    });

    let captures = RE
        .captures(s)
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Input doesn't match regex"))?;
    let left: BotType = captures.name("l").unwrap().as_str().parse()?;
    let right: BotType = captures.name("r").unwrap().as_str().parse()?;
    let wdl = parse_wdl(captures.name("wdl").unwrap().as_str().trim())?;

    Ok((ResultKey { left, right }, wdl))
}
