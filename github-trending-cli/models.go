package main

type Repository struct {
	ID          int       `json:"id"`
	Name        string    `json:"name"`
	Description string    `json:"description"`
	HTMLUrl     string    `json:"html_url"`
	FullName    string    `json:"full_name"`
	Owner       RepoOwner `json:"owner"`
	Language    string    `json:"language"`
	Stars       int       `json:"stargazers_count"`
	Forks       int       `json:"forks_count"`
	Size        int       `json:"size"`
}

type RepoOwner struct {
	ID   int64  `json:"id"`
	Name string `json:"login"`
}

type SortType string

const (
	SortByStars   SortType = "stars"
	SortByForks   SortType = "forks"
	SortByIssues  SortType = "issues"
	SortByCreated SortType = "created"
	SortByUpdated SortType = "updated"
)

type OrderType string

const (
	OrderAsc  OrderType = "asc"
	OrderDesc OrderType = "desc"
)
