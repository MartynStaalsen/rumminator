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
- it makes it easier to cycle players through different games each hand for tournament style play.
- it might make it easier to log, interrogate, and debug decision-making and general game flow.

So, the reasonable thing to do is to start with a plugin-style architecture, get that working, then swap to a distributed approach. I ought to be able to re-use most of the backend code anyway.

## Engine design
Let's think about the engine itself. Here's some elements we'll need:

### Cards
we need to be able to represent and manipulate cards. Cards have various properties that matter for set and bid creation:
- rank (i.e. value)
- suit
- value (score)
- human-readable designation (i.e. "Joker", "Queen of hearts" or "2 of clubs")
- concise designation (i.e. "J-", "QH", "2C")

There's some room in implementation approach here: 
- we could have cards be implementations of a base Card class, with methods to get or calculate i.e. score
- - an object-based approach may be convenient later for fancy set-building operations where sophisticated data-structure patterns might befit complex set searching & creation.
- all possible cards could be enums, with static lookup functions to get properties.
- cards could be their concise designation itself (again, functions for the property lookup)
- - this is advantageously simple for a distributed approach, where passing the card text IS passing the card.


#### Wilds
Already tho, we've got a complication. Joker's have no suit, and neither they nor 2's have a fixed rank. We can use "None" or "-" for these card's suit and/or rank. But we'll find that these deviants are gonna be the bane of our existence if/when we get around to set building & searching algorithms.

### Sets

#### Set-as-container
A set of cards is both an abstract and a physical thing. Physical in the sense that cards on the table are only allowed there if they are contained within exactly one valid bid set. Abstract in the sense that cards in a player's hand are simultaneously in multiple bid candidates.

Building sets on the table and building sets in the hand are almost the same problem. So to the extent we allow/provide set building logic to the players themselves, it will make sense to re-use this logic. Naively, that suggests that set-as-container is only a viable pattern for table representation. 

But, I think set-as-container is troublesome for on-table representation as well. That's because the on-table manipulation phase starts and ends with valid sets, but may have broken bids while cards are being manipulated.

This points me towards the following implementation approach:
Card containers should be a thing, but "set"ness maybe ought to be the result of a validation query rather than object construction. 
A hybrid approach may still make sense: CardContainer as a base class, of which Run and Set are sub-classes. In this scenario, on-table play would look like a player proposing a new configuration of Runs and Sets which would be validated before turn-end.

### Moves
on-table moves might be the hardest thing to implement. We can think of these as steps or transformations in table-state space (edges in a graph of all possible table states). But while this representation is convenient for searching, it has some issues:
- "illegal" moves may lead to "legal" bid configurations. That is, you might have to break a set along the way to making a new valid one.
- If you include illegal moves, the graph gets super dense, making blind searching potentially very costly. 

This is a whole mess. But, it's the player's job to figure out! The Engine only needs to be able to validate if a proposed new configuration is legal, not the moves needed to get there. So, I propose that all the actual move logic will rest on the Player side, with the engine simply a "can you make the table look like this please" suggestions which it will either accept or reject. 
Note: rejection here is intended as a game-control mechanism, but would probably be inefficient as a reasoning aid.

# Proposed implementation
So, here's a way to attempt this:

## Engine first
First off, the engine will be the primary priority. We'll consider Player architecture in the design, but the goal is to first build a complete working engine before actually implementing any player code.

### Validator, not facilitator
We'll implement the engine as a state holder and representer, but not as an actual transformer of state except for the basic functions of drawing & discarding. Instead, actual table manipulation will be implemented using a proposal system, where players will propose a whole-table state which the engine accept and update itself to match (if valid). This will cause problems later, but it's simple enough for now.

### State representation scheme
The engine will represent all elements of game state, including on-table sets, each player's hand contents, and draw/discard decks. It will need to be able to provide parts of this state as appropriate to each player (only give a player visibility to _their_ hand etc), but that shouldn't be too hard.

### Focused Responsibility
The engine is strictly about representing and managing game state. For the initial implementation, we will NOT consider re-use of any of its logic or representation ability for Player use. I'm doing this for two reasons:
- disconnect implementation time for engine & player. Success for this project _could_ look like a working engine with now players, initially
- allow the ability for competing player implementations: multiple developers could build their own Player models simultaneously, without the complexity & conflict of interest created by the possibility of modifying Engine code to suit such implementations.

# Implementation Plan: Engine
Let's start easy. The engine will be constructed from N (3-4) references to Player plugins. We'll use a "Player" trait to facilitate some method calls to each player on their turn. 
At construction, the engine will initialize the game, which will include spawning a random initial deck (shuffling), dealing dealing hands, and placing the remaining cards in two stacks for the draw and discard pile.

Now, play has begun. The engine will first have to figure out if anyone wants the discard, while also giving the first-of-turn the opportunity to take it as their draw. This will necessitate some kind of "i would like to draw that discard, if that's ok" negotiation between players -- either a pre-poll of the NOT to access if they want the discard, or a pre-nunu check where players can bid if they'd like the discard, only to be denied if they are preempted. The later provides more chance to gather intel on opponents, so we'll aim for that.

Now, the turn-proper begins. The requested draw target is added to the player's hand, and a game view is generated for that player and passed as a TURN notification to the NOT. This player will then get a game view (includes public game state plus their hand). The player will now have an opportunity to make moves thru a game-state bid which will include a Discard.

If, after any turn, a player has no cards remaining in their hand, the engine closes out, calculating a score for each player and exiting.
