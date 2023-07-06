package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"regexp"
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

func part1(data []map[string]string) int {
	var total int = 0
	for _, passport := range data {
		if len(passport) == 7 {
			if _, ok := passport["cid"]; ok {
				continue
			}
		}
		total++
	}
	return total
}

func validate(passport map[string]string) bool {
	birth, err := strconv.Atoi(passport["byr"])
	if err != nil || birth < 1920 || birth > 2002 {
		return false
	}
	issue, err := strconv.Atoi(passport["iyr"])
	if err != nil || issue < 2010 || issue > 2020 {
		return false
	}
	expiration, err := strconv.Atoi(passport["eyr"])
	if err != nil || expiration < 2020 || expiration > 2030 {
		return false
	}
	var hg int
	var unit string
	_, error := fmt.Sscanf(passport["hgt"], "%d%s", &hg, &unit)
	if error != nil {
		return false
	}
	if unit == "cm" {
		if hg < 150 || hg > 193 {
			return false
		}
	} else {
		if hg < 59 || hg > 76 {
			return false
		}
	}
	hair_regex := regexp.MustCompile(`^#[0-9a-f]{6}$`)
	if !hair_regex.MatchString(passport["hcl"]) {
		return false
	}
	eye_color := []string{
		"amb", "blu", "brn", "gry", "grn", "hzl", "oth",
	}
	found := false
	for _, color := range eye_color {
		if color == passport["ecl"] {
			found = true
		}
	}
	if !found {
		return false
	}
	id_regex := regexp.MustCompile(`^\d{9}$`)
	if !id_regex.MatchString(passport["pid"]) {
		return false
	}

	return true
}

func part2(data []map[string]string) int {
	var total int = 0
	for _, passport := range data {
		if len(passport) == 7 {
			if _, ok := passport["cid"]; ok {
				continue
			}
		}
		if validate(passport) {
			total++
		}
	}
	return total
}

func parse_input() []map[string]string {
	file, err := os.Open("input.txt")
	if err != nil {
		panic("Error: Unable to open file")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	var passports []map[string]string
	currentPassport := make(map[string]string)

	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			fields := strings.Split(line, " ")
			for _, field := range fields {
				parts := strings.Split(field, ":")
				currentPassport[parts[0]] = parts[1]
			}
		} else {
			if len(currentPassport) > 6 {
				passports = append(passports, currentPassport)
			}
			currentPassport = make(map[string]string)
		}
	}
	if len(currentPassport) > 6 {
		passports = append(passports, currentPassport)
	}

	return passports
}
