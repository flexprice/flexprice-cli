import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { Customer, ListResponse } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, optionalInput } from './prompts.js';
import type { GlobalOpts } from './types.js';

export async function customersList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching customers...');
  const resp = (await client.get('/v1/customers')) as ListResponse<Customer>;
  sp.stop();
  const rows = (resp.items ?? []).map((c) => ({
    ID: c.id ?? '',
    Name: c.name ?? '',
    Email: c.email ?? '',
    'External ID': c.external_id ?? '',
    Status: c.status ? output.statusBadge(c.status) : '',
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function customersGet(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching customer...');
  const customer = (await client.get(`/v1/customers/${id}`)) as Customer;
  sp.stop();
  process.stdout.write(output.printDetail(customer, json));
}

export async function customersCreate(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Create a new customer (press Enter to skip optional fields)');
    const name = await requiredInput('Name:');
    const email = await optionalInput('Email (optional):');
    const external_id = await optionalInput('External ID (optional):');
    body = { name, ...(email ? { email } : {}), ...(external_id ? { external_id } : {}) };
  }

  const sp = createSpinner('Creating customer...');
  const customer = (await client.post('/v1/customers', body)) as Customer;
  sp.stop();
  output.success(`Customer created: ${customer.id ?? ''}`);
  process.stdout.write(output.printDetail(customer, false));
}

export async function customersDelete(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Deleting customer...');
  await client.deleteEmpty(`/v1/customers/${id}`);
  sp.stop();
  output.success(`Customer ${id} deleted.`);
}

export async function customersUsage(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching usage...');
  const usage = await client.get(`/v1/customers/${id}/usage`);
  sp.stop();
  process.stdout.write(output.printDetail(usage, json));
}

export async function customersEntitlements(
  globalOpts: GlobalOpts,
  id: string,
  json: boolean
): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching entitlements...');
  const ents = await client.get(`/v1/customers/${id}/entitlements`);
  sp.stop();
  process.stdout.write(output.printDetail(ents, json));
}
