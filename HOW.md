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

# Player
I imagine building a couple different "classical" algorightms (i.e. always discard your lowest card) in addition to more exciting ML approaches. That means the Player implementation should be modular, or something.
