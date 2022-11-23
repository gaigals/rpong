package main

import (
    "fmt"
    "net/http"
)



func AddHandlers() {
    http.HandleFunc("/", HandlerHome)

    fileServer := http.FileServer(http.Dir("server/www/static"))
    http.Handle("/static/",http.StripPrefix("/static/",fileServer))
}

func StartServer() error {
    server := &http.Server{
        Addr:    ":80",
    }

    err := server.ListenAndServe()
    if err != nil {
        return fmt.Errorf("error occurred in ListenAndServe%s, error: %v",
            server.Addr, err)
    }

    return nil
}

