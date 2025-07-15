# Chess AI
This is a Rust-based implementation of chess, featuring different AI agents that can be played against. In addition agents can play against each other aswell. The game can be run locally or through the web using WebAssembly.

## Agents
- **Neural Network**: The easiest agent turns out to be the neural network agent as not enough training is done to get good weights. The training works through evolutionary selection in which the currently best agent plays against a slightly mutated agent and the winner will be selected as the next best agent. While this approach is logically easy to implement and use, training against itself is not the most effective method.
- **Monte Carlo**: This agent is medium difficulty and uses the typical ucb-monte-carlo with a simulation depth of ~25. Normally the monte-carlo tries to complete a full game before performing back propagation, it is not feasable as the typical games only last around 15-40 moves. Improvements in the generation of possible moves will improve the agent, as well as using some heuristical policy in a simulation compared to the random policy.
- **Minimax**: The hardest agent uses the plain alpha-beta-minimax.

## Todo Features
- en pasant
- queen upgrade
- extensive testing
- Minimax parallel computing
- Minimax caching
- Minimax alpha beta interval
- Monte Carlo pseudorandom-heuristical policy in simulations
- Neural Network Training overhaul, train using real games instead of evolution

## WebAssembly Setup

1. **Prepare the Assets:**
   - Archive the assets folder into a file named `assets.tar`:
     ```
     tar -cvf assets.tar assets
     ```
   - On Windows, you can use the above command in a terminal that supports `tar`, such as Git Bash.

2. **Add the WebAssembly Target:**
```rustup target add wasm32-unknown-unknown```

3. **Build the Project:**
```cargo build --target wasm32-unknown-unknown --release```

If you do not use the `--release` flag, make sure the `index.html` file is set to load the `.wasm` file from the correct build directory.

4. **Start the Web Server:**
- Using `basic-http-server` from `cargo install basic-http-server`:
  ```
  basic-http-server .
  ```
- Or using `http-server` from `npm install http-server`:
  ```
  http-server
  ```

## Run Locally

To run the game as a standard desktop application simply use:
```cargo run --release```

## Profiling
Use `samply record ./target/release/chess-ai` to profile the code and find the most time consuming functions.
