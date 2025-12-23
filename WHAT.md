# What is Contract Rummy?

"Contract rummy" is a card game favorite among my extended family. It's probably called something else by others, but we've been playing the following for years:

# Big picture
Contract Rummy is a game about building sets of cards. Players use a starting "hand" of cards, to which they add by Drawing and diminish by Laying Down and Discarding. On a given "Hand" of play (not to be )

# PrePendix: Card set types
There are two types of card sets:
- **group**: 3 or more cards of the same "rank" (ordinal value, that is).  
- - **example**: 3 of clubs, 3 of spades, 3 of hearts
- - **example**: 5 of spades, 5 of spades, 5 of hearts
- - - doesn't matter if you've got multiple of the same suit.
- - **example**: King of diamonds, King of hearts, King of Clubs, King of Hearts
- - - at least 3, not exactly 3
- - **example**: 2 of spades, 4 of diabmonds, 4 of clubs
- - - 2 is wild here, standing in as a 4 of spades

- **run**: 4 or more cards of same suit which form a sequencial series.

- - **example**: 5, 6, 7, 8 of spades
- - **example**: Ace, 2, 3, 4 of hearts 
- - - Ace can be either a "1" or a "14", but can't cross over
- - **example**: 9, 2, Jack, Queen, King of diamonds 
- - - 2 is standing in as a 10

## Note: wilds
Jokers and 2's are both wild. A joker can be anything, while a 2 can only stand in for another card of it's rank. 
- a 2 of clubs can be any club, but can't participate in a Run of hearts

# Contracts
Each hand in a game has a "contract". That is, a specific set of groups and runs which each player must construct before as a part of laying down. Something like "two runs" or "a group and a run". There's a specific logic to which contracts are played when which is irrelevant here for now. 
Each player aims to fulfill the contract so that they can then play the remainder of their cards on other people's laid down sets. 
Note that only the sets stipulated by the contract can exist on the table at a given time: if the contract is two runs, you can't lay down a group even if you've fulfilled your contract already. 

## Standard Contract Progression
GG -- two groups -- 10 cards
GR -- one group, one run -- 10 cards
RR -- two runs -- 10 cards
GGG -- three groups -- 10 cards
GGR -- two groups, one run -- 12 cards
GRR -- one group, two runs -- 12 cards
RRR -- three runs -- 12 cards

Note: on the last hand, players must lay down and go out in a single hand -- there is no "gratuitous" table play.

# Winning, etc
A player wins a hand by laying down all their cards. Once this occurs, play immediately stops and each other player's remaining hand is tabulated to a score for that hand. Scoring is as follows:

## scoring
- Face cards & 10s: 10 points
- Aces & 2's: 20 points
- Jokers: 50 points
- all other cards: face value (i.e. 7 of clubs is worth 7 points).

# Order of play
# Setup
At the start of a hand,each player is dealt some starting hand of cards (10 or 12, depending on the contract). Play starts with the player to the dealer's left. 

## A Turn
The turn starts with a player drawing a card. They can then lay cards (if legal). Finally, then discard.

### Drawing
On their turn, a player take a card face-down from the Draw pile, or take a card face-up from the discard pile (assuming this card is "live"). 

### On-table
After drawing, a player has a chance to manipulate the table state. 

#### Laying down
A player must completely fulfill their contract before they can do anything else on-table: i.e. if the hand's contract is a pair of groups, they must lay down both complete groups. Also, the player must fulfill this contract in a turn, and before any additional manipulation. 
Let's call each implementation of a contract a "bid".

** Example **: The contract is a group & a run. Timmy lays down 3,4,5,6,7 of diamonds and 4 kings of various suits.


#### Gratuity
After laying down, (be that on the same turn or subsequent turns,) a player can play additional cards on on-table bids. This can involve all kinds of shennanigans, but follows these basic rules: 
- bids cannot be created or destroyed, they can only change forms. 
- once a card goes from a hand to the table, it can't go back into your hand. 
- on-table manipulation may "break" bids temporarily, but contracts must be complete at the end of the turn

## Discard
A player ends their turn by discarding a card. This discarded card can come from either the player's hand OR fom the table (this is typically reserved for situations where complex table manipulation has occured.)

## Out-of-turn draw ("Nunu")
After a player discards, all other players have the oppertunity to draw that discard, starting with the player to the left of the discarder (for whom this card is an alternate draw target). If a player nunus this card, they receive an additional face-down penalty card from the top of the deck. 
Once the next player draws to start their turn, this card is "dead", and is no longer a valid nunu target.

### Decisionmaking notes
In general, good most of Rummy is played in the discard phase.

Good rummy tactics comes down to two questions: what to collect, and what to discard. 
A naive strategy would be to always discard one's highest point-valued cards. A slightly more intelligent strategy involves looking for viable bids candidates in one's held cards, and selecting discards which maximize one's probability to lay down a complete bid on a subsequent turn. 

This decision-making will extend to nunu oppertunities, where a player will choose to draw out of turn if a discard is a helpful addition to a held bid candidate.

Additionally, players may observe the actions or laid bids of other players to guide their deciisionmaking. Noticing that an opponent is nunuing Kings on a hand with Groups in the contract might prompt a player to choose to discard a lower  card instead of their King, suspecting they may be able to play it later on their opponent's bid.

Finally, note that Nunuing for its own sake is a tactic some players will employ to increase their hand's card population. A sparse hand with few bid candidates may become more likely to be playable by the increased draw rate the penalty card provides, at the cost of more "trash" cards. This tactic is especially important when it can make the difference between being able to lay down a bid before an opponent plays out. 

By contrast, there are some niche scenarios where a player may wish to "play low" and hold low value cards without building bid candidates at all, if they think this is a better probable outcome for the game as a whole.