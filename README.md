# DotsAndBoxes
Challange each other in this adversarial classic. The game is explained in this Wikipedia page: [Dots and Boxes](https://en.wikipedia.org/wiki/Dots_and_Boxes). 

## How to run?
To run two random players against each other, simply issue: `cargo run`, which will execute the main function defined in `src/main.rs`. 

## How to add a player?

To add a player, one must implment the `Player` trait. A struct implementing this can then be passed along to the `run_game` function. 