import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { Entitlement, ListResponse } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, confirm, number } from './prompts.js';
import type { GlobalOpts } from './types.js';

export async function entitlementsList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching entitlements...');
  const resp = (await client.get('/v1/entitlements')) as ListResponse<Entitlement>;
  sp.stop();
  const rows = (resp.items ?? []).map((e) => ({
    ID: e.id ?? '',
    Plan: e.plan_id ?? '',
    Feature: e.feature_id ?? '',
    Type: e.feature_type ?? '',
    Enabled:
      e.is_enabled === undefined ? '' : e.is_enabled ? '✓' : '✗',
    'Usage Limit':
      e.usage_limit === undefined ? '∞' : String(Math.round(e.usage_limit)),
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function entitlementsGet(
  globalOpts: GlobalOpts,
  id: string,
  json: boolean
): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching entitlement...');
  const ent = (await client.get(`/v1/entitlements/${id}`)) as Entitlement;
  sp.stop();
  process.stdout.write(output.printDetail(ent, json));
}

export async function entitlementsCreate(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Create a new entitlement (press Enter to use defaults)');
    const plan_id = await requiredInput('Plan ID:');
    const feature_id = await requiredInput('Feature ID:');
    const is_enabled = await confirm({ message: 'Is enabled?', default: true });
    const usage_limit = await number({
      message: 'Usage limit (0 = unlimited, leave blank = no limit):',
      default: 0,
    });
    body = {
      plan_id,
      feature_id,
      is_enabled,
      ...(usage_limit !== undefined && usage_limit > 0 ? { usage_limit } : {}),
    };
  }

  const sp = createSpinner('Creating entitlement...');
  const ent = (await client.post('/v1/entitlements', body)) as Entitlement;
  sp.stop();
  output.success(`Entitlement created: ${ent.id ?? ''}`);
  process.stdout.write(output.printDetail(ent, false));
}

export async function entitlementsDelete(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Deleting entitlement...');
  await client.deleteEmpty(`/v1/entitlements/${id}`);
  sp.stop();
  output.success(`Entitlement ${id} deleted.`);
}
