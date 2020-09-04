package com.example.restservice.model;

import java.net.URL;
import java.util.Date;
import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Customer {
    @JsonProperty("_id")
    private String id;
    private int index;
    private UUID guid;
    @JsonProperty("is_active")
    private boolean isActive;
    private String balance;
    private URL picture;
    private Name name;
    private String company;
    private String email;
    private String phone;
    private Address[] addresses;
    private String about;
    private Date registered;
    private float latitude;
    private float longitude;
    private String[] tags;
    private Account[] accounts;
    private Card[] cards;

    public Customer() {
    }

    public Customer(String id, int index, UUID guid, boolean isActive, String balance, URL picture, Name name,
            String company, String email, String phone, Address[] addresses, String about, Date registered,
            float latitude, float longitude, String[] tags, Account[] accounts, Card[] cards) {
        this.setId(id);
        this.setIndex(index);
        this.setGuid(guid);
        this.setIsActive(isActive);
        this.setBalance(balance);
        this.setPicture(picture);
        this.setName(name);
        this.setCompany(company);
        this.setEmail(email);
        this.setPhone(phone);
        this.setAddresses(addresses);
        this.setAbout(about);
        this.setRegistered(registered);
        this.setLatitude(latitude);
        this.setLongitude(longitude);
        this.setTags(tags);
        this.setAccounts(accounts);
        this.setCards(cards);
    }

    public Card[] getCards() {
        return cards;
    }

    public void setCards(Card[] cards) {
        this.cards = cards;
    }

    public Account[] getAccounts() {
        return accounts;
    }

    public void setAccounts(Account[] accounts) {
        this.accounts = accounts;
    }

    public String[] getTags() {
        return tags;
    }

    public void setTags(String[] tags) {
        this.tags = tags;
    }

    public float getLongitude() {
        return longitude;
    }

    public void setLongitude(float longitude) {
        this.longitude = longitude;
    }

    public float getLatitude() {
        return latitude;
    }

    public void setLatitude(float latitude) {
        this.latitude = latitude;
    }

    public Date getRegistered() {
        return registered;
    }

    public void setRegistered(Date registered) {
        this.registered = registered;
    }

    public String getAbout() {
        return about;
    }

    public void setAbout(String about) {
        this.about = about;
    }

    public Address[] getAddresses() {
        return addresses;
    }

    public void setAddresses(Address[] addresses) {
        this.addresses = addresses;
    }

    public String getPhone() {
        return phone;
    }

    public void setPhone(String phone) {
        this.phone = phone;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public String getCompany() {
        return company;
    }

    public void setCompany(String company) {
        this.company = company;
    }

    public Name getName() {
        return name;
    }

    public void setName(Name name) {
        this.name = name;
    }

    public URL getPicture() {
        return picture;
    }

    public void setPicture(URL picture) {
        this.picture = picture;
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
