# Overview

This is a singleplayer matching game where there are "Cards" on the board and the user can select 2 to flip over and if their symbols are the same you have a match. The goal of the game is to clear the board of cards in as few turns as possible. The user specifies a card to select through the use of a simple coordinate system reminiscent to chess. I created this program in the Rust language so that I could dive into and learn the language.

[Software Demo Video](https://www.youtube.com/watch?v=LSeJ6zt8ne4&ab_channel=JohnEllefson)

# Development Environment

- Visual Studio Code
- Git / GitHub
- Rust
- Libraries such as:  std::io and rand::Rng 

# Useful Websites

- [VisualStudio.com](https://code.visualstudio.com/docs/languages/rust)
- [Rust-Lang.org](https://doc.rust-lang.org/rust-by-example/index.html)
- [Crates.io](https://crates.io/)
- [TheNewsStack.io](https://thenewstack.io/tutorial-import-libraries-of-rust-code-with-crates-io/)
- [LogRocket.com](https://blog.logrocket.com/fundamentals-for-using-structs-in-rust/)

# Future Work

- Instead of using simple characters, use emojis to represent the cards and their respective symbols on the board.
- Allow the user to specify the dimensions of the board before starting a game which could increase the difficulty of the game with additional cards and symbols to match.
- Implement sound effects for flipping cards, and when a match is correct or incorrect.
- Implement a "Play Again" option at the end of each game.