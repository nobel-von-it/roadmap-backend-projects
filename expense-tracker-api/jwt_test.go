package main_test

import (
	. "nobel/expense-api"
	"time"

	"testing"

	"github.com/golang-jwt/jwt/v5"
	"github.com/stretchr/testify/require"
)

func Test_VerifyJWT(t *testing.T) {
	type testCase struct {
		name          string
		tokenFunc     func() string
		wantErr       bool
		expectedEmail string
		expectedID    int32
	}

	tests := []testCase{
		{
			name: "valid token",
			tokenFunc: func() string {
				token, err := GenerateJWT(DBUser{
					ID:    1,
					Email: "[EMAIL_ADDRESS]",
				})
				if err != nil {
					t.Fatalf("GenerateJWT: %v", err)
				}
				return token
			},
			wantErr:       false,
			expectedEmail: "[EMAIL_ADDRESS]",
			expectedID:    1,
		},
		{
			name: "invalid token",
			tokenFunc: func() string {
				return "invalid-token"
			},
			wantErr:       true,
			expectedEmail: "",
			expectedID:    0,
		},
		{
			name: "expired token",
			tokenFunc: func() string {
				token, err := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
					"user_id": 1,
					"email":   "[EMAIL_ADDRESS]",
					"exp":     time.Now().Add(-time.Hour).Unix(),
				}).SignedString([]byte("secret"))
				if err != nil {
					t.Fatalf("GenerateJWT: %v", err)
				}
				return token
			},
			wantErr:       true,
			expectedEmail: "",
			expectedID:    0,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			token := tc.tokenFunc()
			claims, err := VerifyJWT(token)
			if tc.wantErr {
				require.Error(t, err)
				require.Nil(t, claims)
			} else {
				require.NoError(t, err)
				require.NotNil(t, claims)

				email := (*claims)["email"].(string)
				require.Equal(t, tc.expectedEmail, email)

				user_id := int32((*claims)["user_id"].(float64))
				require.Equal(t, tc.expectedID, user_id)
			}
		})
	}
}
