package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"time"
)

type ApiBuilder struct {
	request *http.Request
}

func NewApiBuilder() (*ApiBuilder, error) {
	request, err := http.NewRequest(http.MethodGet, "https://api.themoviedb.org/3", nil)
	if err != nil {
		return nil, err
	}

	return &ApiBuilder{request}, nil
}

func (b *ApiBuilder) WithApiKey() *ApiBuilder {
	apiKey := os.Getenv("TMDB_JWT_KEY")
	if apiKey == "" {
		return nil
	}

	b.request.Header.Set("Authorization", "Bearer "+apiKey)
	return b
}

func (b *ApiBuilder) WithMovieType(movieType MovieType) *ApiBuilder {
	b.request.URL.Path += "/movie/" + movieType.String()
	return b
}

func (b *ApiBuilder) WithLanguage(language Language) *ApiBuilder {
	q := b.request.URL.Query()
	q.Add("language", string(language))
	b.request.URL.RawQuery = q.Encode()
	return b
}

func (b *ApiBuilder) Build() *http.Request {
	return b.request
}

type Api struct {
	client *http.Client
	req    *http.Request
}

func NewApi(req *http.Request) *Api {
	return &Api{
		client: &http.Client{
			Timeout: 10 * time.Second,
		},
		req: req,
	}
}

func (a *Api) Fetch() (*TMDBResponse, error) {
	resp, err := a.client.Do(a.req)
	if err != nil {
		return nil, err
	}

	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("server error: %d", resp.StatusCode)
	}

	var result TMDBResponse
	err = json.NewDecoder(resp.Body).Decode(&result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}
