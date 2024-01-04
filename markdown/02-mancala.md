# Solving Mancala in Rust **ATTENTION: THIS IS UNFINISHED**
### 12.27.23
---
This project was originally inspired by playing GamePigeon Mancala with my friend...

By the end of this post, you should have a fully working program to give you the most optimal move in your game of Mancala!

* This post assumes you have basic knowledge of the Rust Programming Language, there will be links to further reading where relevant if you need help with concepts. If you do not know Rust, start by reading the [Rust Book](https://doc.rust-lang.org/book/).

## Following the Gamepigeon rules (avalanche):
> 1. Each player has a store on one side of the board.
> 2. Players take turns choosing a pile from one of the holes. Moving counter-clockwise, stones from the selected pile are deposited in each of the following hole.
> 3. If you drop the last stone into an unempty hole, you will pick up the stones from that hole and continue depositing them counter-clockwise.
> 4. Your turn is over when you drop the last stone into an empty hole.
> 5. If you drop the last stone into your store - you get a free turn.
> 6. The game ends when all six holes on either side of the board are empty.
> The player with the most stones in their store wins.

## Our Mancala board

```md
---              Player Side                     ---
| Player [00][12][11][10][09][08][07][NA] Opponent |
| Store >[00]------------------------[NA]< Store   |
|        [00][01][02][03][04][05][06][NA]          |
---              Opponent Side                   ---
```
We will be numbering our board spaces, starting with our player "store", going down the opponent side, and then back through the player side. Making the store the first element will be helpful later.

(The opponent store is skipped because you do not move there)

## New game layout, and user input

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

### First Steps - Making a Board

To solve a game of Mancala, first we need the board. Let's make a board!

Go ahead and put this at the top of your `main.rs`:

```rust
struct MancalaBoard {
    spaces: [u8; 13],
    opponent_store: u8,
    move_history: Vec<u8>,
}
```

Read more about structs [here](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

We will represent our board spaces as an array of `u8` integers, 13 in length (6 spaces per side + player store). These are all the movable spaces, so they are the ones we have to calculate on. 

We also want to keep track of the opponent store just incase we want to do more with it later (displaying the board, etc.).

Lastly, we will keep a vector of `u8` to represent each move made on a board, this will be used to show the user at the end the proper moves to take.

### Creating a default board

Now that we have a board, we need a way to either: create a new game with default piece layout, or accept a user created board. We will start with the former.

Add this underneath the `MancalaBoard` struct:
```rust
impl Default for MancalaBoard {
    fn default() -> Self {
        MancalaBoard {
            spaces: [0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
            opponent_store: 0,
            move_history: vec![],
        }
    }
}
```

Here we are implementing the `Default` trait for our board. 

We will create an instance of our `MancalaBoard`, inside of which we will create a new array, starting with a 0 to represent our player store, and the rest filled with 4's (these are both the player, and opponent sides combined). We will also initialize our opponent store to be 0. Lastly, an empty vector will be created with the `vec![]` macro for our move history, preparing new moves to be added as we make them.

### Creating a new board from existing pieces

Add this underneath the `Default` impl:
```rust
impl MancalaBoard {
    fn new(spaces: [u8; 13], opponent_store: u8) -> MancalaBoard {
        return MancalaBoard {
            spaces,
            opponent_store,
            move_history: vec![],
        };
    }
}
```

Here we are just offering a simple way to create a new board from existing pieces. Doing this will allow us to initialize the move history seperately so we don't have to make it ourselves everytime we need a new board.

### Testing!!

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

### Moving pieces

Now that we can create boards, we need a way to make moves.

Create a new function inside of the `impl MancalaBoard`:

```rust
fn move_piece(&mut self, space: usize) -> MoveResult {}
```

Before we get into the actual movement, we need to define the outcomes of our moves, we will do this with an enum named `MoveResult`