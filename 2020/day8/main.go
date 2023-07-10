package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

type Command string

const (
	Acc Command = "acc"
	Nop Command = "nop"
	Jmp Command = "jmp"
)

type Operation struct {
	Action Command
	Num    int
}

func (op *Operation) run(current *int, counter *int) {
	switch op.Action {
	case Acc:
		*current++
		*counter += op.Num
		break
	case Nop:
		*current++
		break
	case Jmp:
		*current += op.Num
	}
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

func part1(data []Operation) int {
	counter := 0
	currentOp := 0
	hash := map[int]bool{}

	for true {
		if hash[currentOp] {
			break
		}
		hash[currentOp] = true
		data[currentOp].run(&currentOp, &counter)
	}
	return counter
}

func part2(data []Operation) int {
	for i, op := range data {
		if op.Action == Acc {
			continue
		}
		copy_data := make([]Operation, len(data))
		copy(copy_data, data)
		if op.Action == Jmp {
			copy_data[i].Action = Nop
		} else {
			copy_data[i].Action = Jmp
		}

		if counter, isLoop := runLoop(copy_data); !isLoop {
			return counter
		}
	}
	// dnf
	return -1
}

// essencialy run part 1 until something is called again wich indicates a loop
func runLoop(data []Operation) (int, bool) {
	counter := 0
	currentOp := 0
	hash := map[int]bool{}
	for currentOp < len(data) {
		if hash[currentOp] {
			return 0, true
		}
		hash[currentOp] = true
		data[currentOp].run(&currentOp, &counter)
	}
	return counter, false
}

func parse_input() []Operation {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	program := []Operation{}

	for scanner.Scan() {
		var line Operation
		var operator rune
		_, err := fmt.Sscanf(scanner.Text(), "%s %c%d", &line.Action, &operator, &line.Num)
		if err != nil {
			panic("Error: Converting to valid Input")
		}
		if operator == '-' {
			line.Num *= -1
		}
		program = append(program, line)
	}

	return program
}
