package main

import (
	"log"
)

func main() {
	cli, err := ParseCli()
	if err != nil {
		log.Fatal(err)
	}

	api := NewApiBuilderWithCli(cli).Build()

	repos, err := api.Fetch()
	if err != nil {
		log.Fatal(err)
	}

	printer := NewPrinter(repos)
	printer.Print()
}
