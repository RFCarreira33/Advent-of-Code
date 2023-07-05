package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strconv"
)

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

func part1(data []int) int {
	for i := 0; i < len(data); i++ {
		for j := i + 1; j < len(data); j++ {
			if data[i]+data[j] == 2020 {
				return data[i] * data[j]
			}
		}
	}
	return -1
}

func part2(data []int) int {
	for i := 0; i < len(data); i++ {
		for j := i + 1; j < len(data); j++ {
			for x := i + 2; x < len(data); x++ {
				if data[i]+data[j]+data[x] == 2020 {
					return data[i] * data[j] * data[x]
				}
			}
		}
	}
	return -1
}

func parse_input() []int {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	numbers := []int{}

	for scanner.Scan() {
		number, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic("Error: Converting to Int")
		}

		numbers = append(numbers, number)
	}

	return numbers
}
