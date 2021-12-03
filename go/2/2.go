package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func partOne(lines []string) int {
	h := 0
	d := 0
	for _, cmd := range lines {
		if cmd == "" {
			continue
		}
		splitted := strings.Split(cmd, " ")
		action := splitted[0]
		X, _ := strconv.Atoi(splitted[1])
		if action == "forward" {
			h += X
		} else if action == "down" {
			d += X
		} else if action == "up" {
			d -= X
		}
	}
	return h * d
}

func partTwo(lines []string) int {
	h := 0
	d := 0
	a := 0
	for _, cmd := range lines {
		if cmd == "" {
			continue
		}
		splitted := strings.Split(cmd, " ")
		action := splitted[0]
		X, _ := strconv.Atoi(splitted[1])
		if action == "forward" {
			h += X
			d += a * X
		} else if action == "down" {
			a += X
		} else if action == "up" {
			a -= X
		}
	}
	return h * d
}

func main() {
	bs, _ := ioutil.ReadFile("2.txt")
	lines := strings.Split(string(bs), "\n")
	fmt.Println(partOne(lines))
	fmt.Println(partTwo(lines))
}
