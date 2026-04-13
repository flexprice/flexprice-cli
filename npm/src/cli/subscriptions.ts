import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { ListResponse, Subscription } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, optionalInput } from './prompts.js';
import type { GlobalOpts } from './types.js';

export async function subscriptionsList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching subscriptions...');
  const resp = (await client.get('/v1/subscriptions')) as ListResponse<Subscription>;
  sp.stop();
  const rows = (resp.items ?? []).map((s) => ({
    ID: s.id ?? '',
    Customer: s.customer_id ?? '',
    Plan: s.plan_id ?? '',
    Status: s.subscription_status ? output.statusBadge(s.subscription_status) : '',
    'Period Start': s.current_period_start ?? '',
    'Period End': s.current_period_end ?? '',
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function subscriptionsGet(
  globalOpts: GlobalOpts,
  id: string,
  json: boolean
): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching subscription...');
  const sub = (await client.get(`/v1/subscriptions/${id}`)) as Subscription;
  sp.stop();
  process.stdout.write(output.printDetail(sub, json));
}

export async function subscriptionsCreate(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Create a new subscription (press Enter to skip optional fields)');
    const customer_id = await requiredInput('Customer ID:');
    const plan_id = await requiredInput('Plan ID:');
    const start_date = await optionalInput('Start date (ISO 8601, optional — default: now):');
    body = {
      customer_id,
      plan_id,
      ...(start_date ? { start_date } : {}),
    };
  }

  const sp = createSpinner('Creating subscription...');
  const sub = (await client.post('/v1/subscriptions', body)) as Subscription;
  sp.stop();
  output.success(`Subscription created: ${sub.id ?? ''}`);
  process.stdout.write(output.printDetail(sub, false));
}

export async function subscriptionsCancel(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Cancelling subscription...');
  const sub = await client.postEmpty(`/v1/subscriptions/${id}/cancel`);
  sp.stop();
  output.success(`Subscription ${id} cancelled.`);
  process.stdout.write(output.printDetail(sub, false));
}

/** Rust always passes `false` for JSON on usage output. */
export async function subscriptionsUsage(globalOpts: GlobalOpts, file: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const data = fs.readFileSync(file, 'utf-8');
  const body = JSON.parse(data) as unknown;
  const sp = createSpinner('Fetching usage...');
  const usage = await client.post('/v1/subscriptions/usage', body);
  sp.stop();
  process.stdout.write(output.printDetail(usage, false));
}
