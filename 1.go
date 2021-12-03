package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	bs, err := ioutil.ReadFile("1.txt")
	if err != nil {
		panic(err)
	}
	input := strings.Split(string(bs), "\n")
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
	fmt.Println(increased)

}
