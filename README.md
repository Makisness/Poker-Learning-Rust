**Simplified Commandline Poker**

This is a project I started to teach myself rust.
Note: I did not know how to play poker before this.
Chess has been a bit overdone, and poker seemed like it may be a bit easier to create.

**Game:**
- 2-player Texas hold-em
- no betting
- there is no "flop", community cards are revealed 1 by 1.
-  the win probabilities for each player are calculated after hands are dealt and after each community card is dealt.
-  The observer/player progresses the game by pressing "enter" to reveal each community card

**Cards, Hands, and Deck:**

Hands and Decks are represented as a Vec of u8s

each card is a u8 0-51, which corresponds to its position in a sorted deck.


0 = Ace of Spades

1  = 2 of Spades

2 = 3 of Spades

...

this allows the suit and rank to be revealed via an Enum that
does integer division by 13 for the suit
and mod 13 for the rank

this is combined into a Card struct that handles this data
although this struct is primarily used for printing to the screen as the 
Vec of u8s is more efficient for actual game logic computations.


**Shuffle Algorithm:**

The deck is created with an interesting anti-sorting algorithm (shuffle algorithm).
I'm using the fisher-yates shuffle algorithm
which takes a sorted vec of numbers 1-52

(1,2,3,4,...,51,52)

then loops through each item and swaps it with another random item inside the vector.
this is ideal since no new numbers are being created I can be sure that there are no duplicates,
additionally the nature of this algorithm is helpful for my Monte Carlo simulation used for calculating win probabilities, primarily because it can randomize the positions of existing values in a vector.
This means not having to do any complicated checks for duplicates or keeping track of cards removed from the deck.

However, this does come a slight tradeoff since it requires the creation and shuffling of 52 items inside the vector when the
majority of poker games are played with far fewer cards.


**Game Logic:**

the game logic was surprisingly the least interesting implementation.
the majority is just parsing the U8s and matching them to the types of hands.
Then returning another u8 based on the quality of that hand.

10 = royal flush

9 = straight flush

...

1 = high card

then some additional logic that handles if both players have the same u8 that represents the hand quality, this handles all the edge case conditions if both players have the same hand pattern
for example
had a full house. (the player with the higher ranking three-of-a-kind would win)

this implementation was aided with several helper functions that check if there are multiples of suits or ranks, or if there are consecutive ranks.


**Win Probability Calculation:**

The win probability is currently being calculated via a Monte Carlo simulation.
this takes in the current player's hands and the revealed community cards and simulates an arbitrary number of games.
the games will simulate different random possibilities of the unknown community cards. 
(this is where the Fisher-Yates shuffle is useful since I can take the remaining deck which already has the hands and revealed community cards removed, and get a random permutation of the deck quickly)
then the wins are calculated for every simulated outcome and the wins are divided by the total number of games (and multiplied by 100) to achieve the win probability for each player.
the player win probability will not always add to 100% since there are rare games where the players can truly tie and this can happen in the simulations meaning that there are some games in which neither player wins.

The exact accuracy of my Monte Carlo implementation is TBD.

to achieve the exact accuracy I can run the same simulation with every combination of cards rather than a random combination of cards. this will produce (47 choose 5) combinations (for 2 players).
52 - 2*(number of players) and 5 community cards
equal to 1,533,939 games that need to be evaluated. (a lot but not unreasonable)
This will give an exact number of how many games a player can win out of all possible games that can be played with p1, and p2's hands.
This can give a benchmark that I can compare my Monte Carlo simulation against.

**Future:**

I believe there is still plenty to explore in Rust and Poker.

Some ideas for additional updates and implementations
beyond refactoring:

- Implement the true win probability algorithm and then use it to benchmark my Monte Carlo simulation.
- Add a variable number of players. (this will require more refactoring than I'd like to admit..)
- Add a playing character with the implementation of betting.
- Add some ML implementation (recommended bet calculator?)
- GUI - maybe some kind of proper GUI implementation, or just a simple ASCII command line GUI






