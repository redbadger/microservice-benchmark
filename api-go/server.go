package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
)

// CustomerResult is a result of fetching a customer from the backend which can be put on a channel
type CustomerResult struct {
	Customer Customer
	Error    error
}

func getCustomer(url string) CustomerResult {
	resp, err := http.Get(url)
	if err != nil || resp.Status != "200 OK" {
		return CustomerResult{Customer{}, fmt.Errorf("Could not fetch customer %e", err)}
	}

	defer resp.Body.Close()
	customerJSON, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return CustomerResult{Customer{}, fmt.Errorf("Could not read customer response: %e", err)}
	}

	var customer Customer
	err = json.Unmarshal(customerJSON, &customer)
	if err != nil {
		return CustomerResult{Customer{}, fmt.Errorf("Could not parse customer JSON: %e", err)}
	}

	return CustomerResult{customer, nil}
}

// AccountsResult is a result of fetching accounts from the backend which can be put on a channel
type AccountsResult struct {
	Accounts []Account
	Error    error
}

func getAccounts(url string) AccountsResult {
	resp, err := http.Get(url)
	if err != nil || resp.Status != "200 OK" {
		return AccountsResult{[]Account{}, fmt.Errorf("Could not fetch accounts %e", err)}
	}

	defer resp.Body.Close()
	accountsJSON, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return AccountsResult{[]Account{}, fmt.Errorf("Could not read accounts response: %e", err)}
	}

	var accounts []Account
	err = json.Unmarshal(accountsJSON, &accounts)
	if err != nil {
		return AccountsResult{[]Account{}, fmt.Errorf("Could not parse accounts JSON: %e", err)}
	}

	return AccountsResult{accounts, nil}
}

// CardsResult is a result of fetching cards from the backend which can be put on a channel
type CardsResult struct {
	Cards []Card
	Error error
}

func getCards(url string) CardsResult {
	resp, err := http.Get(url)
	if err != nil || resp.Status != "200 OK" {
		return CardsResult{[]Card{}, fmt.Errorf("Could not fetch cards %e", err)}
	}

	defer resp.Body.Close()
	cardsJSON, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return CardsResult{[]Card{}, fmt.Errorf("Could not read cards response: %e", err)}
	}

	var cards []Card
	err = json.Unmarshal(cardsJSON, &cards)
	if err != nil {
		return CardsResult{[]Card{}, fmt.Errorf("Could not parse cards JSON: %e", err)}
	}

	return CardsResult{cards, nil}
}

func main() {
	customerAPI := os.Getenv("API_CUSTOMER")
	accountsAPI := os.Getenv("API_ACCOUNTS")
	cardsAPI := os.Getenv("API_CARDS")

	fmt.Printf("Using customer API: %s\n", customerAPI)
	fmt.Printf("Using accounts API: %s\n", accountsAPI)
	fmt.Printf("Using cards API: %s\n", cardsAPI)

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		customerChan := make(chan CustomerResult)
		accountsChan := make(chan AccountsResult)
		cardsChan := make(chan CardsResult)

		go func() { customerChan <- getCustomer(customerAPI) }()
		go func() { accountsChan <- getAccounts(accountsAPI) }()
		go func() { cardsChan <- getCards(cardsAPI) }()

		customerResult := <-customerChan
		if customerResult.Error != nil {
			panic(customerResult.Error)
		}

		accountsResult := <-accountsChan
		if accountsResult.Error != nil {
			panic(accountsResult.Error)
		}

		cardsResult := <-cardsChan
		if cardsResult.Error != nil {
			panic(cardsResult.Error)
		}

		customer := customerResult.Customer
		customer.Accounts = accountsResult.Accounts
		customer.Cards = cardsResult.Cards

		json, err := json.Marshal(customer)
		if err != nil {
			panic(err)
		}

		w.WriteHeader(http.StatusOK)
		w.Write(json)

	})

	http.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNoContent)
	})

	fmt.Printf("Listening on 8000\n")
	http.ListenAndServe(":8000", nil)
}
