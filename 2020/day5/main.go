package main

import (
	"bufio"
	"flag"
	"fmt"
	"math"
	"os"
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

func part1(data [][]string) int {
	var highest_id float64 = 0
	for _, seat := range data {
		var max_col float64 = 7
		var max_row float64 = 127
		var min_col float64 = 0
		var min_row float64 = 0
		for _, c := range seat {
			if max_row*8+7 < highest_id {
				break
			}
			switch c {
			case "F":
				max_row = math.Round((max_row + min_row) / 2)
				break
			case "B":
				min_row += math.Ceil((max_row - min_row) / 2)
				break
			case "L":
				max_col = math.Round((max_col + min_col) / 2)
				break
			case "R":
				min_col += math.Ceil((max_col - min_col) / 2)
			}
		}
		if min_row*8+min_col > highest_id {
			highest_id = min_row*8 + min_col
		}
	}
	return int(highest_id)
}

func part2(data [][]string) int {
	seat_ids := make(map[int]bool)
	for _, seat := range data {
		var max_col float64 = 7
		var max_row float64 = 127
		var min_col float64 = 0
		var min_row float64 = 0
		for _, c := range seat {
			switch c {
			case "F":
				max_row = math.Round((max_row + min_row) / 2)
				break
			case "B":
				min_row += math.Ceil((max_row - min_row) / 2)
				break
			case "L":
				max_col = math.Round((max_col + min_col) / 2)
				break
			case "R":
				min_col += math.Ceil((max_col - min_col) / 2)
			}
		}
		seat_ids[int(min_row*8+min_col)] = true
	}

	for id := range seat_ids {
		if seat_ids[id] && seat_ids[id+2] && !seat_ids[id+1] {
			return id + 1
		}
	}
	return -1
}

func parse_input() [][]string {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	var seats [][]string

	for scanner.Scan() {
		var current []string
		for _, letter := range scanner.Text() {
			current = append(current, string(letter))
		}
		seats = append(seats, current)
	}

	return seats
}
