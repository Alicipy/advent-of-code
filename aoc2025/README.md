# Advent of Code 2025

## About

My solutions for [Advent of Code 2025](https://adventofcode.com/2025).

Due to limited time this year, I decided to do it in Python instead of using a new language.

## Usage

You can install the project by calling `pip install .`,
then you can run a specific day `d` and part `p` via:

```shell
alicipy-aoc2025 -d 1 -p 2
```

This will read the input file `input_{:02d}` from `data` of the current
working directory and print the result of part 2.

## Project layout

```
src/
  aoc2025/
    __init__.py
    __main__.py
    main.py         # CLI entry point
    utils.py        # Argument parsing, input reading, helpers...
    day01/
      __init__.py
      main.py       # implements part_1() and part_2()
    ...
data/
  input_01          # input for day 01
  ...
```