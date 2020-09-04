package com.example.restservice.model;

import java.util.ArrayList;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Card {
    @JsonProperty("_id")
    private String id;
    @JsonProperty("is_frozen")
    private boolean isFrozen;
    @JsonProperty("card_number")
    private String cardNumber;
    private ArrayList<CardTransaction> transactions;

    public Card() {
    }

    public Card(String id, boolean isFrozen, String cardNumber, ArrayList<CardTransaction> transactions) {
        this.setId(id);
        this.setIsFrozen(isFrozen);
        this.setCardNumber(cardNumber);
        this.setTransactions(transactions);
    }

    public ArrayList<CardTransaction> getTransactions() {
        return transactions;
    }

    public void setTransactions(ArrayList<CardTransaction> transactions) {
        this.transactions = transactions;
    }

    public String getCardNumber() {
        return cardNumber;
    }

    public void setCardNumber(String cardNumber) {
        this.cardNumber = cardNumber;
    }

    public boolean isFrozen() {
        return isFrozen;
    }

    public void setIsFrozen(boolean isFrozen) {
        this.isFrozen = isFrozen;
    }

    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }
}
