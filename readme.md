<!--
---
page_type: sample
name: "Quickstart: Azure Cosmos DB for NoSQL and Azure SDK for Rust"
description: This is a simple web application to illustrate common basic usage of Azure Cosmos DB for NoSQL and the Azure SDK for Rust.
urlFragment: template
languages:
- rust
- azdeveloper
products:
- azure-cosmos-db
---
-->

# Quickstart: Azure Cosmos DB for NoSQL - Azure SDK for Rust

This is a simple web application to illustrate common basic usage of Azure Cosmos DB for NoSQL with the Azure SDK for Rust.

## Prerequisites

- [Docker](https://www.docker.com/)
- [Azure Developer CLI](https://aka.ms/azd-install)
- [Rust 1+](https://go.dev/dl/)

## Quickstart

1. Log in to Azure Developer CLI. *This is only required once per-install.*

    ```bash
    azd auth login
    ```

1. Initialize this template (`cosmos-db-nosql-rust-quickstart`) using `azd init`

    ```bash
    azd init --template cosmos-db-nosql-rust-quickstart
    ```

1. Ensure that **Docker** is running in your environment.

1. Use `azd up` to provision your Azure infrastructure and deploy the web application to Azure.

    ```bash
    azd up
    ```

1. Observed the deployed web application

    ![Screenshot of the deployed web application.](assets/web.png)
