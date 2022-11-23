package main

import (
    "fmt"
    "net/http"
    "html/template"
)

func RenderHTML(w http.ResponseWriter, r *http.Request, html ...string) error {
    w.Header().Set("Content-Type", "text/html;charset=utf-8")

    templ, err := template.New("").ParseFiles(html...)
    if err != nil {
        return fmt.Errorf("failed to compile HTML template, error: %w", err)
    }

    err = templ.ExecuteTemplate(w, "layout", nil)
    if err != nil {
        return fmt.Errorf("failed to serve HTML template, error: %w", err)
    }

    return nil
}