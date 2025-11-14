## Frontend - desenvolvimento local

Este diretório contém a aplicação SvelteKit que consome a API do backend.

Passos rápidos para rodar localmente:

1. Copie o arquivo de exemplo de variáveis de ambiente:

   - Windows (PowerShell):
     - copie `frontend/.env.example` para `frontend/.env`

2. Ajuste `PRIVATE_API_BASE` caso seu backend rode em outra porta/endereço (ex.: `http://localhost:3000`).

3. No terminal, instale dependências e rode o dev server:

   - Instalar: `npm install`
   - Rodar: `npm run dev -- --host`

4. Execute o backend separadamente (cargo run no diretório `beckend`) apontando para o mesmo `PRIVATE_API_BASE`.

Notas de segurança e deploy

- Não comite arquivos `.env`; `frontend/.env` está listado no `.gitignore`.
- Em produção, configure `PRIVATE_API_BASE` no ambiente do servidor (por exemplo, Vercel, Netlify, Fly, etc.).
- Se preferir expor o backend diretamente ao cliente (não recomendado para segredos), use `PUBLIC_API_BASE` no `.env` e referências em código cliente — lembrando que tudo em `PUBLIC_` é incorporado no bundle e visível.

Problemas comuns

- Se as chamadas falharem com 502/504, verifique se o backend está rodando e acessível a partir do servidor onde o SvelteKit está sendo executado.
# sv

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```sh
# create a new project in the current directory
npx sv create

# create a new project in my-app
npx sv create my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```sh
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```sh
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
