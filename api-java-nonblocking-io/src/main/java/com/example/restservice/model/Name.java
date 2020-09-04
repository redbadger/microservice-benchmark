package com.example.restservice.model;

public class Name {
    private String first;
    private String last;

    public Name() {
    }

    public Name(String first, String last) {
        this.setFirst(first);
        this.setLast(last);
    }

    public String getLast() {
        return last;
    }

    public void setLast(String last) {
        this.last = last;
    }

    public String getFirst() {
        return first;
    }

    public void setFirst(String first) {
        this.first = first;
    }
}
