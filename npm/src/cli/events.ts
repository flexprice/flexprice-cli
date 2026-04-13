import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import { requiredInput, optionalInput } from './prompts.js';
import type { GlobalOpts } from './types.js';

export async function eventsIngest(globalOpts: GlobalOpts, file?: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);

  let body: unknown;
  if (file) {
    body = JSON.parse(fs.readFileSync(file, 'utf-8'));
  } else {
    output.info('Ingest a usage event (press Enter to skip optional fields)');
    const event_name = await requiredInput('Event name:');
    const external_customer_id = await requiredInput('External customer ID:');
    const timestamp = await optionalInput(
      `Timestamp (ISO 8601, optional — default: now):`,
      new Date().toISOString()
    );
    const propertiesRaw = await optionalInput(
      'Properties (JSON string, optional — e.g. {"quantity":1}):'
    );
    let properties: unknown = undefined;
    if (propertiesRaw) {
      try {
        properties = JSON.parse(propertiesRaw) as unknown;
      } catch {
        output.warning('Could not parse properties as JSON — sending as raw string.');
        properties = propertiesRaw;
      }
    }
    body = {
      event_name,
      external_customer_id,
      timestamp: timestamp ?? new Date().toISOString(),
      ...(properties !== undefined ? { properties } : {}),
    };
  }

  const sp = createSpinner('Ingesting event...');
  const resp = await client.post('/v1/events', body);
  sp.stop();
  output.success('Event ingested successfully!');
  process.stdout.write(output.printDetail(resp, false));
}

export async function eventsIngestBulk(globalOpts: GlobalOpts, file: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const data = fs.readFileSync(file, 'utf-8');
  const body = JSON.parse(data) as unknown;
  const sp = createSpinner('Ingesting events in bulk...');
  const resp = await client.post('/v1/events/bulk', body);
  sp.stop();
  output.success('Bulk events ingested successfully!');
  process.stdout.write(output.printDetail(resp, false));
}

export async function eventsList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching events...');
  const resp = await client.get('/v1/events');
  sp.stop();
  process.stdout.write(output.printDetail(resp, json));
}

export async function eventsGet(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching event...');
  const evt = await client.get(`/v1/events/${id}`);
  sp.stop();
  process.stdout.write(output.printDetail(evt, json));
}

export async function eventsUsage(globalOpts: GlobalOpts, file: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const data = fs.readFileSync(file, 'utf-8');
  const body = JSON.parse(data) as unknown;
  const sp = createSpinner('Fetching usage...');
  const usage = await client.post('/v1/events/usage', body);
  sp.stop();
  process.stdout.write(output.printDetail(usage, false));
}
