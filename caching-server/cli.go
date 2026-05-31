package main

import (
	"flag"
	"fmt"
	"os"
)

type Cli struct {
	Port   int
	Origin string

	Clear bool
}

func ParseCli() *Cli {
	var cli Cli

	flag.IntVar(&cli.Port, "port", 8080, "PORT of the caching server to bind to")
	flag.IntVar(&cli.Port, "p", 8080, "PORT of the caching server to bind to")
	flag.StringVar(&cli.Origin, "origin", "", "ORIGIN is the URL of the origin server to proxy requests to")
	flag.StringVar(&cli.Origin, "o", "", "ORIGIN is the URL of the origin server to proxy requests to")

	flag.BoolVar(&cli.Clear, "clear-cache", false, "Clear the cache")
	flag.BoolVar(&cli.Clear, "c", false, "Clear the cache")

	flag.Usage = func() {
		fmt.Println("Usage: caching-proxy [options]")
		flag.PrintDefaults()
	}

	flag.Parse()

	if cli.Origin == "" && !cli.Clear {
		flag.Usage()
		os.Exit(1)
	}

	return &cli
}
