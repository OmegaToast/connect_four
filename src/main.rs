use std::{io, sync::{Mutex, Arc}, thread};


#[derive(Debug, PartialEq, Clone, Copy)]
enum Player {
    Red,
    Yellow,
}

type GameBoard = [[Option<Player>; 6]; 7]; // 7x6 array of Player


fn main() {
    let mut current_game: GameBoard = [[None; 6]; 7];
    current_game[3][0] = Some(Player::Red);
    let mut current_player: Player = Player::Yellow;
    fancy_print(current_game);
    // let mut big_ass_list: Vec<(Vec<i32>, GameBoard, i32, i32)> = Vec::new();
    // big_ass_list.insert(0, );
    loop {
        match current_player {
            Player::Red => {
                current_game = drop(current_game, get_best_move(current_game, Player::Red).try_into().unwrap(), Player::Red).unwrap();
            },
            Player::Yellow => {

                // current_game = drop(current_game, get_best_move(current_game, Player::Yellow).try_into().unwrap(), Player::Yellow).unwrap();

                let mut drop_index = String::new();
                
                io::stdin()
                    .read_line(&mut drop_index)
                    .unwrap();
                let drop_index = drop_index.trim().parse::<usize>().unwrap() - 1;
        
                current_game = drop(current_game, drop_index, current_player).unwrap();
                
                println!("wins: {:?}", winner(current_game));
            },
        }
        
        fancy_print(current_game);
        println!("\n");

        current_player = match current_player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

fn fancy_print(board: GameBoard) {
    for y in 0..6 {
        for x in 0..7 {
            print!("{} ", match board[x][5-y] {
                Some(x) => match x {
                    Player::Red => "X",
                    Player::Yellow => "O",
                },
                None => "_",
            });
        }
        println!();
    }
    println!("1 2 3 4 5 6 7");
    match winner(board) {
        Some(x) => println!("{:?} has won!", x),
        None => (),
    }
}

fn drop(board: GameBoard, x: usize, player: Player) -> Result<GameBoard, ()> {
    if x > 6 || board[x][5].is_some() {return Err(());}
    for y in 0..6 {
        if board[x][y].is_none() {
            let mut new_board: GameBoard = board.clone();
            new_board[x][y] = Some(player);
            return Ok(new_board);
        }
    }
    Err(())
}

fn winner(board: GameBoard) -> Option<Player>{
    for player in [Player::Red, Player::Yellow] {
        let mut add: [i32; 12] = [0; 12];
        let mut sub: [i32; 12] = [0; 12];
        let mut s_x: [i32; 6] = [0; 6];
        for x in 0..7 {
            let mut s_y: i32 = 0;
            for y in 0..6 {
                if board[x][y] == Some(player) {
                    s_y += 1;
                    s_x[y] += 1;
                    add[x+y] += 1;
                    sub[(x as i32-y as i32+5) as usize] += 1;
                    if s_y == 4 || s_x[y] == 4 || add[x+y] == 4 || sub[(x as i32-y as i32+5) as usize] == 4 {
                        return Some(player);
                    }
                } else {
                    s_y = 0;
                    s_x[y] = 0;
                    add[x+y] = 0;
                    sub[(x as i32 - y as i32 + 5) as usize] = 0;
                }
            }
        }
    }
    // if it ain't broke, don't fix it
    None
}

fn get_moves(board: GameBoard) -> Vec<i32> {
    let mut moves = Vec::new();
    for x in 0..7 {
        if board[x][5].is_none() {
            moves.insert(0, x as i32);
        }
    }
    moves.reverse(); // can be removed
    moves
}

fn get_best_move(board: GameBoard, player: Player) -> i32 {
    

    let best_move = Arc::new(Mutex::new(0));
    let best_eval = Arc::new(Mutex::new(match player {
        Player::Red => -999,
        Player::Yellow => 999,
    }));

    let mut handles = Vec::new();

    for m in get_moves(board) {
        let best_move = Arc::clone(&best_move);
        let best_eval = Arc::clone(&best_eval);
        // let current_move = m.clone();
        let handle = thread::spawn(move || {
            let new_board = drop(board, m.try_into().unwrap(), player).unwrap();
            let eval = minimax(new_board, 50, -999, 999, player == Player::Yellow);
            let mut best_eval_lock = best_eval.lock().unwrap();
            let mut best_move_lock = best_move.lock().unwrap();
            if player == Player::Red {
                if eval > *best_eval_lock {
                    *best_eval_lock = eval;
                    *best_move_lock = m;
                }
            } else {
                if eval < *best_eval_lock {
                    *best_eval_lock = eval;
                    *best_move_lock = m;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = *best_move.lock().unwrap(); x
}

fn minimax(board: GameBoard, depth: i32, mut alpha: i32, mut beta: i32, max_player: bool) -> i32 {
    let this_eval = winner(board);
    if depth == 0 || this_eval.is_some() || get_moves(board).is_empty() {
        return match this_eval {
            Some(x) => match x {
                Player::Red => depth,
                Player::Yellow => -depth,
            },
            None => 0,
        }
    }

    if max_player {
        let mut max_eval = -999;
        for m in get_moves(board) {
            let new_board = drop(board.clone(), m.try_into().unwrap(), Player::Red).unwrap();
            let eval = minimax(new_board, depth-1, alpha, beta, false);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    } else {
        let mut min_eval = 999;
        for m in get_moves(board) {
            let new_board = drop(board.clone(), m.try_into().unwrap(), Player::Yellow).unwrap();
            let eval = minimax(new_board, depth-1, alpha, beta, true);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        return min_eval;
    }
}