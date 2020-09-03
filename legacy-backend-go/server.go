package main

import (
	"fmt"
	"io/ioutil"
	"math/rand"
	"net/http"
	"time"
)

func handler(data []byte) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		pause := int64((rand.NormFloat64()*0.3 + 2) * 1_000_000_000)
		time.Sleep(time.Duration(pause))

		w.Header()["Content-Type"] = []string{"application/json"}
		w.Write(data)
	}
}

func main() {
	customer, err := ioutil.ReadFile("../data/customer.json")
	if err != nil {
		fmt.Printf("Could not read customer file: %s", err)
		return
	}

	accounts, err := ioutil.ReadFile("../data/accounts.json")
	if err != nil {
		fmt.Printf("Could not read accounts file: %s", err)
		return
	}

	cards, err := ioutil.ReadFile("../data/cards.json")
	if err != nil {
		fmt.Printf("Could not read cards file: %s", err)
		return
	}

	http.HandleFunc("/customer", handler(customer))
	http.HandleFunc("/accounts", handler(accounts))
	http.HandleFunc("/cards", handler(cards))

	http.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNoContent)
	})

	fmt.Printf("Listening on 3000\n")
	http.ListenAndServe(":3000", nil)
}
