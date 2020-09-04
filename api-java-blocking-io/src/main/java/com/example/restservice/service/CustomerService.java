package com.example.restservice.service;

import java.util.concurrent.CompletableFuture;

import com.example.restservice.model.Customer;

import org.springframework.boot.web.client.RestTemplateBuilder;
import org.springframework.http.ResponseEntity;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;

@Service
public class CustomerService {
    private final RestTemplate restTemplate;

    public CustomerService(RestTemplateBuilder restTemplateBuilder) {
        this.restTemplate = restTemplateBuilder.build();
    }

    @Async
    public CompletableFuture<Customer> getCustomer() throws InterruptedException {
        String customerResourceUrl = System.getenv("API_CUSTOMER");
        ResponseEntity<Customer> response = restTemplate.getForEntity(customerResourceUrl, Customer.class);
        Customer customer = response.getBody();

        return CompletableFuture.completedFuture(customer);
    }

}
