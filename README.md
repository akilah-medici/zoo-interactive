# Zoo Interactive

Este projeto é uma aplicação web para gerenciamento de dados de um zoológico, incluindo animais, cuidados e registros relacionados. O sistema é composto por um backend em Rust (Axum) e um frontend em React (Vite), além de um banco de dados SQL Server.

## Funcionalidades
- **Cadastro e listagem de animais:**
  - Permite registrar novos animais com informações detalhadas (nome, espécie, data de nascimento, etc.)
  - Visualização de todos os animais cadastrados em uma lista interativa
  - Filtros por espécie, idade e outros atributos
- **Registro de cuidados e interações:**
  - Adição de registros de cuidados realizados (alimentação, limpeza, saúde)
  - Associação dos cuidados a cada animal, com histórico completo
  - Visualização dos cuidados pendentes ou realizados recentemente
- **Interface interativa para visualização e modificação dos dados:**
  - Dashboard com estatísticas e gráficos sobre o zoológico
  - Páginas para modificar dados de animais e cuidados
  - Navegação intuitiva entre diferentes seções do sistema
- **Controle de usuários e permissões (planejado):**
  - Possibilidade de diferentes níveis de acesso para administradores e cuidadores
  - Auditoria de alterações e registros

## Estrutura do Projeto
- **backend/**: API REST desenvolvida em Rust, responsável pela lógica de negócio e acesso ao banco de dados
- **frontend/**: Interface web desenvolvida em React
- **sql/**: Scripts para criação e população do banco de dados
- **docker-compose.yml**: Orquestração dos serviços via Docker

## Desenvolvimento e Deploy
O desenvolvimento foi realizado pensando em um futuro deploy em ambientes cloud, como Azure. O banco de dados pode ser facilmente migrado para um serviço gerenciado (ex: Azure SQL Database) e os containers podem ser integrados a serviços como Azure Container Instances ou Kubernetes.

## Como iniciar o projeto
Consulte os arquivos `INICIAR-PROJETO-DOCKER.md` (Windows) ou `INICIAR-PROJETO-DOCKER-LINUX.md` (Linux) para instruções detalhadas de inicialização local via Docker.

