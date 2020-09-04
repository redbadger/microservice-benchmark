package com.example.restservice.service;

import com.example.restservice.model.Card;

import org.springframework.http.MediaType;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.client.WebClient;

import reactor.core.publisher.Mono;

@Service
public class CardService {
    private final WebClient webClient;

    public CardService(WebClient webClient) {
        this.webClient = webClient;
    }

    public Mono<Card[]> getCards() {
        String cardsResourceUrl = System.getenv("API_CARDS");
        return webClient.get().uri(cardsResourceUrl).accept(MediaType.APPLICATION_JSON).retrieve()
                .bodyToMono(Card[].class);
    }
}
