package main

import "log"

func main() {
    AddHandlers()

    err := StartServer()
    if err != nil {
        log.Fatalln(err)
    }
}