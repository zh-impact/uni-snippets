package main

import (
	"fmt"
	"log"
	"net/url"
	"os"
	"strings"

	readability "codeberg.org/readeck/go-readability/v2"
	"github.com/enetx/surf"
)

func main() {
	surfClient := surf.NewClient().
		Builder().
		Impersonate().Chrome().
		Session().
		Build().
		Unwrap()

	resp := surfClient.Get("https://dev.to/devteam/top-7-featured-dev-posts-of-the-week-555a").Do()
	if resp.IsErr() {
		log.Fatal(resp.Err())
	}

	respContent := resp.Ok().Body.String().Unwrap()

	os.WriteFile("output.html", []byte(respContent), 0644)

	baseURL, _ := url.Parse("https://dev.to/devteam/top-7-featured-dev-posts-of-the-week-555a")

	doc, err := readability.FromReader(strings.NewReader(string(respContent)), baseURL)
	if err != nil {
		log.Fatal(err)
	}

	f, err := os.Create("output.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	fmt.Printf("Title: %s\n", doc.Title())
	if err := doc.RenderHTML(f); err != nil {
		log.Fatal(err)
	}
}
