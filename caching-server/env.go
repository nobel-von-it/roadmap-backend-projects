package main

import (
	"fmt"
	"io"
	"net/http"

	"github.com/gofiber/fiber/v3"
)

type Env struct {
	cache  *Cache
	origin string
	port   int
}

func NewEnv(cache *Cache, origin string, port int) *Env {
	return &Env{
		cache:  cache,
		origin: origin,
		port:   port,
	}
}

func (e *Env) Handle(c fiber.Ctx) error {
	directUrl := e.origin + c.OriginalURL()
	cacheRes, ok := e.cache.Get(directUrl)
	if ok {
		c.Status(cacheRes.StatusCode)
		for k, v := range cacheRes.Headers {
			c.Set(k, v)
		}
		c.Set("X-Cache", "HIT")
		return c.Send(cacheRes.Body)
	}

	return e.handleMiss(c, directUrl)
}

func (e *Env) handleMiss(c fiber.Ctx, directUrl string) error {
	req, err := http.NewRequest(c.Method(), directUrl, c.Request().BodyStream())
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).SendString(err.Error())
	}

	for k, v := range c.Request().Header.All() {
		req.Header.Set(string(k), string(v))
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return c.Status(fiber.StatusBadGateway).SendString(err.Error())
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).SendString(err.Error())
	}

	headers := make(map[string]string)
	for k, v := range resp.Header {
		if len(v) > 0 {
			headers[k] = v[0]
		}
	}

	e.cache.Set(directUrl, Response{
		StatusCode: resp.StatusCode,
		Headers:    headers,
		Body:       body,
	})

	c.Status(resp.StatusCode)
	for k, v := range headers {
		c.Set(k, v)
	}
	c.Set("X-Cache", "MISS")

	return c.Send(body)
}

func (e *Env) ClearCacheHandler(c fiber.Ctx) error {
	e.cache.Clear()
	return c.Status(fiber.StatusOK).JSON(fiber.Map{
		"message": "Cache cleared",
	})
}

func SendClearRequest(port int) error {
	req, err := http.NewRequest(http.MethodDelete, GetLocalUrl(port)+"/clear-cache", nil)
	if err != nil {
		return err
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	if resp.StatusCode != fiber.StatusOK {
		return fmt.Errorf("clear cache request failed: %s", string(body))
	}

	return nil
}
