package main

import (
	"strings"

	"github.com/gofiber/fiber/v3"
)

func AuthMiddleware(c fiber.Ctx) error {
	authHeader := c.Get("Authorization")
	if authHeader == "" {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Authorization header is required",
		})
	}

	var tokenString string
	const prefix = "Bearer "
	if len(authHeader) > len(prefix) && strings.HasPrefix(authHeader, prefix) {
		tokenString = authHeader[len(prefix):]
	}
	if tokenString == "" {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Authorization header is malformed",
		})
	}

	claims, err := VerifyJWT(tokenString)
	if err != nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Provided token is invalid or expired",
		})
	}

	userIDFloat, ok := (*claims)["user_id"].(float64)
	if !ok {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "Invalid token claims",
		})
	}

	userID := int64(userIDFloat)
	c.Locals("userID", userID)

	return c.Next()
}
