package com.example.restservice.service;

import java.util.concurrent.CompletableFuture;

import com.example.restservice.model.Card;

import org.springframework.boot.web.client.RestTemplateBuilder;
import org.springframework.http.ResponseEntity;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;

@Service
public class CardService {
    private final RestTemplate restTemplate;

    public CardService(RestTemplateBuilder restTemplateBuilder) {
        this.restTemplate = restTemplateBuilder.build();
    }

    @Async
    public CompletableFuture<Card[]> getCards() throws InterruptedException {
        String cardsResourceUrl = "http://localhost:3000/cards";
        ResponseEntity<Card[]> response = restTemplate.getForEntity(cardsResourceUrl, Card[].class);
        Card[] cards = response.getBody();

        return CompletableFuture.completedFuture(cards);
    }

}
