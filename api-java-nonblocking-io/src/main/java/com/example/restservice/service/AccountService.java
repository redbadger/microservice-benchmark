package com.example.restservice.service;

import com.example.restservice.model.Account;

import org.springframework.http.MediaType;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.client.WebClient;

import reactor.core.publisher.Mono;

@Service
public class AccountService {
    private final WebClient webClient;

    public AccountService(WebClient webClient) {
        this.webClient = webClient;
    }

    public Mono<Account[]> getAccounts() {
        String accountsResourceUrl = System.getenv("API_ACCOUNTS");
        return webClient.get().uri(accountsResourceUrl).accept(MediaType.APPLICATION_JSON).retrieve()
                .bodyToMono(Account[].class);
    }
}
