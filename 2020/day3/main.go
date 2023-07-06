package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
)

type Coord struct {
	X      int
	Y      int
	Move_X int
	Move_Y int
}

func (c *Coord) Update() {
	c.X += c.Move_X
	c.Y += c.Move_Y
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

func part1(data [][]bool) int {
	var total int = 0
	current := Coord{X: 3, Y: 1, Move_X: 3, Move_Y: 1}
	max_Y := len(data)
	max_X := len(data[0])

	for current.Y < max_Y {
		if data[current.Y%max_Y][current.X%max_X] {
			total++
		}
		current.Update()
	}
	return total
}

func part2(data [][]bool) int {
	var total int = 1
	coordenates := []Coord{
		{X: 1, Y: 1, Move_X: 1, Move_Y: 1},
		{X: 3, Y: 1, Move_X: 3, Move_Y: 1},
		{X: 5, Y: 1, Move_X: 5, Move_Y: 1},
		{X: 7, Y: 1, Move_X: 7, Move_Y: 1},
		{X: 1, Y: 2, Move_X: 1, Move_Y: 2},
	}
	max_Y := len(data)
	max_X := len(data[0])

	for _, coordenate := range coordenates {
		var slope_total int = 0
		for coordenate.Y < max_Y {
			if data[coordenate.Y%max_Y][coordenate.X%max_X] {
				slope_total++
			}
			coordenate.Update()
		}
		total *= slope_total
	}
	return total
}

func parse_input() [][]bool {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	matrix := [][]bool{}

	for scanner.Scan() {
		var matrix_line []bool
		for _, char := range scanner.Text() {
			tree := char == '#'
			matrix_line = append(matrix_line, tree)
		}
		matrix = append(matrix, matrix_line)
	}

	return matrix
}
