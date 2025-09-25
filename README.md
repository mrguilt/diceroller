# diceroller

This is a command line dice roller. Currently, it will only only roll one die so-many times. 

## Operation

The command line is:

```
diceroller CdS
```

Where:
    `C` Number of times to roll the dice
    `S` Number of sides on the dice

For instance:

	`dicecroller 2d20`

will roll a 20-sided die 2 times.

You can roll multiple dice at once. Typing

	`diceroller 2d20 1d6`

will roll a 20-sided die 2 times, and a 6-sided die once.

Other options:

```
	--help		This help information
	--version 	Version information
	--silent	Only prints results--no headers, roll count, etc.
	--csv		Creates an output to redirect to a CSV file (Excel import)
	--pretty	ASCII art dice
```