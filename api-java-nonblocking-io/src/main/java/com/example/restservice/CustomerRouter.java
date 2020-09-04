package com.example.restservice;

import static org.springframework.web.reactive.function.server.RequestPredicates.GET;
import static org.springframework.web.reactive.function.server.ServerResponse.status;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpStatus;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.RouterFunctions;
import org.springframework.web.reactive.function.server.ServerResponse;

@Configuration
public class CustomerRouter {
    @Bean
    public WebClient webClient() {
        return WebClient.builder().build();
    }

    @Bean
    public RouterFunction<ServerResponse> getCustomerRoute(CustomerHandler customerHandler) {
        return RouterFunctions.route(GET("/"), customerHandler::getCustomer);
    }

    @Bean
    RouterFunction<ServerResponse> getHealthRoute() {
        return RouterFunctions.route(GET("/healthz"), req -> status(HttpStatus.NO_CONTENT).build());
    }
}
