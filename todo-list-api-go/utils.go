package main

func GetJwtSecret() []byte {
	// todo: move to .env
	return []byte("secret")
}
