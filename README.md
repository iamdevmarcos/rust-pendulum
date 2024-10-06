# Rust Pendulum

This project is a simple 2D pendulum simulation using the `rust` and `speedy2d` crate for rendering. The simulation creates two pendulums that swing from a common origin and are displayed on a window. Each pendulum is modeled by its length, mass, and gravitational force.

## Features

- **Double Pendulum Simulation**: The simulation consists of two pendulums, each with independent properties.
- **Dynamic Physics**: The pendulums exhibit motion governed by basic physics principles such as angular velocity and angular acceleration.
- **Real-Time Rendering**: Uses `speedy2d` for efficient 2D rendering.
- **Customizable Pendulums**: Change pendulum properties like length, mass, and gravity to alter the simulation behavior.

## Dependencies

This project requires the following Rust crates:

- `speedy2d`: A fast and easy-to-use 2D graphics library for rendering.
- `math`: The project uses basic vector math to model pendulum motion (custom `vector` module).

Make sure to add the necessary dependencies to your `Cargo.toml` file:

```toml
[dependencies]
speedy2d = "0.8"
```

## Modules

### 1. `main.rs`
This file sets up the window and runs the pendulum simulation loop:

- **Window Setup**: A window is created using `speedy2d::Window::new_centered()`.
- **Pendulums**: Two pendulums are created with different lengths, both attached at the same origin.
- **Event Loop**: The `window.run_loop()` starts the main loop, which continuously updates and renders the pendulums.

### 2. `pendulum.rs`
This module defines the `Pendulum` struct and its methods for updating and rendering:

- **`new`**: Constructor to create a new pendulum with specific origin, position, and length.
- **`update`**: Calculates the angular acceleration, updates angular velocity and position.
- **`draw`**: Draws the pendulum using lines and circles for the arm and the pendulum's bob.

### 3. `window_handler.rs`
This module defines `MyWindowHandler` which handles window events, including rendering and updating the pendulum's state:

- **`on_draw`**: Called every frame to clear the screen, update pendulum positions, and render them.

### 4. `vector.rs`
A custom module defining a basic 2D vector used for pendulum position and movement calculations.

## How to Run

1. Clone the repository.
2. Ensure that you have Rust installed: https://www.rust-lang.org/tools/install.
3. Build and run the project:

```bash
cargo run
```

## Customization

- **Pendulum Length**: Modify the length of the pendulum by adjusting the `r` parameter in the `Pendulum::new()` method.
- **Gravity**: The gravitational constant `g` can be modified within the `Pendulum` struct.
- **Mass**: Although mass (`m`) is part of the pendulum struct, it is currently not used in calculations. Feel free to modify the physics by incorporating the mass in the equations if desired.

## Preview

![Screenshot 2024-10-06 at 17 46 45](https://github.com/user-attachments/assets/2edb9e37-3998-4cbe-8cea-db102c5a9e2a)
