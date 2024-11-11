# Cee lo game

Outline
1. Accounts:

- Game State: Stores the current game state, active players, the current point, and bet information.


- Player State: Holds each player’s information, like their bet, roll result, and time they joined.

2. Functions:

- join_game: Allows a player to join the game by sending a bet.

- roll: Allows a player to roll dice and stores their result.

- call_push: If the player rolls the current point, all players must re-bet the initial amount to continue.

3. Randomness:

- Use the Switchboard Randomness Oracle to generate secure random numbers for dice rolls.

4. Timing:

- Leverage Solana’s Unix time clock to manage time limits for each roll.
