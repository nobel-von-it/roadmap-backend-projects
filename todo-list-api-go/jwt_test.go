package main_test

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	. "nobel/todo-api"
)

func TestVerifyJwt(t *testing.T) {
	type testCase struct {
		name          string
		tokenFunc     func() string
		wantErr       bool
		expectedEmail string
	}

	tests := []testCase{
		{
			name: "successful jwt validation",
			tokenFunc: func() string {
				token, _ := GenerateJwt(DbUser{ID: 1, User: User{Email: "test@example.com"}})
				return token
			},
			wantErr:       false,
			expectedEmail: "test@example.com",
		},
		{
			name: "failed jwt validation",
			tokenFunc: func() string {
				return "invalid token"
			},
			wantErr: true,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			token := tc.tokenFunc()
			claims, err := VerifyJwt(token)
			if tc.wantErr {
				assert.Error(t, err)
				assert.Nil(t, claims)
			} else {
				require.NoError(t, err)
				require.NotNil(t, claims)

				email := (*claims)["email"].(string)
				assert.Equal(t, tc.expectedEmail, email)
			}
		})
	}
}
