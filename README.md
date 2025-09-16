# rusty-scape

A falling-blocks-type 2D game that challenges the player to avoid falling blocks to get the best score.

## About

Game-dev framework used is [ggez](https://ggez.rs/).

## Features

- **Menu System**: Clean title screen with instructions and high score display
- **Progressive Difficulty**: Game gets harder over time with faster falling blocks and increased spawn rate
- **Visual Feedback**: 
  - Player color changes when moving (blue for left, red for right)
  - Obstacles change color as difficulty increases
  - Particle explosion effects when player dies
- **Game States**: Proper menu, playing, and game over states
- **High Score Tracking**: Your best score is saved and displayed
- **Smooth Controls**: Responsive left/right movement with boundary checking

## Controls

- **Menu**: SPACE to start
- **Playing**: Left/Right arrow keys to move, ESC to return to menu
- **Game Over**: SPACE to restart, ESC to return to menu

## Game Mechanics

- Avoid the falling blue/purple blocks
- Score increases over time while you survive
- Every 50 points, the difficulty level increases
- Higher difficulty = faster falling blocks and more frequent spawning
- Try to beat your high score!

## Building

Requires Rust and system dependencies for audio/graphics:

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev pkg-config libudev-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev libxkbcommon-dev

# Build and run
cargo run
```
