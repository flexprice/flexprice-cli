import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { Feature, ListResponse } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, select, toSnakeCase } from './prompts.js';
import type { GlobalOpts } from './types.js';

const FEATURE_TYPE_CHOICES = [
  { name: 'BOOLEAN — on/off feature flag', value: 'BOOLEAN' },
  { name: 'METERED — usage-based feature', value: 'METERED' },
  { name: 'STATIC — static value feature', value: 'STATIC' },
];

export async function featuresList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching features...');
  const resp = (await client.get('/v1/features')) as ListResponse<Feature>;
  sp.stop();
  const rows = (resp.items ?? []).map((f) => ({
    ID: f.id ?? '',
    Name: f.name ?? '',
    'Lookup Key': f.lookup_key ?? '',
    Type: f.type ?? '',
    Status: f.status ? output.statusBadge(f.status) : '',
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function featuresGet(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching feature...');
  const feature = (await client.get(`/v1/features/${id}`)) as Feature;
  sp.stop();
  process.stdout.write(output.printDetail(feature, json));
}

export async function featuresCreate(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Create a new feature');
    const name = await requiredInput('Feature name:');
    const lookup_key = await requiredInput(`Lookup key (suggested: ${toSnakeCase(name)}):`);
    const type = await select({
      message: 'Feature type:',
      choices: FEATURE_TYPE_CHOICES,
    });
    body = { name, lookup_key: lookup_key || toSnakeCase(name), type };
  }

  const sp = createSpinner('Creating feature...');
  const feature = (await client.post('/v1/features', body)) as Feature;
  sp.stop();
  output.success(`Feature created: ${feature.id ?? ''}`);
  process.stdout.write(output.printDetail(feature, false));
}

export async function featuresDelete(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Deleting feature...');
  await client.deleteEmpty(`/v1/features/${id}`);
  sp.stop();
  output.success(`Feature ${id} deleted.`);
}
