package com.example.restservice;

import static org.springframework.http.MediaType.APPLICATION_JSON;
import static org.springframework.web.reactive.function.server.ServerResponse.ok;

import com.example.restservice.model.Account;
import com.example.restservice.model.Card;
import com.example.restservice.model.Customer;
import com.example.restservice.service.AccountService;
import com.example.restservice.service.CardService;
import com.example.restservice.service.CustomerService;

import org.springframework.stereotype.Component;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;

import reactor.core.publisher.Mono;

@Component
public class CustomerHandler {
    private final AccountService accountService;
    private final CardService cardService;
    private final CustomerService customerService;

    public CustomerHandler(AccountService accountService, CardService cardService, CustomerService customerService) {
        this.accountService = accountService;
        this.cardService = cardService;
        this.customerService = customerService;
    }

    public Mono<ServerResponse> getCustomer(ServerRequest request) {
        Mono<Account[]> accounts = accountService.getAccounts();
        Mono<Card[]> cards = cardService.getCards();
        Mono<Customer> customer = customerService.getCustomer();

        return Mono.zip(customer, accounts, cards).flatMap(data -> {
            Customer c = data.getT1();
            c.setAccounts(data.getT2());
            c.setCards(data.getT3());
            return ok().contentType(APPLICATION_JSON).bodyValue(c);
        });
    }
}
