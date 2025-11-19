package main

import (
	// "fmt"
	"fmt"
	"unsafe"
)

/*
#cgo LDFLAGS: -L${SRCDIR}/target/release -lrust_ffi_go
#include <stdlib.h>
void go_create_wallet(const char* str);
typedef struct { char* uuid; char* uivk; char* uifk; char* source; } CAccount;
typedef struct { CAccount* ptr; size_t len; } CAccountArray;
CAccountArray go_list_accounts(const char* str);
void free_struct_array(CAccountArray);
*/
import "C"

func main() {
	
	/// Play string
	myString := "hello"
	cs := C.CString(myString)

	defer C.free(unsafe.Pointer(cs))

//	C.go_create_wallet(cs)
  accArray := C.go_list_accounts(cs)
  defer C.free_struct_array(accArray)

    goSlice := (*[1 << 28]C.CAccount)(unsafe.Pointer(accArray.ptr))[:accArray.len:accArray.len]
    // result := make([]YourGoStruct, arr.len)
    for _, s := range goSlice {
        // result[i] = YourGoStruct{
        //     Field1: C.GoString(s.field1),
        //     Field2: C.GoString(s.field2),
        //     Field3: C.GoString(s.field3),
        //     Field4: C.GoString(s.field4),
        // }

		fmt.Printf("UUid %v \n",  C.GoString(s.uuid))
    }	

	/*
	/// Get string
	cStr := C.get_string()
    if cStr == nil {
        println("Empty string")
		return
    }
    defer C.free_string(cStr)
    
    // Convert C string to Go string
    goStr := C.GoString(cStr)
	fmt.Printf("string from rust %v \n", goStr )
	*/



}
