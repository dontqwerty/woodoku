# woodoku
For now, a Rust implementation of the brain puzzle game Woodoku... soon an AI Woodoku solver

## The game

![Alt text](images/game.png?raw=true "The ")

The player is presented with a 9x9 board in which to place one of the 3 possible shapes on the side.

After placing a shape, the next one has to be placed.

Once there are no more shapes to place, 3 new random shapes are selected from the possible ones and presented
to the player and everything repeats.

A shape can only be placed on the board if it doesn't overlap with other already placed shapes.

Every time there is a "full" column, row or grid, the respective slots on the board are freed meaning that another shape could be placed there.
In this context full means that all the slots are filled by a shape.

At every move, the score is incremented based on a logic described in the code.

If none of the availables shapes can be placed on the board, the game ends.

### Shapes

Here an image showing all the shapes defined in [woodoku-lib/src/data/shapes.json](woodoku-lib/src/data/shapes.json) grouped by their size.

![Alt text](images/shapes.png?raw=true "Shapes")