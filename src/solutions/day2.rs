use crate::utils::strings::read_lines;

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;
const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;
const SCORE_LOSE: i32 = 0;

struct Round {
    my: String,
    oppo: String,
}

#[allow(dead_code)]
pub fn day2() {
    let lines = read_lines("day2.txt");
    // let mut rounds = vec![];
    let mut total1 = 0;
    let mut total2 = 0;
    for lines in lines {
        let l = lines.unwrap();
        let parts: Vec<&str> = l.split(" ").collect();
        let r = Round {
            oppo: String::from(parts[0]),
            my: String::from(parts[1]),
        };
        total1 += scores1(&r);
        total2 += scores2(&r);
    }

    println!(
        "Day\t2: Total score for round 1: {}, round 2: {}",
        total1, total2
    );
}

fn scores1(rounds: &Round) -> i32 {
    let mut score: i32;
    // if I play rock
    if rounds.my == "X" {
        score = SCORE_ROCK;
        if rounds.oppo == "A" {
            // oppo plays rock
            score += SCORE_DRAW;
        } else if rounds.oppo == "C" {
            // oppo plays scissors
            score += SCORE_WIN;
        }
    } else if rounds.my == "Y" {
        // if I play paper
        score = SCORE_PAPER;
        if rounds.oppo == "A" {
            // opponent plays rock
            score += SCORE_WIN;
        } else if rounds.oppo == "B" {
            // opponent plays paper
            score += SCORE_DRAW;
        }
    } else {
        // if I play scissors
        score = SCORE_SCISSORS;
        if rounds.oppo == "B" {
            // oppo plays
            score += SCORE_WIN;
        } else if rounds.oppo == "C" {
            score += SCORE_DRAW;
        }
    }

    score
}

fn scores2(r: &Round) -> i32 {
    let mut score: i32;
    if r.my == "X" {
        // I need to lose
        score = SCORE_LOSE;
        if r.oppo == "A" {
            // to rock, I have to play scissors
            score += SCORE_SCISSORS;
        } else if r.oppo == "B" {
            // to paper, I have to play rock
            score += SCORE_ROCK;
        } else {
            score += SCORE_PAPER;
        }
    } else if r.my == "Y" {
        // I need to draw
        score = SCORE_DRAW;
        if r.oppo == "A" {
            // to rock, I have to play rock
            score += SCORE_ROCK;
        } else if r.oppo == "B" {
            // to paper, I have to play paper
            score += SCORE_PAPER;
        } else {
            score += SCORE_SCISSORS;
        }
    } else {
        score = SCORE_WIN;
        if r.oppo == "A" {
            // to rock, I have to play paper
            score += SCORE_PAPER;
        } else if r.oppo == "B" {
            // to paper, I have to play scissor
            score += SCORE_SCISSORS;
        } else {
            score += SCORE_ROCK;
        }
    }

    score
}
