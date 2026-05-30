package main

import (
	"fmt"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

const JWT_EXPIRATION_TIME = time.Hour * 24

func GetJWTKey() ([]byte, error) {
	return []byte("secret"), nil
}

func GenerateJWT(user DBUser) (string, error) {
	key, err := GetJWTKey()
	if err != nil {
		return "", err
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": user.ID,
		"email":   user.Email,
		"exp":     time.Now().Add(JWT_EXPIRATION_TIME).Unix(),
	})
	return token.SignedString(key)
}

func VerifyJWT(tokenString string) (*jwt.MapClaims, error) {
	token, err := jwt.ParseWithClaims(tokenString, &jwt.MapClaims{}, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return GetJWTKey()
	})
	if err != nil {
		return nil, err
	}
	return token.Claims.(*jwt.MapClaims), nil
}
