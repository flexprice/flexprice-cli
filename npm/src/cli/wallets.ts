import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { ListResponse, Wallet, WalletBalance } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, optionalInput, number } from './prompts.js';
import type { GlobalOpts } from './types.js';

export async function walletsList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching wallets...');
  const resp = (await client.get('/v1/wallets')) as ListResponse<Wallet>;
  sp.stop();
  const rows = (resp.items ?? []).map((w) => ({
    ID: w.id ?? '',
    Customer: w.customer_id ?? '',
    Balance: w.balance !== undefined ? w.balance.toFixed(2) : '',
    Currency: w.currency ?? '',
    Status: w.wallet_status ? output.statusBadge(w.wallet_status) : '',
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function walletsGet(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching wallet...');
  const wallet = (await client.get(`/v1/wallets/${id}`)) as Wallet;
  sp.stop();
  process.stdout.write(output.printDetail(wallet, json));
}

export async function walletsCreate(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Create a new wallet (press Enter to use defaults)');
    const customer_id = await requiredInput('Customer ID:');
    const currency = await requiredInput('Currency (e.g. USD):');
    const initial_balance = await number({
      message: 'Initial balance (default: 0):',
      default: 0,
    });
    body = { customer_id, currency, initial_balance: initial_balance ?? 0 };
  }

  const sp = createSpinner('Creating wallet...');
  const wallet = (await client.post('/v1/wallets', body)) as Wallet;
  sp.stop();
  output.success(`Wallet created: ${wallet.id ?? ''}`);
  process.stdout.write(output.printDetail(wallet, false));
}

export async function walletsTopUp(
  globalOpts: GlobalOpts,
  id: string,
  file?: string
): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info(`Top up wallet ${id}`);
    const credits = await number({
      message: 'Credits to add:',
      validate: (v) => (v !== undefined && v > 0 ? true : 'Must be greater than 0'),
    });
    const description = await optionalInput('Description (optional):');
    body = { credits, ...(description ? { description } : {}) };
  }

  const sp = createSpinner('Topping up wallet...');
  const resp = await client.post(`/v1/wallets/${id}/top-up`, body);
  sp.stop();
  output.success(`Wallet ${id} topped up.`);
  process.stdout.write(output.printDetail(resp, false));
}

export async function walletsBalance(
  globalOpts: GlobalOpts,
  id: string,
  json: boolean
): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching balance...');
  const balance = (await client.get(`/v1/wallets/${id}/balance/real-time`)) as WalletBalance;
  sp.stop();
  process.stdout.write(output.printDetail(balance, json));
}
