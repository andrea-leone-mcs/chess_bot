# chess_bot
## Brief Description
In this repository I'm storing my personal implementation of a chess bot.
This project is done just for fun (I enjoy chess & programming, so why not doing something which relates both of them?) without the intention of replacing any current chess bot or chess-programming best practice.
The goals of this project are:
- Practicing with Rust language;
- Being able to develop a GUI application written in Rust using the **gtk4** library.;
- Enhancing problem solving skills by facing challenging problems which will naturally come up while programming a chess bot.

## How to run
... TODO ...

## Project structure
### Phase 1 - GUI
The first phase of the project was very easy and consisted in building the graphical interface of the chess board. This choice was made in order to always be able to visualize some output for the next phases.
- The GUI is built using the **gtk4** library and uses a **Grid** widget to represent the chess board. Each chell of the grid is a **GtkButton** widget which in this phase is not doing anything.
- The chess board was styled using CSS, and the pieces images are loaded from the apposite PNG files.
- In this phase the chess board is displayed as empty, with no pieces on it.

### Phase 2 - Piece & Board
The second phase consisted in just programming the basics of pieces and board.
- The color, type and position where defined for the pieces.
- The board was defined as a 2D array of optional piece values, and other values about the game status are also stored.
- A function to load a chessboard configuration from a FEN string was implemented. This is used to create the initial chessboard configuration easily and can become usefull in the future to load a specific board configuration.
- A function which displays the board on the Grid widget was implemented. This function should be called every time the board is updated.

### Phase 3 - Movements
The third phase of the project consisted in implementing the movements of the pieces. This was done by implementing a method for the **Piece** struct which returns a vector of possible movements for the piece. This already takes into account en-passant, castling, and the fact that pinned pieces cannot move in a way which would expose the king to a check. However, it doesn't take into account the fact that if the king is in check, the piece must move in a way such that the king won't be in check in the resulting position. This last thing has been implemented in a very _dumb_ way, by just rollbacking a move if the king is in check after the move. This is not a good solution, but it's a start.\
- Some function to generate moves for the pieces have been implemented.
- Some function that check for pinned pieces, attacks, etc. have been implemented.
- A function to run a random game has been implemented.
- Still missing: checkmate and draw conditions (not necessary at the moment).

### (Next) Phase 4 - Engine
The next phase will consist in implementing a basic chess engine able to analyze a position and understanding which is the best move to do.