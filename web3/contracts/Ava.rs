// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use lazy_static::lazy_static;
use rand::Rng;

use ethereum_types::Address;
use sha3::{Digest, Keccak256};
use zksync_crypto::params::MAX_CIRCUIT_TREE_DEPTH;

struct GameToken {
    name: String,
    id: u256,
    attack_strength: u256,
    defense_strength: u256,
}

struct Player {
    player_address: Address,
    player_name: String,
    player_mana: u256,
    player_health: u256,
    in_battle: bool,
}

enum BattleStatus {
    PENDING,
    STARTED,
    ENDED,
}

struct Battle {
    battle_status: BattleStatus,
    battle_hash: [u8; 32],
    name: String,
    players: [Address; 2],
    moves: [u8; 2],
    winner: Address,
}

lazy_static! {
    static ref PLAYER_INFO: HashMap<Address, u256> = HashMap::new();
    static ref PLAYER_TOKEN_INFO: HashMap<Address, u256> = HashMap::new();
    static ref BATTLE_INFO: HashMap<String, u256> = HashMap::new();
    static ref PLAYERS: Vec<Player> = Vec::new();
    static ref GAME_TOKENS: Vec<GameToken> = Vec::new();
    static ref BATTLES: Vec<Battle> = Vec::new();
    static ref BASE_URI: String = String::new();
    static ref TOTAL_SUPPLY: u256 = 0u256;
    static ref DEVIL: u256 = 0u256;
    static ref GRIFFIN: u256 = 1u256;
    static ref FIREBIRD: u256 = 2u256;
    static ref KAMO: u256 = 3u256;
    static ref KUKULKAN: u256 = 4u256;
    static ref CELESTION: u256 = 5u256;
    static ref MAX_ATTACK_DEFEND_STRENGTH: u256 = 10u256;
}

fn is_player(addr: Address) -> bool {
    PLAYER_INFO.contains_key(&addr)
}

fn get_player(addr: Address) -> Option<&Player> {
    let player_index = PLAYER_INFO.get(&addr)?;
    PLAYERS.get(*player_index as usize)
}

fn get_all_players() -> Vec<Player> {
    PLAYERS.clone()
}

fn is_player_token(addr: Address) -> bool {
    PLAYER_TOKEN_INFO.contains_key(&addr)
}

fn get_player_token(addr: Address) -> Option<&GameToken> {
    let token_index = PLAYER_TOKEN_INFO.get(&addr)?;
    GAME_TOKENS.get(*token_index as usize)
}

fn get_all_player_tokens() -> Vec<GameToken> {
    GAME_TOKENS.clone()
}

fn is_battle(name: &str) -> bool {
    BATTLE_INFO.contains_key(name)
}

fn get_battle(name: &str) -> Option<&Battle> {
    let battle_index = BATTLE_INFO.get(name)?;
    BATTLES.get(*battle_index as usize)
}

fn get_all_battles() -> Vec<Battle> {
    BATTLES.clone()
}

fn update_battle(name: &str, new_battle: &Battle) {
    if let Some(battle_index) = BATTLE_INFO.get(name) {
        BATTLES[*battle_index as usize] = new_battle.clone();
    }
}

fn initialize() {
    GAME_TOKENS.push(GameToken {
        name: "".to_string(),
        id: 0u256,
        attack_strength: 0u256,
        defense_strength: 0u256,
    });

    PLAYERS.push(Player {
        player_address: Address::from([0u8; 20]),
        player_name: "".to_string(),
        player_mana: 0u256,
        player_health: 0u256,
        in_battle: false,
    });

    BATTLES.push(Battle {
        battle_status: BattleStatus::PENDING,
        battle_hash: [0u8; 32],
        name: "".to_string(),
        players: [Address::from([0u8; 20]), Address::from([0u8; 20])],
        moves: [0u8; 2],
        winner: Address::from([0u8; 20]),
    });
}

fn create_random_num(max: u256, sender: Address) -> u256 {
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..u256::MAX);
    let random_value = random_num % max;
    if random_value == 0u256 {
        max / 2u256
    } else {
        random_value
    }
}

fn create_game_token(name: &str) -> GameToken {
    let rand_attack_strength = create_random_num(MAX_ATTACK_DEFEND_STRENGTH, sender);
    let rand_defense_strength = MAX_ATTACK_DEFEND_STRENGTH - rand_attack_strength;

    let mut rng = rand::thread_rng();
    let rand_id = rng.gen_range(0..100u8) % 6u8 + 1u8;

    let new_game_token = GameToken {
        name: name.to_string(),
        id: rand_id.into(),
        attack_strength: rand_attack_strength,
        defense_strength: rand_defense_strength,
    };

    let token_index = GAME_TOKENS.len() as u256;
    GAME_TOKENS.push(new_game_token.clone());
    PLAYER_TOKEN_INFO.insert(sender, token_index);
    TOTAL_SUPPLY += 1u256;

    // _mint(sender, rand_id, 1, '0x0'); // Call your token minting function here

    new_game_token
}

fn create_random_game_token(name: &str) {
    if !getPlayer(sender).in_battle && is_player(sender) {
        create_game_token(name);
    }
}

fn get_total_supply() -> u256 {
    *TOTAL_SUPPLY
}

fn create_battle(name: &str) -> Battle {
    if is_player(sender) && !is_battle(name) {
        let battle_hash = keccak256(name.as_bytes());
        let new_battle = Battle {
            battle_status: BattleStatus::PENDING,
            battle_hash,
            name: name.to_string(),
            players: [sender, Address::from([0u8; 20])],
            moves: [0u8; 2],
            winner: Address::from([0u8; 20]),
        };

        let battle_index = BATTLES.len() as u256;
        BATTLE_INFO.insert(name.to_string(), battle_index);
        BATTLES.push(new_battle.clone());

        new_battle
    } else {
        // Handle error condition here
        // e.g., return an error struct or panic
    }
}

fn join_battle(name: &str) -> Battle {
    if is_player(sender) {
        let mut battle = get_battle(name).unwrap().clone();

        if battle.battle_status == BattleStatus::PENDING
            && battle.players[0] != sender
            && !getPlayer(sender).in_battle
        {
            battle.battle_status = BattleStatus::STARTED;
            battle.players[1] = sender;

            update_battle(name, &battle);

            let player1_index = PLAYER_INFO.get(&battle.players[0]).unwrap();
            let player2_index = PLAYER_INFO.get(&battle.players[1]).unwrap();

            PLAYERS[*player1_index as usize].in_battle = true;
            PLAYERS[*player2_index as usize].in_battle = true;

            // Emit NewBattle event (you would need to implement event handling)
            // emit_new_battle(battle.name, battle.players[0], sender);

            battle
        } else {
            // Handle error condition here
            // e.g., return an error struct or panic
        }
    } else {
        // Handle error condition here
        // e.g., return an error struct or panic
    }
}

fn get_battle_moves(battle_name: &str) -> (u8, u8) {
    let battle = get_battle(battle_name).unwrap();
    (battle.moves[0], battle.moves[1])
}

fn register_player_move(player: u8, choice: u8, battle_name: &str) {
    if choice == 1 || choice == 2 {
        if choice == 1 && getPlayer(sender).player_mana >= 3 {
            let mut battle = get_battle(battle_name).unwrap();
            battle.moves[player as usize] = choice;
            update_battle(battle_name, &battle);
        }
    } else {
        // Handle error condition here
        // e.g., return an error struct or panic
    }
}

fn attack_or_defend_choice(choice: u8, battle_name: &str) {
    let mut battle = get_battle(battle_name).unwrap();

    if battle.battle_status == BattleStatus::STARTED
        && battle.battle_status != BattleStatus::ENDED
        && (battle.players[0] == sender || battle.players[1] == sender)
    {
        if battle.moves[(battle.players[0] == sender) as usize] == 0 {
            register_player_move((battle.players[0] == sender) as u8, choice, battle_name);

            let battle = get_battle(battle_name).unwrap();
            let moves_left = 2 - (battle.moves[0] == 0) as u8 - (battle.moves[1] == 0) as u8;

            // Emit BattleMove event (you would need to implement event handling)
            // emit_battle_move(battle_name, moves_left == 1);

            if moves_left == 0 {
                await_battle_results(battle_name);
            }
        } else {
            // Handle error condition here
            // e.g., return an error struct or panic
        }
    } else {
        // Handle error condition here
        // e.g., return an error struct or panic
    }
}

fn await_battle_results(battle_name: &str) {
    let battle = get_battle(battle_name).unwrap();

    if battle.players[0] == sender || battle.players[1] == sender {
        if battle.moves[0] != 0 && battle.moves[1] != 0 {
            resolve_battle(&battle);
        } else {
            // Handle error condition here
            // e.g., return an error struct or panic
        }
    } else {
        // Handle error condition here
        // e.g., return an error struct or panic
    }
}

struct P {
    index: u256,
    move: u8,
    health: u256,
    attack: u256,
    defense: u256,
}

fn resolve_battle(battle: &Battle) {
    let p1 = P {
        index: PLAYER_INFO.get(&battle.players[0]).unwrap().clone(),
        move: battle.moves[0].clone(),
        health: getPlayer(battle.players[0]).player_health.clone(),
        attack: getPlayer_token(battle.players[0]).attack_strength.clone(),
        defense: getPlayer_token(battle.players[0]).defense_strength.clone(),
    };

    let p2 = P {
        index: PLAYER_INFO.get(&battle.players[1]).unwrap().clone(),
        move: battle.moves[1].clone(),
        health: getPlayer(battle.players[1]).player_health.clone(),
        attack: getPlayer_token(battle.players[1]).attack_strength.clone(),
        defense: getPlayer_token(battle.players[1]).defense_strength.clone(),
    };

    let mut damaged_players: [Address; 2] = [Address::from([0u8; 20]), Address::from([0u8; 20])];

    if p1.move == 1 && p2.move == 1 {
        if p1.attack >= p2.health {
            end_battle(battle.players[0], battle);
        } else if p2.attack >= p1.health {
            end_battle(battle.players[1], battle);
        } else {
            PLAYERS[p1.index as usize].player_health -= p2.attack;
            PLAYERS[p2.index as usize].player_health -= p1.attack;

            PLAYERS[p1.index as usize].player_mana -= 3;
            PLAYERS[p2.index as usize].player_mana -= 3;

            // Both players' health damaged
            damaged_players = battle.players;
        }
    } else if p1.move == 1 && p2.move == 2 {
        let phad = p2.health + p2.defense;
        if p1.attack >= phad {
            end_battle(battle.players[0], battle);
        } else {
            let health_after_attack;

            if p2.defense > p1.attack {
                health_after_attack = p2.health;
            } else {
                health_after_attack = phad - p1.attack;

                // Player 2 health damaged
                damaged_players[0] = battle.players[1];
            }

            PLAYERS[p2.index as usize].player_health = health_after_attack;

            PLAYERS[p1.index as usize].player_mana -= 3;
            PLAYERS[p2.index as usize].player_mana += 3;
        }
    } else if p1.move == 2 && p2.move == 1 {
        let phad = p1.health + p1.defense;
        if p2.attack >= phad {
            end_battle(battle.players[1], battle);
        } else {
            let health_after_attack;

            if p1.defense > p2.attack {
                health_after_attack = p1.health;
            } else {
                health_after_attack = phad - p2.attack;

                // Player 1 health damaged
                damaged_players[0] = battle.players[0];
            }

            PLAYERS[p1.index as usize].player_health = health_after_attack;

            PLAYERS[p1.index as usize].player_mana += 3;
            PLAYERS[p2.index as usize].player_mana -= 3;
        }
    } else if p1.move == 2 && p2.move == 2 {
        PLAYERS[p1.index as usize].player_mana += 3;
        PLAYERS[p2.index as usize].player_mana += 3;
    }

    // Emit RoundEnded event (you would need to implement event handling)
    // emit_round
    // Ended event
    // emit_round_ended(damaged_players);

    // Reset moves to 0
    let mut battle = get_battle(battle_name).unwrap();
    battle.moves[0] = 0;
    battle.moves[1] = 0;
    update_battle(battle_name, &battle);

    // Reset random attack and defense strength
    let random_attack_strength_player1 = create_random_num(MAX_ATTACK_DEFEND_STRENGTH, &battle.players[0]);
    GAME_TOKENS[PLAYER_TOKEN_INFO.get(&battle.players[0]).unwrap().clone()].attack_strength = random_attack_strength_player1.clone();
    GAME_TOKENS[PLAYER_TOKEN_INFO.get(&battle.players[0]).unwrap().clone()].defense_strength = MAX_ATTACK_DEFEND_STRENGTH - random_attack_strength_player1.clone();

    let random_attack_strength_player2 = create_random_num(MAX_ATTACK_DEFEND_STRENGTH, &battle.players[1]);
    GAME_TOKENS[PLAYER_TOKEN_INFO.get(&battle.players[1]).unwrap().clone()].attack_strength = random_attack_strength_player2.clone();
    GAME_TOKENS[PLAYER_TOKEN_INFO.get(&battle.players[1]).unwrap().clone()].defense_strength = MAX_ATTACK_DEFEND_STRENGTH - random_attack_strength_player2.clone();
}

fn quit_battle(battle_name: &str) {
    let mut battle = get_battle(battle_name).unwrap();
    if battle.players[0] == sender || battle.players[1] == sender {
        let battle_loser = if battle.players[0] == sender {
            battle.players[1]
        } else {
            battle.players[0]
        };

        end_battle(battle_loser, battle);
    }
}

fn end_battle(battle_ender: Address, mut battle: Battle) {
    if battle.battle_status != BattleStatus::ENDED {
        battle.battle_status = BattleStatus::ENDED;
        battle.winner = battle_ender.clone();
        update_battle(&battle.name, &battle);

        let player1_index = PLAYER_INFO.get(&battle.players[0]).unwrap();
        let player2_index = PLAYER_INFO.get(&battle.players[1]).unwrap();

        PLAYERS[*player1_index as usize].in_battle = false;
        PLAYERS[*player1_index as usize].player_health = 25;
        PLAYERS[*player1_index as usize].player_mana = 10;

        PLAYERS[*player2_index as usize].in_battle = false;
        PLAYERS[*player2_index as usize].player_health = 25;
        PLAYERS[*player2_index as usize].player_mana = 10;

        let battle_loser = if battle_ender == battle.players[0] {
            battle.players[1]
        } else {
            battle.players[0]
        };

        // Emit BattleEnded event (you would need to implement event handling)
        // emit_battle_ended(battle.name, battle_ender, battle_loser);
        emit_battle_ended(battle.name.clone(), battle_ender.clone(), battle_loser.clone());
    }
}

// Implement the uintToStr function in Rust

// Implement the token_uri function in Rust

// Implement the _before_token_transfer function in Rust

// You will also need to implement any missing structs and enums, and handle event emissions, storage, and other contract-specific logic.
    // Emit BattleEnded event (you would need to implement event handling)
    


// Implement the uintToStr function in Rust
fn uint_to_str(n: u256) -> String {
    if n == 0u256 {
        return "0".to_string();
    }

    let mut j = n;
    let mut len = 0u256;
    while j != 0u256 {
        len += 1u256;
        j /= 10u256;
    }

    let mut bstr = vec![0u8; len.clone() as usize];
    let mut k = len.clone();
    let mut i = n.clone();
    while i != 0u256 {
        k -= 1u256;
        let temp = (48u8 + (i - (i / 10u256) * 10u256)) as u8;
        bstr[k as usize] = temp;
        i /= 10u256;
    }

    String::from_utf8(bstr).unwrap()
}

// Implement the token_uri function in Rust
fn token_uri(token_id: u256) -> String {
    format!("{}/{:?}.json", base_uri, token_id)
}

// Implement the _before_token_transfer function in Rust
fn _before_token_transfer(
    operator: Address,
    from: Address,
    to: Address,
    ids: Vec<u256>,
    amounts: Vec<u256>,
    data: Vec<u8>,
) {
    super::_before_token_transfer(operator, from, to, ids.clone(), amounts.clone(), data.clone());
}
