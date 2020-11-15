package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	//if factorial(5) != 120 {
	//	log.Fatal("falty")
	//}
	//fmt.Println("done")

	concurrent()
}

var members = 100

func concurrent() {
	wg := &sync.WaitGroup{}
	
	for i := 0; i < members; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			time.Sleep(1 * time.Second)
			fmt.Printf("%d: person done eating\n", i+1)
		}(i)
	}
	wg.Wait() //0
}

func factorial(n uint64) (result uint64) {
	if n > 0 {
		result = n * factorial(n-1)
		return result
	}
	return 1
}
