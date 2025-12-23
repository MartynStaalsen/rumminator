# How will I teach a bot to play Contract Rummy?

I've got a two-part scheme in mind. First of all, we need the foundational playing framework. That is, we need an way for an agent to participate in a game of rummy. We'll call this the "Game Engine". Once that exists, we'll build the second part (the Player).

# Engine
The engine is about owning the game state, and providing the ability to query & act on that game state. Acting on the state is done via Moves. 

## Objectives
- **fidelity**: obviously, the engine must accurately simulate the Game to agents. That means correctly translating their Moves into Game State change.
- **RL-able**: the engine must be suitble for the purpose of teaching a bot to play this game via Reinforcement learning. That implies a couple things:
-- **performative**: RL will take a lot of iterations to train, presumably. The engine should be computational efficiency conscious. This is the lowest priority tho, since implementing _something_ is vastly more important than implementing the best thing. Enemy of good and all that.
-- **multi-agent**: The engine should be designed for multiple agents to play at a time. Contract Rummy is a game for 3-4 players, and all of these should be able to be distinct models.

I'll add additional _discovered_ objectives for the engine as prototyping uncovers them.

## Making Moves
As I discuss below, it's still up in the air if the engine should provide the list of all legal moves to players. In human play, Move Discovery (in the laying down phase at least) is a huge component of what differentiates good and bad players. So having Players themselves provide moves in a more open-ended format than just selecting off a buffet would be best. Hoewver, the Engine needs at least to be able to validate if an operation is legal. For now, i'm imagining that the engine supports the ability for a player to request to make an operation, for the engine to reject the action if it's invalid. The details of this will undoubtedly evolve a lot.

# Player
I imagine building a couple different "classical" algorightms (i.e. always discard your lowest card) in addition to more exciting ML approaches. That means the Player implementation should be modular, or something.

Fundamentally, a player model, whether classical or fancier, is a box that does the following:
- selects "moves" to make based on current information. For Rummy, these moves will occur both on and between a player's turn.
- injest information about the game state. We'll assume for now that all Players have perfect memory about the public game state (i.e. what cards have been discarded, and which have been nunu'd, and by whom)
That suggests that the player model is a system that produces per-turn output based on two inputs:

## input: public state
This is anything all players know. Includes, in order of ease-of-use:
- current contract
- card sets cards on the table
- number of cards in each player's hand
- record of discard & nunu actions by other players
- 

A naive model & implementation will only consider the easiest of these to RL against -- that is, those which are easy to compact into generic state. Meaning, if you only look at instantaneous data instead of considering historical data, it'll be much easier to compare that learnings from similar game states. I've got some statistical ideas about how to get around this tho.

## input: private state
Cards in the player's hand. Simple 

## output: move selection
A player can make moves on their turn, but also has the opportunity to draw out of turn. In my mind, it seems that the legal moves in a given situation might be considered an additional input to the Player. Providing this for free to the player does limit the opportunity for player ingenuity, but _might_ be necessary for an RL approach. I'll keep noodling on this. Maybe models can include a sub-component that discovers moves. 

## Bonus Information
- Players might also find it useful to know what their total score is in a game (might inform riskier/more conservative play)
- historical performance of opponents would let one make assumptions about what schemes they might employ, or failures they might be susceptible to.