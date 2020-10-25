# TicTacoToe in Rust.

## Simple project for fun and learning Rust.

Tic Tac Toe is here implemented as a command line program.
The players need to specify the index of the field that they are setting their marker on.
The following is an exmaple of how the board is encoded into indices.

```
0 1 2
_|_|_
3 4 5
_|_|_
6 7 8
_|_|_
```

## If you are unfamiliar with Tic Tac Toe

There are two players, one marking 'X' and the other 'O'.
The win condition is if one of the players get three of their marks in a row or diagonally.

Example: 
Here the player who is 'X' won.

```
X|X|O
X|O|_ 
X|O|_
```

Player 1 will play as 'X', and Player 2 will play as 'O'.
