# TODO features:

- pagination module. Some kind of data structure that wraps around a message and handles all the reaction handling, i.e. by creating listeners for the reaction event and destroying them when the message is deleted or reactions are disabled. The listeners should be in a list on the client/context that the fn reaction_add/reaction_remove events iterate through/ or rather find the listener for the particular message in the list and let it handle the reaction.

- Know Your Meme search
- translation command

- fetch command
  - make an embed when executing the command that tells you up to what point it will fetch (if given), and a time estimate
  - make it intelligently fetch up to a larger block of images or messages, when no arguments are given
  - make it support a start and end point to fetch older intervals
