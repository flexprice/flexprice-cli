# FlexPrice CLI (npm package)

TypeScript implementation of the FlexPrice terminal CLI. Requires **Node.js 20+**.

## Development

```bash
cd npm
npm install
npm run build
node dist/cli-entry.js --help
```

## Global install (from this repo)

```bash
npm install -g ./npm
flexprice --help
```

## Publish to npm

From this directory:

```bash
npm run build
npm publish
```

Ensure the package `name` in `package.json` is available on the npm registry (consider a scoped name such as `@flexprice/cli` if `flexprice-cli` is taken).

## Scripts

- `npm run build` — bundle CLI to `dist/cli-entry.js` (tsup)
- `npm run dev` — watch mode with `tsx`
- `npm run typecheck` — `tsc --noEmit`
