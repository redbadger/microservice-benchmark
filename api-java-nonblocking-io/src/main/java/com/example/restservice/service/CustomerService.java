package com.example.restservice.service;

import com.example.restservice.model.Customer;

import org.springframework.http.MediaType;
import org.springframework.stereotype.Service;
import org.springframework.web.reactive.function.client.WebClient;

import reactor.core.publisher.Mono;

@Service
public class CustomerService {
    private final WebClient webClient;

    public CustomerService(WebClient webClient) {
        this.webClient = webClient;
    }

    public Mono<Customer> getCustomer() {
        String customerResourceUrl = System.getenv("API_CUSTOMER");
        return webClient.get().uri(customerResourceUrl).accept(MediaType.APPLICATION_JSON).retrieve()
                .bodyToMono(Customer.class);
    }
}
