package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

func main() {
	var part int
	flag.IntVar(&part, "part", 1, "Part 1 or 2")
	flag.Parse()
	data, sizes := parse_input()

	var result int
	if part == 1 {
		result = part1(data)
	} else {
		result = part2(data, sizes)
	}
	fmt.Print("Result: ", result)
}

func part1(data []map[string]int) int {
	total := 0
	for _, group := range data {
		total += len(group)
	}
	return total
}

func part2(data []map[string]int, group_sizes []int) int {
	total := 0
	for i, group := range data {
		group_total := 0
		for _, question := range group {
			if question == group_sizes[i] {
				group_total++
			}
		}
		total += group_total
	}
	return total
}

func parse_input() ([]map[string]int, []int) {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	var answers []map[string]int
	var sizes []int
	group := map[string]int{}
	group_size := 0

	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			for _, c := range line {
				group[string(c)]++
			}
			group_size++
		} else {
			answers = append(answers, group)
			sizes = append(sizes, group_size)
			group = map[string]int{}
			group_size = 0
		}
	}
	if len(group) > 0 {
		answers = append(answers, group)
		sizes = append(sizes, group_size)
	}

	return answers, sizes
}
