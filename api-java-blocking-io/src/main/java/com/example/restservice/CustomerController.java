package com.example.restservice;

import java.util.concurrent.CompletableFuture;

import com.example.restservice.model.Account;
import com.example.restservice.model.Card;
import com.example.restservice.model.Customer;
import com.example.restservice.service.AccountService;
import com.example.restservice.service.CardService;
import com.example.restservice.service.CustomerService;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class CustomerController {

	private final AccountService accountService;
	private final CardService cardService;
	private final CustomerService customerService;

	public CustomerController(AccountService accountService, CardService cardService, CustomerService customerService) {
		this.accountService = accountService;
		this.cardService = cardService;
		this.customerService = customerService;
	}

	@GetMapping("/")
	public Customer getCustomer() throws Exception {
		CompletableFuture<Account[]> accountsFuture = accountService.getAccounts();
		CompletableFuture<Card[]> cardsFuture = cardService.getCards();
		CompletableFuture<Customer> customerFuture = customerService.getCustomer();

		CompletableFuture.allOf(accountsFuture, cardsFuture, customerFuture).join();

		Customer customer = customerFuture.get();
		customer.setAccounts(accountsFuture.get());
		customer.setCards(cardsFuture.get());

		return customer;
	}
}
