package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
)

func main() {
	customer, err := ioutil.ReadFile("data/customer.json")
	if err != nil {
		fmt.Printf("Could not read customer file: %s", err)
		return
	}

	accounts, err := ioutil.ReadFile("data/accounts.json")
	if err != nil {
		fmt.Printf("Could not read accounts file: %s", err)
		return
	}

	cards, err := ioutil.ReadFile("data/cards.json")
	if err != nil {
		fmt.Printf("Could not read cards file: %s", err)
		return
	}

	http.HandleFunc("/customer", func(w http.ResponseWriter, r *http.Request) {
		w.Header()["Content-Type"] = []string{"application/json"}
		w.Write(customer)
	})

	http.HandleFunc("/accounts", func(w http.ResponseWriter, r *http.Request) {
		w.Header()["Content-Type"] = []string{"application/json"}
		w.Write(accounts)
	})

	http.HandleFunc("/cards", func(w http.ResponseWriter, r *http.Request) {
		w.Header()["Content-Type"] = []string{"application/json"}
		w.Write(cards)
	})

	fmt.Printf("Listening on 3000\n")
	http.ListenAndServe(":3000", nil)
}
