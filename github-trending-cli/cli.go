package main

import (
	"flag"
	"fmt"
	"time"
)

type Cli struct {
	Duration DurationList
	Limit    int
}

type DurationList string

const (
	DurationDay   DurationList = "day"
	DurationWeek  DurationList = "week"
	DurationMonth DurationList = "month"
	DurationYear  DurationList = "year"
)

func (d *DurationList) String() string {
	return string(*d)
}

func (d *DurationList) Set(value string) error {
	switch DurationList(value) {
	case DurationDay, DurationWeek, DurationMonth, DurationYear:
		*d = DurationList(value)
		return nil
	default:
		return fmt.Errorf("invalid duration %q (must be: day, week, month, year)", value)
	}
}

func (d *DurationList) ToQuery() string {
	switch *d {
	case DurationDay:
		return "created:>" + time.Now().AddDate(0, 0, -1).Format("2006-01-02")
	case DurationWeek:
		return "created:>" + time.Now().AddDate(0, 0, -7).Format("2006-01-02")
	case DurationMonth:
		return "created:>" + time.Now().AddDate(0, -1, 0).Format("2006-01-02")
	case DurationYear:
		return "created:>" + time.Now().AddDate(-1, 0, 0).Format("2006-01-02")
	default:
		return "" // Unreachable code
	}
}

func ParseCli() (*Cli, error) {
	var cli Cli

	flag.Var(&cli.Duration, "duration", "Specifies the time range to query (day, week, month, year)")
	flag.Var(&cli.Duration, "d", "Specifies the time range (shorthand)")

	flag.IntVar(&cli.Limit, "limit", 10, "Specifies the number of repositories to display")
	flag.IntVar(&cli.Limit, "l", 10, "Specifies the number of repositories (shorthand)")

	flag.Usage = func() {
		fmt.Println("Usage: trending-repos [flags]")
		fmt.Println("Flags:")
		flag.VisitAll(func(f *flag.Flag) {
			fmt.Println("\t", f.Name, "\t", f.Usage)
		})
	}

	flag.Parse()

	if cli.Limit <= 0 {
		return nil, fmt.Errorf("limit must be a positive integer")
	}
	if cli.Duration == "" {
		cli.Duration = DurationWeek
	}

	return &cli, nil
}
