package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
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

func part1(data map[string]map[string]int) int {
	total := 0
	for bag := range data {
		if dfs(data, bag) {
			total++
		}
	}

	return total
}

func dfs(data map[string]map[string]int, entry string) bool {
	if data[entry]["shiny gold"] > 0 {
		return true
	}

	for bag := range data[entry] {
		if dfs(data, bag) {
			return true
		}
	}

	return false
}

func part2(data map[string]map[string]int) int {
	total := 0
	total += dfs_part2(data, "shiny gold", 1)
	return total
}

func dfs_part2(data map[string]map[string]int, entry string, mult int) int {
	contained := 0
	for bag, num := range data[entry] {
		contained += mult * num
		contained += mult * dfs_part2(data, bag, num)
	}
	return contained
}

func parse_input() map[string]map[string]int {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	regulations := map[string]map[string]int{}

	for scanner.Scan() {
		split := strings.Split(scanner.Text(), " contain ")
		color := split[0][:strings.Index(split[0], " bags")]
		regulations[color] = map[string]int{}

		for _, content := range strings.Split(split[1], ", ") {
			if content == "no other bags." {
				continue
			}
			parts := strings.Split(content, " ")
			regulations[color][parts[1]+" "+parts[2]], _ = strconv.Atoi(parts[0])
		}
	}

	return regulations
}
