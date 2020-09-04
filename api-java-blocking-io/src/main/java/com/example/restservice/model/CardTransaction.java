package com.example.restservice.model;

import java.util.Date;

public class CardTransaction {
    private long amount;
    private String currency;
    private String merchant;
    private Date timestamp;

    public CardTransaction() {
    }

    public CardTransaction(long amount, String currency, String merchant, Date timestamp) {
        this.setAmount(amount);
        this.setCurrency(currency);
        this.setMerchant(merchant);
        this.setTimestamp(timestamp);
    }

    public Date getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Date timestamp) {
        this.timestamp = timestamp;
    }

    public String getMerchant() {
        return merchant;
    }

    public void setMerchant(String merchant) {
        this.merchant = merchant;
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
