package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strings"
)

type Input struct {
	Min    int
	Max    int
	Letter string
	Pass   string
}

func main() {
	var part int
	flag.IntVar(&part, "part", 1, "Part 1 or 2")
	flag.Parse()
	data := parse_input()

	var result int
	if part == 1 {
		result = part1(data)
	} else {
		result = part2(data)
	}
	fmt.Print("Result: ", result)
}

func part1(data []Input) int {
	var total int = 0
	for _, input := range data {
		count := strings.Count(input.Pass, string(input.Letter))
		if count >= input.Min && count <= input.Max {
			total++
		}
	}
	return total
}

func part2(data []Input) int {
	var total int = 0
	for _, input := range data {
		// XOR
		if (string(input.Pass[input.Min-1]) == input.Letter) != (string(input.Pass[input.Max-1]) == input.Letter) {
			total++
		}
	}
	return total
}

func parse_input() []Input {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	inputs := []Input{}

	for scanner.Scan() {
		var parsed Input
		_, err := fmt.Sscanf(scanner.Text(), "%d-%d %1s: %s", &parsed.Min, &parsed.Max, &parsed.Letter, &parsed.Pass)
		if err != nil {
			panic("Error: Converting to valid Input")
		}

		inputs = append(inputs, parsed)
	}

	return inputs
}
