package main

import (
	"errors"
	"fmt"
	"log"
	"net/http"
)

func ConvertLenght(from string, to string, value float64) (float64, error) {
	conversionFactors := map[string]float64{
		"millimeter": 0.001,
		"centimeter": 0.01,
		"meter":      1.0,
		"kilometer":  1000.0,
		"inch":       0.0254,
		"foot":       0.3048,
		"yard":       0.9144,
		"mile":       1609.344,
	}

	fromFactor, fromOk := conversionFactors[from]
	toFactor, toOk := conversionFactors[to]
	if !fromOk || !toOk {
		return 0, errors.New("invalid units provided")
	}

	meters := value * fromFactor
	return meters / toFactor, nil
}

func main() {
	http.HandleFunc("/convert/", func(w http.ResponseWriter, r *http.Request) {
		conertType := r.URL.Query().Get("type")
		from := r.URL.Query().Get("from")
		to := r.URL.Query().Get("to")
		value := r.URL.Query().Get("value")

		switch conertType {
		case "lenght":
			result, err := ConvertLenght(from, to, value)
			if err != nil {
				fmt.Println(err)
				http.Error(w, err.Error(), http.StatusBadRequest)
				return
			}

		}

		fmt.Println(conert_type, from, to, value)
	})
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "../client/index.html")
	})
	log.Fatalln(http.ListenAndServe(":8080", nil))
}
