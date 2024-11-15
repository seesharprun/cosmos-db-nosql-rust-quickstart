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

# Quickstart: Azure Cosmos DB for NoSQL client library for Go

This is a simple web application to illustrate common basic usage of Azure Cosmos DB for NoSQL's client library for Rust. This sample application accesses an existing account, database, and container using the [`azure_data_cosmos`](https://docs.rs/azure_data_cosmos) and [`azure_identity`](https://docs.rs/azure_identity) crates.

## Prerequisites

- [Docker](https://www.docker.com/)
- [Azure Developer CLI](https://aka.ms/azd-install)
- [Rust 1+](https://go.dev/dl/)

## Quickstart

1. Log in to Azure Developer CLI.

    ```bash
    azd auth login
    ```

    > [!TIP]
    > This is only required once per-install.

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