fn smith_waterman(seq1: &str, seq2: &str, match_score: i32, mismatch_penalty: i32, gap_penalty: i32) -> (String, String, i32) {
    let seq1: Vec<char> = seq1.chars().collect();
    let seq2: Vec<char> = seq2.chars().collect();
    let rows = seq1.len() + 1;
    let cols = seq2.len() + 1;
    let mut score_matrix = vec![vec![0; cols]; rows];
    
    let mut max_score = 0;
    let mut max_pos = (0, 0);

    // Fill the scoring matrix
    for i in 1..rows {
        for j in 1..cols {
            let top_left_score = score_matrix[i - 1][j - 1] + if seq1[i - 1] == seq2[j - 1] {
                match_score
            } else {
                mismatch_penalty
            };
            let left_score = score_matrix[i - 1][j] + gap_penalty;
            let top_score = score_matrix[i][j - 1] + gap_penalty;
            
            let score = [0, top_left_score, left_score, top_score].iter().cloned().max().unwrap();
            score_matrix[i][j] = score;

            if score > max_score {
                max_score = score;
                max_pos = (i, j);
            }
        }
    }

    // Print the score matrix (optional for debugging)
    println!("Score matrix:");
    for row in &score_matrix {
        println!("{:?}", row);
    }

    // Traceback to find the optimal local alignment
    let (mut i, mut j) = max_pos;
    let mut align1 = String::new();
    let mut align2 = String::new();

    while score_matrix[i][j] > 0 {
        if i > 0 && j > 0 && score_matrix[i][j] == score_matrix[i - 1][j - 1] +
            if seq1[i - 1] == seq2[j - 1] { match_score } else { mismatch_penalty } {
            align1.insert(0, seq1[i - 1]);
            align2.insert(0, seq2[j - 1]);
            i -= 1;
            j -= 1;
        } else if i > 0 && score_matrix[i][j] == score_matrix[i - 1][j] + gap_penalty {
            align1.insert(0, seq1[i - 1]);
            align2.insert(0, '-');
            i -= 1;
        } else {
            align1.insert(0, '-');
            align2.insert(0, seq2[j - 1]);
            j -= 1;
        }
    }

    (align1, align2, max_score)
}

fn main() {
    let seq1 = "AGACTAGTTAC";
    let seq2 = "CGTGAATTCAT";
    let (align1, align2, score) = smith_waterman(&seq1, &seq2, 2, -1, -1);

    println!("Aligned Sequences:");
    println!("{}", align1);
    println!("{}", align2);
    println!("Alignment Score: {}", score);
}
