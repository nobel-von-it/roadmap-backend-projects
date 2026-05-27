package main

import (
	"github.com/gofiber/fiber/v3"
)

func AuthMiddleware(c fiber.Ctx) error {
	authHeader := c.Get("Authorization")
	if authHeader == "" {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{"message": "Authorization header is required"})
	}
	tokenString := authHeader
	const prefix = "Bearer "
	if len(authHeader) > len(prefix) && authHeader[:len(prefix)] == prefix {
		tokenString = authHeader[len(prefix):]
	}
	if tokenString == "" {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{"message": "Invalid token format"})
	}

	claims, err := VerifyJwt(tokenString)
	if err != nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{"message": "Invalid token"})
	}

	userIdFloat, ok := (*claims)["user_id"].(float64)
	if !ok {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{"message": "Invalid user ID in token"})
	}

	// save user in local context
	c.Locals("userId", int(userIdFloat))

	return c.Next()
}
