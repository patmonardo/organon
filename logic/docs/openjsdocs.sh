# Open API docs locally
xdg-open /home/pat/VSCode/organon/logic/docs/api/index.html

# Regenerate TypeDoc (HTML)
pnpm dlx typedoc \
  --tsconfig /home/pat/VSCode/organon/logic/tsconfig.json \
  --entryPoints "/home/pat/VSCode/organon/logic/src/schema/*.ts" "/home/pat/VSCode/organon/logic/src/form/**/*.ts" \
  --exclude "**/*.test.ts" \
  --readme /home/pat/VSCode/organon/logic/README.md \
  --out /home/pat/VSCode/organon/logic/docs/api
