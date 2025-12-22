This is the original project [idea](https://github.com/MartynStaalsen/project-closet/issues/2), copied here for convenience and context.
-------

A notional approach to teaching a bot to play Contract Rummy (a card game) using a bifurcated architecture: part classical, part reinforcement learning.

# what 
Contract Rummy is a family favorite card game. It involves rounds of play ("hands") where 3 to 5 players compete to build sets of same card value ("groups") and same-suit serial sequences ("runs"). On each turn, players make relatively simple decisions about where to draw from and what to discard in order to lay down a fixed (per hand) "contract" of groups and runs.

# how
Without getting too detailed about the rules here, here's how I'd like to approach building an AI to play rummy: 

## classical model
It should be _fairly_ straightforward to build a deterministic evaluation function to tell one with a given set of cards which is statistically best to discard. Something about graph search...

The thing is, this gets a little harder on the Drawing side, because it's a lot of work to check rounds ahead if you simulate every possible card you might draw. 

## reinforcement learning 
Boxes for possible moves, of which there are few (to draw or not)... Hands are short so fast to learn from... Play some bots against each other after classical model can provide basic direction.
