package main

import (
	"errors"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

type Jwt struct {
	Token string `json:"token"`
}

func GenerateJwt(user DbUser) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": user.ID,
		"email":   user.Email,
		"exp":     time.Now().Add(time.Hour * 24).Unix(),
	})
	return token.SignedString(GetJwtSecret())
}

func VerifyJwt(tokenString string) (*jwt.MapClaims, error) {
	token, err := jwt.ParseWithClaims(tokenString, &jwt.MapClaims{}, func(token *jwt.Token) (interface{}, error) {
		return GetJwtSecret(), nil
	})
	if err != nil {
		return nil, err
	}
	if !token.Valid {
		return nil, errors.New("invalid token")
	}
	return token.Claims.(*jwt.MapClaims), nil
}
