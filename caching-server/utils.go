package main

import (
	"fmt"
	"strconv"
	"strings"
)

const baseUrl = "http://localhost"

func NormalizePort(port int) string {
	portStr := strconv.Itoa(port)
	if strings.HasPrefix(portStr, ":") {
		return portStr
	}
	return fmt.Sprintf(":%d", port)
}

func CheckHeader(headers map[string]string, key string) bool {
	_, ok := headers[key]
	return ok
}

func GetLocalUrl(port int) string {
	return fmt.Sprintf("%s%s", baseUrl, NormalizePort(port))
}

func NormalizeUrl(fullUrl, origin string, port int) string {
	localUrl := GetLocalUrl(port)
	// https://localhost:<port>/products -> origin/products (origin without port)
	return strings.ReplaceAll(fullUrl, localUrl, origin)
}
