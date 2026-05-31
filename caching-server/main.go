package main

import (
	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/log"
)

func main() {
	cli := ParseCli()

	if cli.Clear {
		if err := SendClearRequest(cli.Port); err != nil {
			log.Errorf("Unable to clear cache: %s", err)
		}
		log.Info("Cache cleared")
		return
	}

	app := fiber.New()
	cache := NewCache()
	env := NewEnv(cache, cli.Origin, cli.Port)

	app.Delete("/clear-cache", env.ClearCacheHandler)
	app.All("/*", env.Handle)

	app.Listen(NormalizePort(cli.Port))
}
