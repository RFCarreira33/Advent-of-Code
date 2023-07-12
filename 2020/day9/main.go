package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strconv"
)

const (
	T_preamble = 5
	I_preamble = 25
	// hardcoded I dont wanna be running both parts to get suspects
	T_suspect = 127
	I_suspect = 1398413738
)

var MAX int

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
	for i := I_preamble; i < len(data); i++ {
		suspect := data[i]
		slice := data[i-I_preamble : i]
		valid := false
		for list_i, num_a := range slice[:len(slice)-1] {
			if valid {
				break
			}
			for _, num_b := range slice[list_i+1:] {
				if num_a+num_b == suspect {
					valid = true
					break
				}
			}
		}
		if !valid {
			return suspect
		}
	}
	return -1
}

func part2(data []int) int {
	for i := 0; i < len(data); i++ {
		max, min := 0, data[len(data)-1]
		total := 0
		for _, num := range data[i:] {
			total += num
			if num > max {
				max = num
			}
			if num < min {
				min = num
			}
			if total > I_suspect {
				break
			}
			if total == I_suspect {
				return max + min
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
		if err == nil {
			numbers = append(numbers, number)
		}
	}

	return numbers
}
