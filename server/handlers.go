package main

import (
    "fmt"
    "log"
    "net/http"
)

func HandlerHome(w http.ResponseWriter, r *http.Request) {
   err := RenderHTML(w, r, "server/www/html/index.html")
   if err == nil {
       return
   }

   log.Println(err)
   httpErrCode := http.StatusInternalServerError
   httpErrText := http.StatusText(httpErrCode)
   _, _ = fmt.Fprintf(w, "%s:%d\n",  httpErrText, httpErrCode)
}