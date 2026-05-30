package main

import (
	"fmt"
	"os"
	"strconv"
	"text/tabwriter"
)

type Color string

const (
	ColorReset     Color = "\033[0m"
	ColorRed       Color = "\033[31m"
	ColorGreen     Color = "\033[32m"
	ColorYellow    Color = "\033[33m"
	ColorBlue      Color = "\033[34m"
	ColorPurple    Color = "\033[35m"
	ColorCyan      Color = "\033[36m"
	ColorWhite     Color = "\033[37m"
	ColorGray      Color = "\033[90m"
	StyleBold      Color = "\033[1m"
	StyleUnderline Color = "\033[4m"
)

type Printer struct {
	writer *tabwriter.Writer
	Repos  []Repository
}

func NewPrinter(repos []Repository) *Printer {
	writer := tabwriter.NewWriter(os.Stdout, 0, 0, 3, ' ', 0)
	return &Printer{writer: writer, Repos: repos}
}

func (p *Printer) Print() {
	defer p.writer.Flush()
	p.printHeader()
	p.printRepos()
}

func (p *Printer) printHeader() {
	fmt.Fprintf(
		p.writer,
		"%s\t%s\t%s\t%s\n",
		p.wrapText("REPOSITORY", StyleBold),
		p.wrapText("STARS", ColorYellow),
		p.wrapText("LANGUAGE", ColorCyan),
		p.wrapText("URL", ColorBlue),
	)
}

func (p *Printer) printRepos() {
	for _, repo := range p.Repos {
		fmt.Fprintf(
			p.writer,
			"%s\t%s\t%s\t%s\n",
			p.wrapText(repo.FullName, StyleBold),
			p.wrapText(strconv.Itoa(repo.Stars), ColorYellow),
			p.wrapText(repo.Language, ColorCyan),
			p.wrapText(repo.HTMLUrl, ColorBlue),
		)
	}
}

func (p *Printer) wrapText(text string, color Color) string {
	return string(color) + text + string(ColorReset)
}
