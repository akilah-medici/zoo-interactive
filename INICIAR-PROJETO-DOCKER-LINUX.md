# Guia de Inicialização do Projeto via Docker (Linux)

Este guia descreve como iniciar o projeto localmente em ambiente Linux utilizando Docker, incluindo os passos para buildar os containers, criar e popular o banco de dados, e iniciar os serviços.

## 1. Buildar e subir os containers

Execute o comando abaixo para buildar e iniciar os containers definidos no `docker-compose.yml`:

```bash
docker compose up --build
```

## 2. Inserir dados no banco de dados

### 2.1 Criar tabelas

Copie o script de criação de tabelas para o container do SQL Server e execute-o:

```bash
docker cp ./sql/create-database.sql sqlserver:/tmp/create-database.sql
docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P Password123 -d master -i /tmp/create-database.sql -C
```

### 2.2 Inserir dados iniciais

Copie o script de dados iniciais para o container e execute-o:

```bash
docker cp ./sql/initial-population-data.sql sqlserver:/tmp/initial-population-data.sql
docker exec sqlserver /opt/mssql-tools18/bin/sqlcmd -S localhost -U SA -P Password123 -d zoo_db -i /tmp/initial-population-data.sql -C
```

## 3. Iniciar o projeto após inserir os dados

### 3.1 Reiniciar os serviços

```bash
docker compose up -d
```

### 3.2 Reiniciar o container do backend

```bash
docker compose restart rust-backend
```

---

Siga estes passos para garantir que o ambiente esteja pronto para uso localmente via Docker em sistemas Linux.
