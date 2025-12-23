# Implementation Strategy

In personal projects, it is more important to complete something than to make it perfect. To that end, I'm tracking the following strategy:

- complete the engine first: I'll build a functional rummy framework that supports a basic gameplay api. I'll focus on providing the ability to make moves over analysis of optimal moves.
- Aim for realism, but fallback to convenience: I'd love this system to support all edge cases and intricacies of real play, such as the variable player numbers, last-hand special rules, and whatnot. But a prudent approach suggests writing this up for the simple base case first
- ignore optimization initially: i'm gonna get a complete working solution first before I worry AT ALL about performance optimization. It hurts, but it works

## Player: Model Approach
Because of the complexity burden of decision making with history as a factor, I'm thinking about splitting the decision making process into components. For example, we might make a model for table manipulation and for bid recognition which learns and recommends moves to a higher level actor which makes the actual draw/discard decisions. This way, I've got options about how I implement each process. I could use a mathematically comprehensive board manipulator & bid creator, while allowing room for a more organic intelligence to learn competitive techniques and react to other players.

## Player/Engine interaction patterns

### distributed
I'm thinking I'll build the engine / player relationship using a server/client. The game server can provide an api which players generically use to interact with game state. This gets a little funky tho. The game hangs on each player's turn, so it would seemingly need to be the engine which posts to each player's api to say "its your turn, what move do you want to make". Similarly, each player would need to be notified about nunu opportunities, and about the changing board state. 

This could be flipped, with players pinging to discover if it's their turn and attempting actions accordingly. But that doesn't seem to provide an particular benefit

### Centralized/plugin
An alternative approach would be for each model to be implemented as plugins. I.e. implementations of a base "player" class. The Engine would directly call each player's various update and turn functions. 

Really, these models are the same, just communicating differently. In the first, the players are distributed processes communicating over IPC. This adds complexity due to the additional layer, but provides some useful benefits for future applications:
- it is convenient for the purposes of playing multiple model types against each other as the "configuration" is very run-time dynamic.
- it presents the ability for a human player to participate in games with the same interface 
- it makes it easier to cycle players through different games each hand for tournament style play