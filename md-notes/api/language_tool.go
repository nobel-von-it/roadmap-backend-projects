package api

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
)

type SupportedLanguages string

const (
	English SupportedLanguages = "en-US"
	Russian SupportedLanguages = "ru-RU"
	Auto    SupportedLanguages = "auto"
)

type LTRequest struct {
	Text     string             `json:"text"`
	Language SupportedLanguages `json:"language"`
}

func (l *LTRequest) Valide() error {
	if l.Text == "" {
		return errors.New("text is required")
	}
	if l.Language == "" {
		return errors.New("language is required")
	}
	return nil
}

func (l *LTRequest) Fetch(ctx context.Context) (string, error) {
	if err := l.Valide(); err != nil {
		return "", fmt.Errorf("invalid request data: %w", err)
	}

	data := url.Values{}
	data.Set("text", l.Text)
	data.Set("language", string(l.Language))

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, "http://localhost:8081/v2/check", strings.NewReader(data.Encode()))
	if err != nil {
		return "", fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Set("Accept", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("failed to execute request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		errBody, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("api returned status %d: %s", resp.StatusCode, string(errBody))
	}

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response body: %w", err)
	}

	return string(bodyBytes), nil
}
