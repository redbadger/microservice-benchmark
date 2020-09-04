package com.example.restservice.model;

import java.util.ArrayList;
import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Account {
    @JsonProperty("_id")
    private String id;
    private int index;
    private UUID guid;
    @JsonProperty("is_active")
    private boolean isActive;
    private String balance;
    private ArrayList<AccountTransaction> transactions;

    public Account() {
    }

    public Account(String id, int index, UUID guid, boolean isActive, String balance,
            ArrayList<AccountTransaction> transactions) {
        this.setId(id);
        this.setIndex(index);
        this.setGuid(guid);
        this.setIsActive(isActive);
        this.setBalance(balance);
        this.setTransactions(transactions);
    }

    public ArrayList<AccountTransaction> getTransactions() {
        return transactions;
    }

    public void setTransactions(ArrayList<AccountTransaction> transactions) {
        this.transactions = transactions;
    }

    public String getBalance() {
        return balance;
    }

    public void setBalance(String balance) {
        this.balance = balance;
    }

    public boolean isActive() {
        return isActive;
    }

    public void setIsActive(boolean isActive) {
        this.isActive = isActive;
    }

    public UUID getGuid() {
        return guid;
    }

    public void setGuid(UUID guid) {
        this.guid = guid;
    }

    public int getIndex() {
        return index;
    }

    public void setIndex(int index) {
        this.index = index;
    }

    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }
}
