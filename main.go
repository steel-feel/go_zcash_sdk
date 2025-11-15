package main

import (
	"fmt"
)

/*
#cgo LDFLAGS: -L${SRCDIR}/target/release -lrust_ffi_go
#include <stdlib.h>

int add_numbers(int a, int b);
*/
import "C"

func main() {
	// num := int(time.Now().UnixMicro())
	// Seed(num)
	// fmt.Printf("Random number with %v from C %v \n", num,Random())
	// a := int32(1) 
	// b := int32(2)
	a := 1
	b := 2
	c := int(C.add_numbers(C.int(a),C.int(b)))

	fmt.Printf("Addition of number %v + %v = %v \n",a,b,c)

}
