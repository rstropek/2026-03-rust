* Array because number of fields are fixed (3x3 = 9)
* Represent each field
  * a) Using an enum with three variants: X, O, Empty
  * b) Using Option<enum with X/O> 🦀 - this is what we will do to practice Option
* `Coordinate` struct
  * `Copy`, `Clone`
  * Once constructed, `Coordinate` is guaranteed to be valid (row and column are within bounds)
  * row: enum with 1, 2, 3
  * column: enum with A, B, C
  * Contains logic to parse e.g. "A1"- `FromStr`
* `Game` struct
  * Holds the board data (array)
  * Holds the current player (enum with X/O)
  * `check_winner` method
    * Input: `&self`
    * Output: `Option<enum with X/O>`
  * Implements `Display` so we can e.g. print the board with `println!`
  * `make_move` method
    * Input: `&mut self`, `Coordinate`
    * Output: Result<`Option<enum with X/O>` /* winner */, enum with AlreadyOccupied, AlreadyWinner>
    * Algorithm:
      1. Check error conditions (field empty? game already won?), return if error
      2. Make the move
      3. Update the current player
      4. Return the (optional) winner
* `main` function
  * Handles entire user interface (input, output from/to console)
  * Algorithm in pseude code:
    
    ```
    Initialize everything, create board (singleton)
    Loop until game is over:
      Print the board
      Ask current player for input
      Parse input into Coordinate
      If parsing fails, print error and continue at the beginning of the loop
      Call make_move with the coordinate
      If make_move returns an error, print error and continue
      If make_move returns a winner, print winner and break loop
    Print final board state
    ```

* Modules:
  * `tictactoe` module with business logic, separate file
  * `main` module with user interface logic (console interaction, main game loop)
