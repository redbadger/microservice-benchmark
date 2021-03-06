package com.example.restservice.service;

import java.util.concurrent.CompletableFuture;

import com.example.restservice.model.Account;

import org.springframework.boot.web.client.RestTemplateBuilder;
import org.springframework.http.ResponseEntity;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;

@Service
public class AccountService {
    private final RestTemplate restTemplate;

    public AccountService(RestTemplateBuilder restTemplateBuilder) {
        this.restTemplate = restTemplateBuilder.build();
    }

    @Async
    public CompletableFuture<Account[]> getAccounts() throws InterruptedException {
        String accountsResourceUrl = System.getenv("API_ACCOUNTS");
        ResponseEntity<Account[]> response = restTemplate.getForEntity(accountsResourceUrl, Account[].class);
        Account[] accounts = response.getBody();

        return CompletableFuture.completedFuture(accounts);
    }

}
