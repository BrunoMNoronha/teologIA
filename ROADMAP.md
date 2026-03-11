\<\!-- ./ROADMAP.md \--\>

# **ROADMAP: Agent-Driven Development (ADD)**

Este roteiro define a esteira de evolução do **TeologIA** dentro da arquitetura Monorepo, alinhada à filosofia Zero-Bloat.

## **FASE 1: Fundação (Concluída ✅)**

* Frontend Vue 3 (Tailwind v4) e Backend Rust (Axum) conversam com sucesso (CORS liberado).  
* Eli (Gemini Flash) recebe e responde requisições simples.

## **FASE 2: Purismo e Memória Cognitiva (Atual 🟡)**

* **Passo 2.1 (Engenheiro Frontend):** Remover a dependência do axios em ./frontend/ e refatorar api.ts utilizando a **Fetch API** nativa.  
* **Passo 2.2 (Engenheiro Frontend):** Criar parser nativo (TypeScript \+ Regex) em ./frontend/ para formatar os retornos do Gemini (estrito a negrito, itálico, listas e \\n).  
* **Passo 2.3 (Engenheiro Backend):** Estabelecer a **Memória do Eli** em ./backend/. Ler o SQLite via SQLx e enviar as *últimas 5 interações* para o Gemini, criando contexto cognitivo sem estourar limite de tokens.

## **FASE 3: Integração do Conhecimento / RAG (Próxima ⚪)**

* **Passo 3.1 (Engenheiro Backend / IA):** Desenvolver mecanismo de leitura contextual. Quando houver perguntas históricas/exegéticas, o Rust fará RAG (Retrieval) lendo Chronological-Study-Bible.pdf e Biblia-King-James.pdf da pasta ./doc/ antes de montar o payload.  
* **Passo 3.2 (Engenheiro Frontend):** Ajustes finos de UX silênciosa (auto-scroll garantido, estados desabilitados seguros).

## **FASE 4: O Grande Lançamento (Deploy Final ⚪)**

* **Passo 4.1:** Build do frontend (pnpm build).  
* **Passo 4.2:** Configurar Nginx na VPS (vps.dev2bless.com.br) para servir estáticos e rotear /api para a porta local do Rust.  
* **Passo 4.3:** SSL Let's Encrypt para segurança de ponta a ponta.