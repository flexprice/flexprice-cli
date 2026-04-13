import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { ListResponse, Plan } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, optionalInput } from './prompts.js';
import type { GlobalOpts } from './types.js';

export async function plansList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching plans...');
  const resp = (await client.get('/v1/plans')) as ListResponse<Plan>;
  sp.stop();
  const rows = (resp.items ?? []).map((p) => ({
    ID: p.id ?? '',
    Name: p.name ?? '',
    Description: p.description ?? '',
    Status: p.status ? output.statusBadge(p.status) : '',
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function plansGet(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching plan...');
  const plan = (await client.get(`/v1/plans/${id}`)) as Plan;
  sp.stop();
  process.stdout.write(output.printDetail(plan, json));
}

export async function plansCreate(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Create a new pricing plan (press Enter to skip optional fields)');
    const name = await requiredInput('Plan name:');
    const description = await optionalInput('Description (optional):');
    body = { name, ...(description ? { description } : {}) };
  }

  const sp = createSpinner('Creating plan...');
  const plan = (await client.post('/v1/plans', body)) as Plan;
  sp.stop();
  output.success(`Plan created: ${plan.id ?? ''}`);
  process.stdout.write(output.printDetail(plan, false));
}

export async function plansDelete(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Deleting plan...');
  await client.deleteEmpty(`/v1/plans/${id}`);
  sp.stop();
  output.success(`Plan ${id} deleted.`);
}
