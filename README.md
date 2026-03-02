# WaterSort
Hobby attempt at solving the water sorting problem.

## The game
The goal of the game is to sort each color into its own glass.

https://www.reddit.com/r/watersortpuzzle/?rdt=51939

The rules are simple. Given source `i` and destination `j` glasses:

- `j` cannot be full
- if `j` is empty, `i` can be any non-empty glass
- if `j` contains any colors, it must match with the top color of `i`
- if possible, pouring from `i` pours as much consecutive units of liquid as possible. (e.g., if `i` is: [R, G, G, G] and `j` is: [B, G, _, _], then the result will be `i`: [R, G, _, _], `j`: [B, G, G, G]).

It is similar in nature to the [Tower of Hanoi](https://en.wikipedia.org/wiki/Tower_of_Hanoi) problem.

## Solver

The problem is modelled as a [state diagram](https://en.wikipedia.org/wiki/State_diagram) and graph search algorithms are used as a solver:

- Plain [Breadth-First-Search](https://en.wikipedia.org/wiki/Breadth-first_search) using a regular queue.
- A heuristic approach using a minimum priority queue (resembles [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)), except there are no guarantees for shortes path here. I guess it can be interpreted as some sort of A-star?

The BFS approach finds shortest paths, but takes considerably slower for smaller instances, and scales infeasibly worse as the number of colors grows. Meanwhile, the heuristic approach does not guarantee shortest paths (because the value I assign to each state is not universally leading to a better solution), but is much more likely to find one particular solution orders of magnitude faster which (usually) isn't "too far" from the best optimal solution.

## Explorer

If instead of directly solving we are interested in the "shape" of the solution space, it's possible to run the plain BFS to explore the solutions space. The results are written to a text file.

## Usage

To "quickly" (in most cases) solve a system of 8 colors:

```bash
cargo run -- --number-of-colors 8
```

To "find" (if possible) the best solution:

```bash
cargo run -- --number-of-colors 8 --search-method "BFS"
```

To just explore the solutions space:

```bash
cargo run -- --number-of-colors 8 --program-goal "Explore"
```

To explore a given input system and write the resulting search space into an output file:

```bash
cargo run -- --number-of-colors 8 --program-goal "Explore" --input-system "test_input.txt" --output-path "test_output.txt"
```

Using all the parameters cuz why not:
```bash
cargo run -- --number-of-colors 8 --program-goal "Explore" --input-system "test_input.txt" --output-path "test_output.txt" --random-seed 42 --heuristic-evaluation "ColorCounting" --color-scheme "HighContrast" --glass-size 4 --max-depth 255
```


