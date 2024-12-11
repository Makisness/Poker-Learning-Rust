**Simplified Commandline Poker**
This is a project I started to teach myself rust.
Note: I did not know how to play poker before this.
Chess has been a bit over done, and poker seemed like it may be a bit easier to create.

**Game:**
- 2 player texas holdem
- no betting
- there is no "flop", community cards are revealed 1 by 1.
- win probabilities for each player are calculated after hands are delt and after each community card is delt.

**Implementation:**

Cards, Hands, and Deck:
cards, hands, and decks are Vec<u8>
each card is a u8 0-51, this corresponds to it's position in a sorted deck.

0 = Ace of Spades
1  = 2 of Spades
2 = 3 of Spades
...

this allows the suit and rank to be revealed via an Enum that
does integer division by 13 for the suit
and mod 13 for the rank

this is combined into a Card struct that handles this data
although this struct is primarily used for printing to the screen as the 
Vec<u8> is more efficent for actual game logic computations.


Shuffle Algorithm:
The deck is created with an interesting anti-sorting algoithm (shuffle algorithm).
Im using the fisher-yates shuffle algorithm
it takes a sorted vec of numbers 1-52

(1,2,3,4,...,51,52)

then loops through each item and swaps it with another random item inside the vector.
this is ideal since no new numbers are being created I can be sure that there are no duplicates,
additionally it works with vectors of any length which is helpful for my monte carlo simulation used for calculating win probabilities.

Although this may be less efficent since it requires the creation and shuffling of 52 items inside the vector when the
majority of poker games are played with far fewer cards.
but I do enjoy how it somewhat resembles the realworld act of shuffling a deck of cards (sorta..)


Game Logic:
the game logic was surprisingly the least interesting implementation.
the majority is just parsing the u8's and matching them to the types of hands.
Then returning anther u8 based on the quality of that hand.
10 = royal flush
9 = straight flush
...
1 = high card

then some additional logic that handles if both players have the same u8 that represents the hand quality, this handles all the edge case conditions if both players have the same hand pattern
for example
had a full house. (the player with the higher ranking three-of-a-kind would win)

this implementation was aided with a numbher of helper functions that check if there are multiples of a suits or ranks, or if there are consecutive ranks.


Win Probability Calculation:
The win probability is currentyly being calculated via a monte carlo simulation.
this takes in the current player hands and the revealed community cards and simulates an arbitratry number of games.
the games will simulate different random possibilities of the unknown community cards. 
(this is where the fisher yates shuffle is useful, since I can take the remaining deck which already has the hands and revealed community cards removed and get a random permutaion of the deck quickly)
then the wins are calculated for every simulated outcome and the wins are divided by the total number of games (and multiplied by 100) to achieve the win probability for each player.
the player win probabiliities will not always add to 100% since there are rare games where the players can truely tie and this can happen in the simulations meaning that there are some games that neither player wins.

The exact accuracy of my monte carlo implementation is TBD.

to achieve the exact accuarcy I can run the same simulation with every combination of cards rather than a random combination of cards. this will produce (47 choose 5) combinations (for 2 players).
52 - 2*(number of players) and 5 community cards
equal to 1,533,939 games that need to be evaluated. (a lot but not unreasonable)
This will give an exact number of how many games a player can win out of all possible games that can be played with p1, and p2's hands.
This can give a benchmark that I can compare my monte carlo simulation against.






