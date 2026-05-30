package main

import (
	"fmt"
	"log"

	"github.com/joho/godotenv"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Fatalln("No .env file found")
	}

	cli, err := ParseCli()
	if err != nil {
		log.Fatalln(err)
	}

	reqBuilder, err := NewApiBuilder()
	if err != nil {
		log.Fatalln(err)
	}
	reqBuilder.WithApiKey().WithMovieType(cli.Type).WithLanguage(cli.Language)
	req := reqBuilder.Build()

	api := NewApi(req)
	result, err := api.Fetch()
	if err != nil {
		log.Fatalln(err)
	}

	fmt.Printf("Movie Type: %s\n", cli.Type)
	fmt.Printf("Language: %s\n", cli.Language)
	fmt.Println("----------------------------")
	for _, movie := range result.Results {
		fmt.Printf("%s (%s) - %.1f/10\n", movie.Title, movie.ReleaseDate, movie.VoteAverage)
	}
}
