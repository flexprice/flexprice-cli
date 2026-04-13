import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { ListResponse, Meter } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, select } from './prompts.js';
import type { GlobalOpts } from './types.js';

const AGGREGATION_CHOICES = [
  { name: 'SUM — sum all values', value: 'SUM' },
  { name: 'COUNT — count events', value: 'COUNT' },
  { name: 'AVG — average of values', value: 'AVG' },
  { name: 'MAX — maximum value', value: 'MAX' },
  { name: 'MIN — minimum value', value: 'MIN' },
  { name: 'UNIQUE_COUNT — count distinct values', value: 'UNIQUE_COUNT' },
];

export async function metersList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching meters...');
  const resp = (await client.get('/v1/meters')) as ListResponse<Meter>;
  sp.stop();
  const rows = (resp.items ?? []).map((m) => ({
    ID: m.id ?? '',
    Name: m.name ?? '',
    'Event Name': m.event_name ?? '',
    Aggregation: m.aggregation ?? '',
    Status: m.status ? output.statusBadge(m.status) : '',
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function metersGet(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching meter...');
  const meter = (await client.get(`/v1/meters/${id}`)) as Meter;
  sp.stop();
  process.stdout.write(output.printDetail(meter, json));
}

export async function metersCreate(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Create a new usage meter');
    const name = await requiredInput('Meter name:');
    const event_name = await requiredInput('Event name (the event to track):');
    const aggregation = await select({
      message: 'Aggregation type:',
      choices: AGGREGATION_CHOICES,
    });
    body = { name, event_name, aggregation };
  }

  const sp = createSpinner('Creating meter...');
  const meter = (await client.post('/v1/meters', body)) as Meter;
  sp.stop();
  output.success(`Meter created: ${meter.id ?? ''}`);
  process.stdout.write(output.printDetail(meter, false));
}

export async function metersDelete(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Deleting meter...');
  await client.deleteEmpty(`/v1/meters/${id}`);
  sp.stop();
  output.success(`Meter ${id} deleted.`);
}
