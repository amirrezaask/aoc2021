package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func partOne(input []string) int {
	var increased int
	var last int
	for idx, line := range input {
		item, _ := strconv.Atoi(line)

		if idx == 0 {
			last = item
			continue
		}
		if last < item {
			increased++
		}
		last = item
	}
	return increased
}

func partTwo(input []string) int {
	// x,y,z are three elements of the sliding window
	var last int
	var increased int
	for i := 0; i < len(input); i++ {
		if i+2 >= len(input) {
			break
		}
		x, _ := strconv.Atoi(input[i])
		y, _ := strconv.Atoi(input[i+1])
		z, _ := strconv.Atoi(input[i+2])
		if i == 0 {
			last = x + y + z
			continue
		}
		if last < x+y+z {
			increased++
		}
		last = x + y + z
	}
	return increased
}

func main() {
	bs, err := ioutil.ReadFile("1.txt")
	if err != nil {
		panic(err)
	}
	input := strings.Split(string(bs), "\n")
	increaseOne := partOne(input)
	increaseTwo := partTwo(input)
	fmt.Println(increaseOne)
	fmt.Println(increaseTwo)
}
