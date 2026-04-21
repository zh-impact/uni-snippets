package main

import (
	"fmt"
	"log"

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

	fmt.Println(resp.Ok().Body.String().Unwrap())
}
