package main

// Account

// AccountTransaction is a transaction on an account
type AccountTransaction struct {
	Amount   int64  `json:"amount"`
	Currency string `json:"currency"`
	From     string `json:"from"`
}

// Account is a customer's bank account
type Account struct {
	ID           string               `json:"_id"`
	Balance      string               `json:"balance"`
	GUID         string               `json:"guid"`
	Index        int64                `json:"index"`
	IsActive     bool                 `json:"isActive"`
	Transactions []AccountTransaction `json:"transactions"`
}

// CardTransaction is a transaction on a card
type CardTransaction struct {
	Amount    int64  `json:"amount"`
	Currency  string `json:"currency"`
	Merchant  string `json:"merchant"`
	Timestamp string `json:"timestamp"`
}

// Card is a customers card
type Card struct {
	ID           string            `json:"_id"`
	CardNumber   string            `json:"cardNumber"`
	IsFrozen     bool              `json:"isFrozen"`
	Transactions []CardTransaction `json:"transactions"`
}

// Address is a customer's street address
type Address struct {
	Line1 string `json:"line1"`
	Line2 string `json:"line2"`
}

// Customer is a person, also a top level record
type Customer struct {
	ID        string    `json:"_id"`
	About     string    `json:"about"`
	Addresses []Address `json:"addresses"`
	Balance   string    `json:"balance"`
	Company   string    `json:"company"`
	Email     string    `json:"email"`
	GUID      string    `json:"guid"`
	Index     int64     `json:"index"`
	IsActive  bool      `json:"isActive"`
	Latitude  float64   `json:"latitude"`
	Longitude float64   `json:"longitude"`
	Name      struct {
		First string `json:"first"`
		Last  string `json:"last"`
	} `json:"name"`
	Phone      string    `json:"phone"`
	Picture    string    `json:"picture"`
	Registered string    `json:"registered"`
	Tags       []string  `json:"tags"`
	Accounts   []Account `json:"accounts"`
	Cards      []Card    `json:"cards"`
}
