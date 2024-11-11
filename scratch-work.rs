
// Some initial data structs for the game

#[account]
pub struct Game {
    players_joined: Vec<Pubkey>,          // public addresses of users who have joined the round
    players_joined_count: u64,            // number of players who have joined the round
    players_remaining: Vec<Pubkey>,       // addresses of users who can still win the pot
    players_remaining_count: u64,         // number of players who have joined the round
    losers: Vec<Pubkey>,                  // users who have lost (rolled 123, lower than point, or timed out)
    initial_bet: u64,                     // number of lamports in the initial bet
    rake: u64,                            // number of lamports taken out of the pot as our fee 
    total_pot: u64,                       // total number of lamports in the pot
    current_point: u8,                    // current point die roll (between 1 and 6)
    trips: bool,                          // whether current point is a triple
    winner: Pubkey,                       // public address of the  winner
    current_roller: Pubkey,               // address of the user whose turn it is to roll
    winner_456: bool,                     // whether a player has won with 456
    push_count: u16,                      // how many pushes there have been
    game_start_timestamp: i64,            // timer after second player joins before game starts (allowing more to join) 
    roll_deadline: i64,                   // when roller must roll by, timestamp that gets reset after each roll 
    push_deadline: i64,                   // when players must call push by, timestamp that gets reset on each push roll
}

#[account]
struct Player { 
    total_bet: u64,                       // total amount the player has put in the pot
    join_time: i64,                       // timestamp of when the player joined the round
    last_roll_time: i6,                   // timestamp of when the player last rolled
    last_roll_time: i6,                   // timestamp of when the player last called a push
    last_roll: i6,                        // value of the player's last roll (1-6 is a point, zero means no point)
    trips: bool,                          // whether the last roll was triples or a regular point
    winner_456: bool,                     // whether the user has won with a 456 roll
    loser_123: bool,                      // whether the user has lost with a 123 roll
    loser_lower_point: bool,              // whether the user has lost by making a point lower than the current one
    loser_roll_timeout: bool,             // whether the user has lost by not rolling in time
    loser_push_timeout: bool,             // whether the user has lost by not calling a push in time 
}