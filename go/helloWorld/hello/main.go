package main

import (
	"fmt"
	"greetings"
)

func main() {
	message := greetings.Hello("Nick")
	fmt.Println(message)
}
