package main

import (
	"fmt"
	"math"
)

func main() {
	var a, b uint8 = 200, 100

	// Addition
	sum := a + b
	println(sum)
	if sum < a || sum < b {
		fmt.Println("Overflow occurred in addition!")
	} else {
		fmt.Println("Sum without overflow:", sum)
	}

	// Subtraction
	diff := a - b
	if a < b || diff > a {
		fmt.Println("Overflow occurred in subtraction!")
	} else {
		fmt.Println("Difference without overflow:", diff)
	}

	// Multiplication
	product := a * b
	println(product)
	if a != 0 && b != 0 && product/a != b {
		fmt.Println("Overflow occurred in multiplication!")
	} else {
		fmt.Println("Product without overflow:", product)
	}

	// Overflow using math.MaxUint8
	c := math.MaxUint8
	overflow := c + 1
	fmt.Println("Overflow using math.MaxUint8:", overflow)
}
