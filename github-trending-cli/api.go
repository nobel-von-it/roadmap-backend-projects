package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

const baseApiUrl = "https://api.github.com/search/repositories"

type ApiBilder struct {
	Url     string
	Sort    SortType
	Order   OrderType
	PerPage int
	Page    int
	Query   string
}

func NewApiBuilder() *ApiBilder {
	return &ApiBilder{
		Url:     baseApiUrl,
		Sort:    SortByStars,
		Order:   OrderDesc,
		PerPage: 10,
		Page:    1,
		Query:   "",
	}
}

func (a *ApiBilder) WithSort(sort SortType) *ApiBilder {
	a.Sort = sort
	return a
}

func (a *ApiBilder) WithOrder(order OrderType) *ApiBilder {
	a.Order = order
	return a
}

func (a *ApiBilder) WithPerPage(perPage int) *ApiBilder {
	a.PerPage = perPage
	return a
}

func (a *ApiBilder) WithPage(page int) *ApiBilder {
	a.Page = page
	return a
}

func (a *ApiBilder) WithQuery(query string) *ApiBilder {
	a.Query = query
	return a
}

func (a *ApiBilder) Build() Api {
	return Api{
		Url: fmt.Sprintf("%s?q=%s&sort=%s&order=%s&per_page=%d&page=%d", a.Url, a.Query, a.Sort, a.Order, a.PerPage, a.Page),
	}
}

func NewApiBuilderWithCli(cli *Cli) *ApiBilder {
	return NewApiBuilder().WithPerPage(cli.Limit).WithQuery(cli.Duration.ToQuery())
}

type Api struct {
	Url string
}

func (a *Api) Fetch() ([]Repository, error) {
	resp, err := http.Get(a.Url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var result struct {
		Items []Repository `json:"items"`
	}

	err = json.NewDecoder(resp.Body).Decode(&result)
	if err != nil {
		return nil, err
	}

	return result.Items, nil
}
