package main

import (
	"flag"
	"fmt"
)

type MovieType string

const (
	Playing  MovieType = "playing"
	Popular  MovieType = "popular"
	Top      MovieType = "top"
	Upcoming MovieType = "upcoming"
)

func (m *MovieType) String() string {
	switch *m {
	case Playing:
		return "now_playing"
	case Popular:
		return "popular"
	case Top:
		return "top_rated"
	case Upcoming:
		return "upcoming"
	}
	return "" // Impossible case
}

func (m *MovieType) Set(value string) error {
	switch MovieType(value) {
	case Playing, Popular, Top, Upcoming:
		*m = MovieType(value)
		return nil
	default:
		return fmt.Errorf("invalid movie type: %s", value)
	}
}

type Language string

const (
	English Language = "en-US"
	Russian Language = "ru-RU"
)

func (l *Language) String() string {
	return string(*l)
}

func (l *Language) Set(value string) error {
	switch Language(value) {
	case English, Russian:
		*l = Language(value)
		return nil
	default:
		return fmt.Errorf("invalid language: %s", value)
	}
}

type Cli struct {
	Type     MovieType
	Language Language
}

func ParseCli() (*Cli, error) {
	var cli Cli
	flag.Var(&cli.Type, "type", "Movie type to fetch (playing, popular, top, upcoming)")
	flag.Var(&cli.Type, "t", "Movie type to fetch (playing, popular, top, upcoming)")
	flag.Var(&cli.Language, "language", "Language to fetch (en-US, ru-RU)")
	flag.Var(&cli.Language, "l", "Language to fetch (en-US, ru-RU)")
	flag.Parse()

	if cli.Type == "" {
		cli.Type = Playing
	}
	if cli.Language == "" {
		cli.Language = English
	}
	return &cli, nil
}
