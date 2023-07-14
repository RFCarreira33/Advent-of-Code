package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"sort"
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
	jolt1, jolt3 := 0, 1
	for i := 0; i < len(data)-1; i++ {
		switch data[i+1] - data[i] {
		case 1:
			jolt1++
			break
		case 3:
			jolt3++
		}
	}

	return jolt1 * jolt3
}

func part2(data []int) int {
	var visited = map[int]int{}
	return recursive(data, 0, visited)
}

func recursive(data []int, start int, visited map[int]int) int {
	if len(data) == 0 {
		return 1
	}
	if val, ok := visited[start]; ok {
		return val
	} else {
		visited[start] = 0
	}

	count := 0
	for i, num := range data {
		if num-start <= 3 {
			count += recursive(data[i+1:], num, visited)
		}
	}
	visited[start] = count
	return count
}

func parse_input() []int {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	numbers := []int{0}

	for scanner.Scan() {
		number, err := strconv.Atoi(scanner.Text())
		if err == nil {
			numbers = append(numbers, number)
		}
	}

	sort.Ints(numbers)

	return numbers
}
