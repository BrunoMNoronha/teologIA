\<\!-- ./README.md \--\>

# **TeologIA \- Motor de Sabedoria Digital 🕊️**

**SaaS de Consulta Teológica Impulsionada por IA \- Powered by [Dev2Bless](https://dev2bless.com.br)**

A plataforma **TeologIA** é um assistente virtual especializado em teologia. O sistema hospeda o **"Eli"**, um Mestre Teólogo Digital concebido para democratizar o acesso a debates bíblicos, históricos e teológicos profundos, outrora restrito a plataformas de IA pagas.

## **🏛️ Princípio Arquitetural: "Zero-Bloat" & "Monorepo"**

O pilar fundamental deste projeto é a **escassez intencional de dependências** (Zero-Bloat). Privilegiamos o uso de APIs nativas das linguagens base.

O projeto utiliza uma arquitetura **Monorepo**, garantindo que as ferramentas de IA (Codex, Copilot, Gemini) tenham contexto integral (Full-Stack) sobre o Frontend, Backend e Base de Conhecimento simultaneamente.

## **📁 Estrutura do Monorepo**

teologIA/  
├── backend/       \# Motor Rust (Axum, SQLx, SQLite)  
├── frontend/      \# Interface Vue 3 (Vite, Tailwind v4, Fetch API)  
├── doc/           \# Base de Conhecimento RAG (PDFs e Prompts da Persona)  
├── AGENTES.md     \# Leis e Personas para a orquestração de IA  
├── OBJETIVO.md    \# Visão, ética e a origem do "Eli"  
├── ROADMAP.md     \# O mapa de evolução tática do projeto  
├── VISAO.md       \# O manifesto de origem e conhecimento  
└── README.md      \# Este arquivo

## **🚀 Como Executar o Sistema Localmente**

**1\. Levantar o Motor (Backend)**

cd backend  
export GEMINI\_API\_KEY="sua\_chave\_aqui"  
cargo run

**2\. Levantar a Interface (Frontend)**

Em outro terminal:

cd frontend  
pnpm install  
pnpm dev

Acesse http://localhost:5173 no seu navegador.
