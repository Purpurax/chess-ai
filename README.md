next todo:
- There is a glitch in which: If you get checked, you have to uncheck
- queen upgrade

- fully block user interaction, if agent is playing
- game imports and exports (general chess notation)

- agent selection menu
- win/restart menu
- display captured pieces
- change outlines when agent plays, maybe a blue outline or some indication

- profile and optimize anything with `samply record ./target/release/chess-ai`

## Minimax
- parallel computing
- caching
- alpha beta border pruning

## MCTS
Should immediate captures in a simulation be values more than later captures?


## Neural Networks
- improve the games further until gpu training can be applied

## en pasant
This rule is not being considered as it complicates the code for no reason.

## 3D pixel art
This is more of a rabbit hole to do in plain rust. Choose Godot instead.
It is difficult as you want to render a pixel art image using 3d models.
- Find a good rust package to have UI and 3d rendering (ggez is not enough maybe). If you create your own pixel rendering, you can keep ggez
- Google whether the performance impact of 3d rendering is high on mobile devices
- Find a way to rotate the board
