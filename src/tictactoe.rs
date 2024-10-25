use leptos::*;
use crate::components::tictactoe_component::TicTacToeBoard;
use crate::components::weather_component::WeatherDashboard;

#[derive(Clone, Copy, PartialEq, Debug)]  // Added Debug derive
pub enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Clone)]
pub struct GameState {
    pub board: [Option<Player>; 9],
    pub current_player: Player,
    pub winner: Option<Player>,
    pub game_over: bool,
    pub winning_cells: Option<[usize; 3]>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: [None; 9],
            current_player: Player::X,
            winner: None,
            game_over: false,
            winning_cells: None,
        }
    }
}

#[component]
pub fn TicTacToe() -> impl IntoView {
    let (state, set_state) = create_signal(GameState::default());

    let on_cell_click = move |index: usize| {
        set_state.update(|s| {
            if s.board[index].is_none() && !s.game_over {
                s.board[index] = Some(s.current_player);
                check_game_state(s);
                s.current_player = match s.current_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
            }
        });
    };

    let reset_game = move |_| {
        set_state.set(GameState::default());
    };

    view! {
        <div class="flex flex-row gap-8 justify-center items-start p-8">
        <div class="flex flex-col gap-4 justify-center items-center">
            <p class="flex justify-center items-center text-2xl font-bold">"Tic-Tac-Toe"</p>
            <TicTacToeBoard 
                on_cell_click=on_cell_click 
                state=state.into()
            />
            <div class="text-xl font-semibold p-2">
                {move || match state.get().winner {
                    Some(player) => format!("Winner: {}", player),
                    None if state.get().game_over => "It's a tie!".to_string(),
                    None => format!("Current player: {}", state.get().current_player)
                }}
            </div>
            <button class="border border-2 border-black bg-[#f0f0f0] hover:bg-[#ddd] p-2 rounded-[0.625rem] px-12" on:click=reset_game>
                "Play Again"
            </button>
            </div>
            
            // Weather Dashboard
            <div class="weather-container bg-white p-6 rounded-lg shadow-lg">
                <WeatherDashboard/>
            </div>
        </div>
    }
}

fn check_game_state(state: &mut GameState) {
    let winning_combinations = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],  // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8],  // Columns
        [0, 4, 8], [2, 4, 6],             // Diagonals
    ];

    for &comb in &winning_combinations {
        if let [Some(p1), Some(p2), Some(p3)] = 
            [state.board[comb[0]], state.board[comb[1]], state.board[comb[2]]] {
            if p1 == p2 && p2 == p3 {
                state.winner = Some(p1);
                state.game_over = true;
                state.winning_cells = Some(comb);  // Add this line to set winning cells
                break;
            }
        }
    }

    if state.board.iter().all(|&cell| cell.is_some()) && state.winner.is_none() {
        state.game_over = true;  // Tie condition
    }
}