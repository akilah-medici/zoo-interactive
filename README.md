# Zoo Interactive

Este projeto é uma aplicação web para gerenciamento de dados de um zoológico, incluindo animais, cuidados e registros relacionados. O sistema é composto por um backend em Rust (Axum) e um frontend em React (Vite), além de um banco de dados SQL Server.

## Funcionalidades
- **Cadastro e listagem de animais:**
  - Permite registrar novos animais com informações detalhadas (nome, espécie, data de nascimento, etc.)
  - Visualização de todos os animais cadastrados em uma lista interativa
- **Registro de cuidados e interações:**
  - Adição de registros de cuidados realizados (alimentação, limpeza, saúde)
  - Associação dos cuidados a cada animal, com histórico completo
  - Visualização dos cuidados
- **Interface interativa para visualização e modificação dos dados:**
  - Dashboard com estatísticas e gráficos sobre o zoológico
  - Páginas para modificar dados de animais e cuidados
  - Navegação intuitiva entre diferentes seções do sistema

## Estrutura do Projeto
- **backend/**: API REST desenvolvida em Rust, responsável pela lógica de negócio e acesso ao banco de dados
- **frontend/**: Interface web desenvolvida em React
- **sql/**: Scripts para criação e população do banco de dados
- **docker-compose.yml**: Orquestração dos serviços via Docker

# Imagens do projeto

![Tela incial](https://drive.google.com/uc?export=view&id=1z8WZucqMQ7vD5idRRpXgtS_0dtloYuOE)

![Listar e criar animais](https://drive.google.com/uc?export=view&id=1ks1LvmCBTI_7rJg3fNubf3pXmeeNnpqt)

![Modifcar animais](https://drive.google.com/uc?export=view&id=1zauPGb_Afx5MEGcpA2oncqyy9oBekLIU)


## Desenvolvimento e Deploy
O desenvolvimento foi realizado pensando em um futuro deploy em ambientes cloud, como Heroku. O banco de dados pode ser facilmente migrado para um serviço gerenciado (ex: Azure SQL Database) e os containers podem ser integrados a serviços como Azure Container Instances ou Kubernetes.

O processo de desenvolvimento se deu da seguinte forma:

Primeiro foi pensado em como deveria ser o design do aplicativo e em como seria experiencia do usuário, ou seja, quais telas haveriam, o que cada tela conteria, e como o fluxo de uso do aplicativo seria pretendido
![Modelo conceitual do site](https://drive.google.com/uc?export=view&id=1BRNDzLUzZEJfL1LJb1T1HRBK_hQ9Jt1l)

Após isso foi pensado em como estruturar o banco de dados, os modelos foram feitos no brModelo, e depois foi gerado o modelo físico. o modelo final se da pelo diagrama lógico abaixo.
![Modelo lógico do banco de dados](https://drive.google.com/uc?export=view&id=12k6YwfNxrI2WAygF6wSfteIdxvHi_UqO)

Após isso se deu inicio ao desenvolvimento, criando o container no Docker e inserido a aplicação em Rust/REACT dentro de seus respectivos containers, e foram utilizadas estas ferramentas por serem robustas, e REACT sendo uma facil de se aprender e trabalhar.


## Como iniciar o projeto
Consulte os arquivos `INICIAR-PROJETO-DOCKER.md` (Windows) ou `INICIAR-PROJETO-DOCKER-LINUX.md` (Linux) para instruções detalhadas de inicialização local via Docker.

## Dificuldades e expectativas

O projeto incialmente tinha como objetivo a total integração a um ambiente web, com o deploy do projeto feito na plataforma Heroku
e a conectividade ao banco de dados hospedado na Azure, mas por conta da complexidade de se relizar tais implementações no perido de tempo
restante, estas funcionalidades ficaram de fora do projeto, com ele sendo executado apenas localmente para teste.

Fora estas cosiderações há outras a se fazer, como por exemplo, algumas consultas ao banco de dados e requisições funcionam para projetos pequenos e de aprendizado como este, mas pensando na futura escalabilidade que um projeto parecido possa ter, é necessário realizar varias otimizações, como por exemplo, em meu projeto o Frontend requisita a lista de animais e o Backend faz apenas um SELECT na tabela de Animais, e traz todas as linhas da tabela para o Frontend copiando desnecessariamente toda a estrutura de dados, o que poderia ser resolvido através de uma implementação por filtros ou algo do tipo, bem como a forma com que é feita a busca de cuidados pelo id de cada animal, sendo feita algumas consultas de forma desnecessário, que é algo que poderia ser resolvido por um JOIN entre a tabela de cuidados e a tabela relacionamento.

Por ultimo há uma questão com o requisitado no enunciado do projeto, sendo o requisito de um CRUD da tabela de Cuidados, as funcionalidades de UPDATE e DELETE foram implementadas no Backend mas não foi possivel implementa-las no Frontend.

Dadas estas considerações, posso dizer que foi um projeto bem interessante de divertido de se realizar, que me permitiu aprender e crescer um pouco mais.


