package main

import "time"

const ContextTimeout = 5 * time.Second

func Ptr[T any](val T) *T {
	return &val
}
