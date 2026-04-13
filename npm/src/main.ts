import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Command } from 'commander';
import { ApiError } from './api/client.js';
import * as authCmd from './cli/auth.js';
import * as customers from './cli/customers.js';
import * as entitlements from './cli/entitlements.js';
import * as events from './cli/events.js';
import * as features from './cli/features.js';
import * as invoices from './cli/invoices.js';
import * as meters from './cli/meters.js';
import * as plans from './cli/plans.js';
import * as subscriptions from './cli/subscriptions.js';
import type { GlobalOpts } from './cli/types.js';
import * as wallets from './cli/wallets.js';
import { credentialsPath, loadCredentials, maskedApiKey } from './config/store.js';
import { runDashboard } from './tui/dashboard.js';
import * as output from './utils/output.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const pkg = JSON.parse(readFileSync(join(__dirname, '../package.json'), 'utf-8')) as {
  version: string;
};

function globalOpts(cmd: Command): GlobalOpts {
  const o = cmd.optsWithGlobals() as { apiUrl?: string; apiKey?: string };
  return { apiUrl: o.apiUrl, apiKey: o.apiKey };
}

function runConfig(cmd: Command): void {
  const opts = globalOpts(cmd);
  const creds = loadCredentials(opts.apiUrl, opts.apiKey);
  console.log();
  output.info(`API URL:     ${creds.api_url.length === 0 ? '(not set)' : creds.api_url}`);
  output.info(`API Key:     ${maskedApiKey(creds)}`);
  output.info(`Env ID:      ${creds.environment_id ?? '(not set)'}`);
  output.info(`Config path: ${credentialsPath()}`);
  console.log();
}

export async function run(argv: string[]): Promise<void> {
  const program = new Command()
    .name('flexprice')
    .description('⚡ FlexPrice CLI — Usage-based billing, from your terminal.')
    .version(pkg.version)
    .option('--api-url <url>', 'Override the API base URL')
    .option('--api-key <key>', 'Override the API key');

  program.showHelpAfterError('(add --help for usage)');

  program
    .command('config')
    .description('Show current configuration')
    .action((_opts, cmd) => {
      runConfig(cmd);
    });

  program
    .command('dashboard')
    .description('Launch the interactive TUI dashboard')
    .action(async (_opts, cmd) => {
      const creds = authCmd.requireAuth(globalOpts(cmd));
      await runDashboard(creds);
    });

  const auth = program.command('auth').description('Authenticate with FlexPrice (API key)');

  auth
    .command('set-api-key')
    .description('Store an API key (default URL: cloud API)')
    .argument('<key>', 'The API key to store')
    .option('--api-url <url>', 'API endpoint URL', authCmd.DEFAULT_API_URL)
    .action(async (key: string, opts: { apiUrl: string }) => {
      await authCmd.setApiKey(key, opts.apiUrl);
    });

  auth
    .command('whoami')
    .description('Show current authenticated user and tenant')
    .action(async (_opts, cmd) => {
      await authCmd.whoami(globalOpts(cmd));
    });

  auth.command('status').description('Show authentication status').action(async () => {
    await authCmd.status();
  });

  auth.command('logout').description('Remove stored credentials').action(() => {
    authCmd.logout();
  });

  const customersCmd = program.command('customers').description('Manage customers');
  customersCmd
    .command('list')
    .description('List all customers')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await customers.customersList(globalOpts(cmd), opts.json);
    });
  customersCmd
    .command('get')
    .description('Get a customer by ID')
    .argument('<id>', 'Customer ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await customers.customersGet(globalOpts(cmd), id, opts.json);
    });
  customersCmd
    .command('create')
    .description('Create a new customer (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file with customer data (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await customers.customersCreate(globalOpts(cmd), opts.json);
    });
  customersCmd
    .command('delete')
    .description('Delete a customer by ID')
    .argument('<id>', 'Customer ID')
    .action(async (id: string, _opts, cmd) => {
      await customers.customersDelete(globalOpts(cmd), id);
    });
  customersCmd
    .command('usage')
    .description('View customer usage summary')
    .argument('<id>', 'Customer ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await customers.customersUsage(globalOpts(cmd), id, opts.json);
    });
  customersCmd
    .command('entitlements')
    .description('View customer entitlements')
    .argument('<id>', 'Customer ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await customers.customersEntitlements(globalOpts(cmd), id, opts.json);
    });

  const plansCmd = program.command('plans').description('Manage pricing plans');
  plansCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await plans.plansList(globalOpts(cmd), opts.json);
    });
  plansCmd
    .command('get')
    .argument('<id>', 'Plan ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await plans.plansGet(globalOpts(cmd), id, opts.json);
    });
  plansCmd
    .command('create')
    .description('Create a new plan (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await plans.plansCreate(globalOpts(cmd), opts.json);
    });
  plansCmd
    .command('delete')
    .argument('<id>', 'Plan ID')
    .action(async (id: string, _opts, cmd) => {
      await plans.plansDelete(globalOpts(cmd), id);
    });

  const subCmd = program.command('subscriptions').description('Manage subscriptions');
  subCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await subscriptions.subscriptionsList(globalOpts(cmd), opts.json);
    });
  subCmd
    .command('get')
    .argument('<id>', 'Subscription ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await subscriptions.subscriptionsGet(globalOpts(cmd), id, opts.json);
    });
  subCmd
    .command('create')
    .description('Create a new subscription (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await subscriptions.subscriptionsCreate(globalOpts(cmd), opts.json);
    });
  subCmd
    .command('cancel')
    .argument('<id>', 'Subscription ID')
    .action(async (id: string, _opts, cmd) => {
      await subscriptions.subscriptionsCancel(globalOpts(cmd), id);
    });
  subCmd
    .command('usage')
    .requiredOption('--json <file>', 'JSON body for usage query')
    .action(async (opts: { json: string }, cmd) => {
      await subscriptions.subscriptionsUsage(globalOpts(cmd), opts.json);
    });

  const invCmd = program.command('invoices').description('Manage invoices');
  invCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await invoices.invoicesList(globalOpts(cmd), opts.json);
    });
  invCmd
    .command('get')
    .argument('<id>', 'Invoice ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await invoices.invoicesGet(globalOpts(cmd), id, opts.json);
    });
  invCmd
    .command('finalize')
    .argument('<id>', 'Invoice ID')
    .action(async (id: string, _opts, cmd) => {
      await invoices.invoicesFinalize(globalOpts(cmd), id);
    });
  invCmd
    .command('void')
    .argument('<id>', 'Invoice ID')
    .action(async (id: string, _opts, cmd) => {
      await invoices.invoicesVoid(globalOpts(cmd), id);
    });
  invCmd
    .command('pdf')
    .argument('<id>', 'Invoice ID')
    .option('-o, --output <path>', 'Output file path', 'invoice.pdf')
    .action(async (id: string, opts: { output: string }, cmd) => {
      await invoices.invoicesPdf(globalOpts(cmd), id, opts.output);
    });

  const metersCmd = program.command('meters').description('Manage meters');
  metersCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await meters.metersList(globalOpts(cmd), opts.json);
    });
  metersCmd
    .command('get')
    .argument('<id>', 'Meter ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await meters.metersGet(globalOpts(cmd), id, opts.json);
    });
  metersCmd
    .command('create')
    .description('Create a new meter (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await meters.metersCreate(globalOpts(cmd), opts.json);
    });
  metersCmd
    .command('delete')
    .argument('<id>', 'Meter ID')
    .action(async (id: string, _opts, cmd) => {
      await meters.metersDelete(globalOpts(cmd), id);
    });

  const eventsCmd = program.command('events').description('Ingest and query events');
  eventsCmd
    .command('ingest')
    .description('Ingest a usage event (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file with event data (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await events.eventsIngest(globalOpts(cmd), opts.json);
    });
  eventsCmd
    .command('ingest-bulk')
    .requiredOption('--json <file>', 'Path to JSON file with bulk events')
    .action(async (opts: { json: string }, cmd) => {
      await events.eventsIngestBulk(globalOpts(cmd), opts.json);
    });
  eventsCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await events.eventsList(globalOpts(cmd), opts.json);
    });
  eventsCmd
    .command('get')
    .argument('<id>', 'Event ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await events.eventsGet(globalOpts(cmd), id, opts.json);
    });
  eventsCmd
    .command('usage')
    .requiredOption('--json <file>', 'JSON body for usage query')
    .action(async (opts: { json: string }, cmd) => {
      await events.eventsUsage(globalOpts(cmd), opts.json);
    });

  const walletsCmd = program.command('wallets').description('Manage wallets and credit balances');
  walletsCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await wallets.walletsList(globalOpts(cmd), opts.json);
    });
  walletsCmd
    .command('get')
    .argument('<id>', 'Wallet ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await wallets.walletsGet(globalOpts(cmd), id, opts.json);
    });
  walletsCmd
    .command('create')
    .description('Create a new wallet (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await wallets.walletsCreate(globalOpts(cmd), opts.json);
    });
  walletsCmd
    .command('top-up')
    .argument('<id>', 'Wallet ID')
    .description('Top up a wallet (interactive, or provide --json <file>)')
    .option('--json [file]', 'JSON body with top-up details (skips interactive prompts)')
    .action(async (id: string, opts: { json?: string }, cmd) => {
      await wallets.walletsTopUp(globalOpts(cmd), id, opts.json);
    });
  walletsCmd
    .command('balance')
    .argument('<id>', 'Wallet ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await wallets.walletsBalance(globalOpts(cmd), id, opts.json);
    });

  const featuresCmd = program.command('features').description('Manage features');
  featuresCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await features.featuresList(globalOpts(cmd), opts.json);
    });
  featuresCmd
    .command('get')
    .argument('<id>', 'Feature ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await features.featuresGet(globalOpts(cmd), id, opts.json);
    });
  featuresCmd
    .command('create')
    .description('Create a new feature (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await features.featuresCreate(globalOpts(cmd), opts.json);
    });
  featuresCmd
    .command('delete')
    .argument('<id>', 'Feature ID')
    .action(async (id: string, _opts, cmd) => {
      await features.featuresDelete(globalOpts(cmd), id);
    });

  const entCmd = program.command('entitlements').description('Manage entitlements');
  entCmd
    .command('list')
    .option('--json', 'Output as JSON', false)
    .action(async (opts: { json: boolean }, cmd) => {
      await entitlements.entitlementsList(globalOpts(cmd), opts.json);
    });
  entCmd
    .command('get')
    .argument('<id>', 'Entitlement ID')
    .option('--json', 'Output as JSON', false)
    .action(async (id: string, opts: { json: boolean }, cmd) => {
      await entitlements.entitlementsGet(globalOpts(cmd), id, opts.json);
    });
  entCmd
    .command('create')
    .description('Create a new entitlement (interactive, or provide --json <file>)')
    .option('--json [file]', 'Path to JSON file (skips interactive prompts)')
    .action(async (opts: { json?: string }, cmd) => {
      await entitlements.entitlementsCreate(globalOpts(cmd), opts.json);
    });
  entCmd
    .command('delete')
    .argument('<id>', 'Entitlement ID')
    .action(async (id: string, _opts, cmd) => {
      await entitlements.entitlementsDelete(globalOpts(cmd), id);
    });

  program.configureHelp({ sortSubcommands: true });

  const tokens = argv.slice(2);
  if (tokens.length === 0) {
    program.outputHelp();
    return;
  }

  await program.parseAsync(argv);
}

export function mainError(err: unknown): void {
  if (err instanceof ApiError) {
    output.error(err.message);
    process.exitCode = 1;
    return;
  }
  if (err instanceof Error) {
    output.error(err.message);
    process.exitCode = 1;
    return;
  }
  output.error(String(err));
  process.exitCode = 1;
}
