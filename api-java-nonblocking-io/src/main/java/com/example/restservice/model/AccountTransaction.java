package com.example.restservice.model;

public class AccountTransaction {
    private long amount;
    private String currency;
    private String from;

    public AccountTransaction() {
    }

    public AccountTransaction(long amount, String currency, String from) {
        this.setAmount(amount);
        this.setCurrency(currency);
        this.setFrom(from);
    }

    public String getFrom() {
        return from;
    }

    public void setFrom(String from) {
        this.from = from;
    }

    public String getCurrency() {
        return currency;
    }

    public void setCurrency(String currency) {
        this.currency = currency;
    }

    public long getAmount() {
        return amount;
    }

    public void setAmount(long amount) {
        this.amount = amount;
    }
}
