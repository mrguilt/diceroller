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

The output would be:

```
Rolling 2d20
	Roll #1: 2
	Roll #2: 8
Total for 2d20: 10
```

*Obviously, the rolls (and total) will be different each run.*

You can roll multiple dice at once. Typing

	`diceroller 2d20 1d6`

will roll a 20-sided die 2 times, and a 6-sided die once.

```
Rolling 2d20
	Roll #1: 14
	Roll #2: 3
Total for 2d20: 17

Rolling 2d6
	Roll #1: 3
	Roll #2: 3
Total for 2d6: 6
```

## Output Options

### Silent

`--silent` option will output the results without any headers, roll count, etc. 

```
% diceroller 2d20 --silent
12
10
```

If multiple rolls are selected, there will be a line break between each group:

```
% diceroller 2d20 2d6 --silent
12
12

1
2
```

### CSV

`--csv` option creates an output of comma separated values (CSV), to input into other tools (like Microsoft Excel). The first item will be what's being rolled, followed by the results. 

```
% diceroller 2d20 2d6 --csv
2d20,13,19
2d6,6,1
```
### Pretty

`--pretty` option generates ASCII art dice. No group headers are provided (I'd recommend doing just one set of rolls).

```
% diceroller 2d20 --pretty
 _______
/       \
|       |
|   2   |
|       |
\_______/
 _______
/       \
|       |
|   15  |
|       |
\_______/
```

## Other Options


|Option      |Function                |
|------------|------------------------|
|`--help`    |Help text               |
|`--version` |Version information     |
