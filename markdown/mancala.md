# Solving Mancala in Rust
### 2.14.24
---
This project was originally inspired by playing GamePigeon Mancala with my friend...

By the end of this post, you should have a fully working program to give you the most optimal move in your game of Mancala!

* This post assumes you have basic knowledge of the Rust Programming Language, there will be links to further reading where relevant if you need help with concepts. If you do not know Rust, start by reading the [Rust Book](https://doc.rust-lang.org/book/).

## Following the Gamepigeon rules (avalanche):  {#rules}
> 1. Each player has a store on one side of the board.
> 2. Players take turns choosing a pile from one of the holes. Moving counter-clockwise, stones from the selected pile are deposited in each of the following hole.
> 3. If you drop the last stone into an unempty hole, you will pick up the stones from that hole and continue depositing them counter-clockwise.
> 4. Your turn is over when you drop the last stone into an empty hole.
> 5. If you drop the last stone into your store - you get a free turn.
> 6. The game ends when all six holes on either side of the board are empty.
> The player with the most stones in their store wins.

## Our Mancala board {#board}

```md
---              Player Side                     ---
| Player [00][12][11][10][09][08][07][NA] Opponent |
| Store >[00]------------------------[NA]< Store   |
|        [00][01][02][03][04][05][06][NA]          |
---              Opponent Side                   ---
```
We will be numbering our board spaces, starting with our player "store", going down the opponent side, and then back through the player side. Making the store the first element will be helpful later.

(The opponent store is skipped because you do not move there)

## New game layout, and user input {#new-game}

```md
---              Player Side                     ---
| Player [00][04][04][04][04][04][04][00] Opponent |
| Store >[00]------------------------[00]< Store   |
|        [00][04][04][04][04][04][04][00]          |
---              Opponent Side                   ---
```
This is what a new game looks like on our board, all sides are filled with 4 pieces, and the stores are empty.

Alternatively, we also want to support inputting an existing game at any point...

---

## Start by creating a new project

`cargo new mancala-solver`

This will give you `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

## First Steps - Making a Board {#making-a-board}

To solve a game of Mancala, first we need the board. Let's make a board!

Go ahead and put this at the top of your `main.rs`:

```rust
struct MancalaBoard {
    spaces: [u8; 13],
    opponent_store: u8,
    move_history: Vec<u8>,
    move_count: u8,
    did_avalanche: bool,
}
```

Read more about structs [here](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

We will represent our board spaces as an array of `u8` integers, 13 in length (6 spaces per side + player store). These are all the movable spaces, so they are the ones we have to calculate on. 

We also want to keep track of the opponent store just incase we want to do more with it later (displaying the board, etc.).

We will also keep a vector of `u8` to represent each move made on a board, this will be used to show the user at the end the proper moves to take. (This vector will NOT store avalanche moves as they are made automatically).

The number of moves taken will be tracked so we can terminate a board if it reaches past the maximum simulation depth (More on this later).

We will also need a boolean to know if the last move ended with an avalanche, in which case we will not push the next move onto the move history vector.

### Creating a default board {#default-board}

Now that we have a board, we need a way to either: create a new game with default piece layout, or accept a user created board. We will start with the former.

Add this underneath the `MancalaBoard` struct:
```rust
impl Default for MancalaBoard {
    fn default() -> Self {
        MancalaBoard {
            spaces: [0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
            opponent_store: 0,
            move_history: vec![],
            move_count: 0,
            did_avalanche: false,
        }
    }
}
```

Here we are implementing the `Default` trait for our board. 

We will create an instance of our `MancalaBoard`, inside of which we will create a new array, starting with a 0 to represent our player store, and the rest filled with 4's (these are both the player, and opponent sides combined). We will also initialize our opponent store to be 0. Lastly, an empty vector will be created with the `vec![]` macro for our move history, preparing new moves to be added as we make them.

### Creating a new board from existing pieces {#existing-board}

Add this underneath the `Default` impl:
```rust
impl MancalaBoard {
    fn new(spaces: [u8; 13], opponent_store: u8) -> MancalaBoard {
        return MancalaBoard {
            spaces,
            opponent_store,
            move_history: vec![],
            move_count: 0,
            did_avalanche: false,
        };
    }
}
```

Here we are just offering a simple way to create a new board from existing pieces. Doing this will allow us to initialize the move history seperately so we don't have to make it ourselves everytime we need a new board.

### Taking user input for a board {#user-board}

The above function allows us to create a board with an existing layout, but we need a way to call it that is user-friendly.

Add this to your code:

```rust
use std::io::Write;
```

```rust
fn get_user_board() -> MancalaBoard {
    let mut input = String::new();

    // Get player store
    print!("Player Store: ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let player_store: u8 = input
        .trim()
        .parse()
        .expect("error parsing player store input");
    input.clear();

    // Get opponent store
    print!("Opponent Store: ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let opponent_store: u8 = input
        .trim()
        .parse()
        .expect("error parsing player store input");
    input.clear();

    // Get player spaces
    print!("Player Spaces (opponent->player, space-seperated): ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let mut player_spaces: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|number_string| number_string.parse::<u8>().unwrap().try_into().unwrap())
        .collect();

    input.clear();

    // Get opponent spaces
    print!("Opponent Spaces (player->opponent, space-seperated): ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let mut opponent_spaces: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|number_string| number_string.parse::<u8>().unwrap().try_into().unwrap())
        .collect();
    input.clear();

    // combine input into array
    let mut spaces = vec![player_store];
    spaces.append(&mut opponent_spaces);
    spaces.append(&mut player_spaces);

    MancalaBoard::new(
        spaces
            .try_into()
            .expect("Error converting spaces vec into array"),
        opponent_store,
    )
}
```

While this function is long, it is relatively simple in nature. It will prompt the user for the store values, and then the board space values (space seperated).

### Testing!! {#testing-1}

Now that we have our board, and two ways of creating them, let's make sure everything is working. Go ahead and put this under your `main` function:

```rust
#[cfg(test)]
mod tests {
    use crate::MancalaBoard;

    #[test]
    fn default_board() {
        let board = MancalaBoard::default();
        assert_eq!([0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4], board.spaces);
    }

    #[test]
    fn new_board() {
        let board = MancalaBoard::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 0);
        assert_eq!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], board.spaces);
    }
}
```

Now you can run these tests with: `cargo test default_board`, `cargo test new_board`

Read more about tests [here](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

## Moving pieces {#moving-pieces}

Now that we can create boards, we need a way to make moves.

BUT: Before we get into the actual movement, we need to define the outcomes of our moves.

What are the possible outcomes of any move in Mancala?
1. You drop your last piece into an empty space - This simply ends the turn
2. You drop your last piece into your store - This gives you a free turn
3. You run out of possible moves and the game is over
4. You drop your last piece into a space with other pieces - This triggers an "avalance"

We are going to represent these possible outcomes in an [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) named `MoveResult`:

```rust
enum MoveResult {
    EmptySpace,
    FreeTurn,
    GameOver,
    Avalanche(usize),
}
```
The reason the `Avalanche` option stores a number, is so we know where to start moving the next turn.

Now that we have defined move outcomes, we can move on to actually writing the code to make moves.

BUT, before we get to that, we are going to need a way to check if the player side of the board is empty, for this I found a function that someone on the internet wrote and stole it.

Go ahead and add this into your code:

```rust
// https://stackoverflow.com/questions/65367552/how-to-efficiently-check-a-vecu8-to-see-if-its-all-zeros
fn is_zero(buf: &[u8]) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}
```

We will use this later.

Create a new function inside of the `impl MancalaBoard`:

```rust
fn move_piece(&mut self, mut space: usize) -> MoveResult {}
```

(We are using a `usize` for the space here because we are going to be doing a lot of indexing, which uses usize)

Before we get into any movement, we need to add the move we are about to make to the `move_history` of the board. HOWEVER, we need to check if the last move was an avalanche and skip adding it if so (this should be inside our new function):

```rust
if !self.did_avalanche {
    self.move_history.push(
        space
            .try_into()
            .expect("Could not convert usize to u8 for move history"),
    );
} else {
    self.did_avalanche = false;
}

self.move_count += 1;
```

We also increment the move counter by 1 no matter what.

We next need to simulate our player picking up pieces, we will create a `hand` variable.

```rust
let mut hand = self.spaces[space];
self.spaces[space] = 0;
```

These two lines will create our player hand and give it the amount of pieces in the space specified. It will then set that space to hold 0 pieces, as if we had picked them all up.

Next we want to simulate moving your hand over the spaces and dropping a piece in each space.

Create a `while` loop:

```rust
while hand > 0 {
    space += 1;
    self.spaces[space] += 1;
    hand -= 1;
}
```

This will move our hand over the board until we run out of pieces. There is a problem here however, if we move around to the other side of the board, our space number will go above 12 which will cause an error. We need to check and wrap around the space if it becomes too large (13->0).

Add this into the loop after `space += 1;`:

```rust
if space >= 13 {
    space -= 13
}
```

To finish off our movement, we need to return the outcome of the move (This is where our `is_zero` function is used):

```rust
if space == 0 && is_zero(&self.spaces[6..]) { // This checks if we ended our turn on the player store, and all of the player side spaces are empty (no possible moves)
    return MoveResult::GameOver;
} else if space == 0 { // Free turn when ending on player store
    return MoveResult::FreeTurn;
} else if self.spaces[space] > 1 { // If the space we ended on has more than one piece (since we just dropped one there), then we avalanche
    self.did_avalanche = true;
    return MoveResult::Avalanche(space);
} else if is_zero(&self.spaces[6..]) { // If it was not an avalanche or free turn, and the player side is empty, this also triggers a game over
    return MoveResult::GameOver;
} else { // End of turn
    return MoveResult::EmptySpace;
}
```

Now the whole function should look like this:

```rust
fn move_piece(&mut self, mut space: usize) -> MoveResult {
    if !self.did_avalanche {
        self.move_history.push(
            space
                .try_into()
                .expect("Could not convert usize to u8 for move history"),
        );
    } else {
        self.did_avalanche = false;
    }

    self.move_count += 1;

    let mut hand = self.spaces[space];
    self.spaces[space] = 0;

    while hand > 0 {
        space += 1;
        if space >= 13 {
            space -= 13
        }
        self.spaces[space] += 1;
        hand -= 1;
    }

    if space == 0 && is_zero(&self.spaces[6..]) {
        return MoveResult::GameOver;
    } else if space == 0 {
        return MoveResult::FreeTurn;
    } else if self.spaces[space] > 1 {
        self.did_avalanche = true;
        return MoveResult::Avalanche(space);
    } else if is_zero(&self.spaces[6..]) {
        return MoveResult::GameOver;
    } else {
        return MoveResult::EmptySpace;
    }
}
```

### Time for more tests! {#testing-2}

```rust
#[test]
fn move_piece() {
    let mut board = MancalaBoard::default();
    board.move_piece(7);
    assert_eq!(board.spaces, [0, 4, 4, 4, 4, 4, 4, 0, 5, 5, 5, 5, 4]);
}
```

## Simulation - Let's get solving!! {#simulation}

Since we are going to be doing a lot of computation with data that resides on the stack, using a recursive approach will cause stack-overflows, so instead we need to run this simulation iteratively.

[Read more about the differences here](https://www.geeksforgeeks.org/difference-between-recursion-and-iteration/)

Create a function called `simulate`:

```rust
fn simulate(board: MancalaBoard, depth: u8) -> MancalaBoard {}
```

### Initialization of the stack {#initialization}

The first step is to create our "stack" which will be of type `Vec<MancalaBoard>` and will be used to store the boards of the initial 6 possible moves.

```rust
let mut initial_space_stack: Vec<MancalaBoard> = vec![];
```

Next we need to create a loop over the 6 player spaces:

```rust
for mut space in 7..=12 {

}
```

Inside of this loop we create another stack to hold all the boards being simulated, as well as a stack to hold the boards that have reached a completion state (game/turn over):

```rust
let mut stack: Vec<MancalaBoard> = vec![];
let mut final_stack: Vec<MancalaBoard> = vec![];
```

We are then going to clone our board inside of this loop so we can make moves without changing the original board that was passed to the function:

```rust
let mut board = board.clone();
```

This will error though, as we need to implement the clone trait ourselves:

```rust
impl Clone for MancalaBoard {
    fn clone(&self) -> Self {
        return MancalaBoard {
            spaces: self.spaces,
            opponent_store: self.opponent_store,
            move_history: self.move_history.clone(),
            move_count: self.move_count,
            did_avalanche: false,
        };
    }
}
```
### Main iteration {#iteration}

Now that everything is ready, we can start making moves! To be able to iterate over our stack, we need to initialize it by making the first move.

Add this to the bottom of the `simulate` function:

```rust
loop {
    match board.move_piece(space) { // Space is currently decided by the loop (the initial 6 player spaces)
        MoveResult::FreeTurn => { // Board can continue to be iterated on, so push it to the stack and break the loop
            stack.push(board);
            break;
        }
        MoveResult::EmptySpace => { // Board has reached a final state as the turn is over, push it to the final stack and break the loop
            final_stack.push(board);
            break;
        }
        MoveResult::Avalanche(new_space) => { // An avalanche has occurred, which means the simulation must continue until another state is reached, simply update the space and continue the loop
            space = new_space;
        }
        MoveResult::GameOver => { // Board has reached a final state since the game is done, push it to the final stack and break the loop
            final_stack.push(board);
            break;
        }
    }
}
```

Now our stack has been initialized, and we can start iteration. Create a loop that continues to iterate the stack until it is empty:

```rust
while stack.len() > 0 {}
```

This is where the main simulation cycles occur.

Start by popping a board off the top of the stack:

```rust
let stack_board = stack.pop().expect("Stack Empty");
```

We are now essentially going to recreate the same steps we took above inside of this loop:

```rust
for mut space in 7..=12 {
    let mut temp_board = stack_board.clone();
    loop {
        match temp_board.move_piece(space) {
            MoveResult::FreeTurn => {
                if temp_board.move_count >= depth { // This is the only difference, make sure we do not continue iterating past the depth
                    final_stack.push(temp_board);
                    break;
                }
                stack.push(temp_board);
                break;
            }
            MoveResult::EmptySpace => {
                final_stack.push(temp_board);
                break;
            }
            MoveResult::Avalanche(new_space) => {
                space = new_space;
            }
            MoveResult::GameOver => {
                final_stack.push(temp_board);
                break;
            }
        }
    }
}
```

You should be able to see how they are doing essentially the same thing.

This will now iterate over each board in the stack, duplicating it, and making a move, and then returning that new board to the bottom of the stack. This cycle continues until every board has reached a final state, which will mean the stack is empty. We now have a final stack with all of our possible board moves calculated out.

### Comparisons {#comparisons}

Now that we have finished the simulation, we need to return the highest scoring board (highest player store count). To do this we will iterate over the final stack and compare the scores, only keeping the higher scoring of the two until none are left.

Initialize the comparison with the first board on the final stack (place this below the `while stack.len() > 0` loop):

```rust
let mut top_board = final_stack.pop().unwrap();
```

Now we can start comparisons:

```rust
final_stack.into_iter().for_each(|final_board| {
    if final_board.spaces[0] > top_board.spaces[0] {
        top_board = final_board;
    };
});
```

This will leave us with `top_board` containing the highest score for the initial move. We will then push this board onto the initial stack, where the other first 5 moves will be:

```rust
initial_space_stack.push(top_board);
```

Now we need to compare the boards of the first 6 initial moves (place this below the outer `for mut space in 7..=12` loop):

```rust
let mut final_board = initial_space_stack.pop().unwrap();
initial_space_stack.into_iter().for_each(|board| {
    if board.spaces[0] > final_board.spaces[0] {
        final_board = board;
    }
});
```

This is just doing the exact same thing as above. 

### Returning {#returning}
`final_board` now contains the highest scoring board for the given layout!!!

Go ahead and return that value:

```rust
return final_board;
```

Your `simulate` function should now look like this:

```rust
fn simulate(board: MancalaBoard, depth: u8) -> MancalaBoard {
    let mut initial_space_stack: Vec<MancalaBoard> = vec![];
    for mut space in 7..=12 {
        let mut stack: Vec<MancalaBoard> = vec![];
        let mut final_stack: Vec<MancalaBoard> = vec![];

        let mut board = board.clone();

        loop {
            match board.move_piece(space) {
                MoveResult::FreeTurn => {
                    stack.push(board);
                    break;
                }
                MoveResult::EmptySpace => {
                    final_stack.push(board);
                    break;
                }
                MoveResult::Avalanche(new_space) => {
                    space = new_space;
                }
                MoveResult::GameOver => {
                    final_stack.push(board);
                    break;
                }
            }
        }

        while stack.len() > 0 {
            let stack_board = stack.pop().expect("Stack Empty");

            for mut space in 7..=12 {
                let mut temp_board = stack_board.clone();
                loop {
                    match temp_board.move_piece(space) {
                        MoveResult::FreeTurn => {
                            if temp_board.move_count >= depth {
                                final_stack.push(temp_board);
                                break;
                            }
                            stack.push(temp_board);
                            break;
                        }
                        MoveResult::EmptySpace => {
                            final_stack.push(temp_board);
                            break;
                        }
                        MoveResult::Avalanche(new_space) => {
                            space = new_space;
                        }
                        MoveResult::GameOver => {
                            final_stack.push(temp_board);
                            break;
                        }
                    }
                }
            }
        }
        let mut top_board = final_stack.pop().unwrap();

        final_stack.into_iter().for_each(|final_board| {
            if final_board.spaces[0] > top_board.spaces[0] {
                top_board = final_board;
            };
        });
        initial_space_stack.push(top_board);
    }

    let mut final_board = initial_space_stack.pop().unwrap();
    initial_space_stack.into_iter().for_each(|board| {
        if board.spaces[0] > final_board.spaces[0] {
            final_board = board;
        }
    });
    return final_board;
}
```

You have now completed the simulation step!!

### More Testing??? {#testing-3}

Add this to your tests:

```rust
#[test]
fn solve_default_board() {
    let board = MancalaBoard::default();
    let solved_board = simulate(board, 100);
    assert_eq!(
        solved_board.move_history,
        [
            12, 9, 8, 11, 7, 7, 11, 11, 12, 7, 9, 10, 12, 8, 12, 12, 11, 8, 12, 8, 12, 10, 12,
            11, 12, 7
        ]
    );
}
```

## Output {#output}

Just for readability, we will create a function to print a board that looks like our diagram in the top of this post:

```rust
fn print_board(board: MancalaBoard) {
    println!(
        "[  ][{}][{}][{}][{}][{}][{}][  ]",
        board.spaces[12],
        board.spaces[11],
        board.spaces[10],
        board.spaces[9],
        board.spaces[8],
        board.spaces[7]
    );
    println!(
        "[{}]-------------------[{}]",
        board.spaces[0], board.opponent_store
    );
    println!(
        "[  ][{}][{}][{}][{}][{}][{}][  ]",
        board.spaces[1],
        board.spaces[2],
        board.spaces[3],
        board.spaces[4],
        board.spaces[5],
        board.spaces[6]
    );
    println!();
}
```

## Main {#main}

All that is left now is to run all of these functions when the program runs!

```rust
fn main() {
    let board = get_user_board();
    let solved_board = simulate(board, 100); // depth can be altered here

    println!("{:?}", solved_board.move_history); // Shows the player what moves to make
    print_board(solved_board);
}
```


## Done! {#done}
You've now successfully written a program that will give you the best move on any given turn of Mancala!

Go ahead and run it with `cargo run`, see what happens!

## Final Thoughts
I really enjoyed writing this program, and it turned out to be relatively simple! I originally took a recursive approach but had to refactor everything to be iterative once I started getting stack overflows. This project taught me iterative design in rust which is something super important to know. I hope this guide was helpful, and maybe you learned a bit as well. This is really my first time making a written explanation of any of my programs, let alone a step-by-step guide to recreating it, I am happy with how it turned out, but I know I still have a lot of room to improve.

If you have any comments/critiques feel free to message me on discord (p0rtl6)!

The repository for this project with the full program can be located [here](https://github.com/p0rtL6/mancala-solver)