import express from "express";
import fetch from "node-fetch";

import { Customer, Account, Card } from "./model";

const app = express();
const PORT = 8000;

const API_CUSTOMER = process.env["API_CUSTOMER"];
const API_ACCOUNTS = process.env["API_ACCOUNTS"];
const API_CARDS = process.env["API_CARDS"];

app.get("/", (_, res) => {
  const customer: Promise<Customer> = fetch(API_CUSTOMER).then((res) =>
    res.json()
  );
  const accounts: Promise<Account[]> = fetch(API_ACCOUNTS).then((res) =>
    res.json()
  );
  const cards: Promise<Card[]> = fetch(API_CARDS).then((res) => res.json());

  Promise.all([customer, accounts, cards]).then(
    ([customer, accounts, cards]) => {
      customer.accounts = accounts;
      customer.cards = cards;

      res.json(customer);
    }
  );
});

app.get("/healthz", (_, res) => {
  res.sendStatus(204);
});

app.listen(PORT, () => console.log("Server listening on port ", PORT));
