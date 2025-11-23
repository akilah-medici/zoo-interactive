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

# Imagens do projeto

![Tela incial](https://drive.google.com/uc?export=view&id=1z8WZucqMQ7vD5idRRpXgtS_0dtloYuOE)

![Listar e criar animais][(https://drive.google.com/file/d/1ks1LvmCBTI_7rJg3fNubf3pXmeeNnpqt/view?usp=sharing)](https://drive.google.com/file/d/1ks1LvmCBTI_7rJg3fNubf3pXmeeNnpqt/view?usp=sharing)

![Modifcar animais][(https://drive.google.com/file/d/1zauPGb_Afx5MEGcpA2oncqyy9oBekLIU/view?usp=sharing)](https://drive.google.com/file/d/1zauPGb_Afx5MEGcpA2oncqyy9oBekLIU/view?usp=sharing)


## Desenvolvimento e Deploy
O desenvolvimento foi realizado pensando em um futuro deploy em ambientes cloud, como Azure. O banco de dados pode ser facilmente migrado para um serviço gerenciado (ex: Azure SQL Database) e os containers podem ser integrados a serviços como Azure Container Instances ou Kubernetes.

## Como iniciar o projeto
Consulte os arquivos `INICIAR-PROJETO-DOCKER.md` (Windows) ou `INICIAR-PROJETO-DOCKER-LINUX.md` (Linux) para instruções detalhadas de inicialização local via Docker.

## Dificuldades e esxpectativas

O projeto incialmente tinha como objetivo a total integração a um ambiente web, com o deploy do projeto feito na plataforma Heroku
e a conectividade ao banco de dados hospedado na Azure, mas por conta da complexidade de se relizar tais implementações no perido de tempo
restante, estas funcionalidades ficaram de fora do projeto, com ele sendo rodado apenas localmente para teste.

Fora estas cosiderações há outras a se fazer, como por exemplo algumas consultas ao banco de dados e requisições funcionam para projetos pequenos e de aprendizado como este, mas pensando na futura escalabilidade que um projeto parecido possa ter, é necessário realizar varias otimizações, como por exemplo, ao Frontend requisitar a lista de animais, o back faz apenas um SELECT na tabela de Animais, e traz todas as linhas da tabela para o Frontend, copiando desnecessariamente toda a estrutura de dados, bem como a forma com que é feita a busca de cuidados pelo id de cada animal, sendo feita algumas consultas de forma desnecessário, que é algo que poderia ser resolvido por um JOIN entre a tabela de cuidados e a tabela relacionamento.

Por ultimo há outra questão com o requisitado no enunciado do projeto sendo um CRUD da tabela de Cuidados, as funcionalidades de UPDATE e DELETE foram implementadas no Backend mas não foi possivel implementa-las no Frontend.

Dadas estas considerações, posso dizer que foi um porjeto bem interessante de divertido de se realizar, que me permitiu aprender e crescer um pouco mais.


