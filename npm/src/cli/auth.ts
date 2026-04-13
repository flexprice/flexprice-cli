import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { Credentials } from '../config/store.js';
import {
  credentialsPath,
  DEFAULT_API_URL,
  deleteCredentials,
  loadCredentials,
  maskedApiKey,
  saveCredentials,
} from '../config/store.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';

export function requireAuth(
  globalOpts: { apiUrl?: string; apiKey?: string }
): Credentials {
  const creds = loadCredentials(globalOpts.apiUrl, globalOpts.apiKey);
  if (!creds.api_key) {
    output.warning(
      'Not authenticated. Run `flexprice auth set-api-key <KEY>` or set FLEXPRICE_API_KEY.'
    );
    process.exit(1);
  }
  return creds;
}

export async function setApiKey(key: string, apiUrl: string): Promise<void> {
  const sp = createSpinner('Validating API key...');
  const creds: Credentials = {
    api_url: apiUrl,
    api_key: key,
    environment_id: undefined,
  };
  const client = new ApiClient(creds);
  await client.healthCheck();
  sp.stop();
  saveCredentials(creds);
  output.success('API key validated and saved!');
  output.success(`API URL: ${apiUrl}`);
  output.success(`Credentials saved to ${credentialsPath()}`);
}

export async function whoami(globalOpts: { apiUrl?: string; apiKey?: string }): Promise<void> {
  const creds = requireAuth(globalOpts);
  const sp = createSpinner('Fetching user info...');
  const client = new ApiClient(creds);
  const userInfo = (await client.get('/v1/users/me')) as Record<string, unknown>;
  sp.stop();

  console.log();
  output.info(`API URL:    ${creds.api_url}`);
  if (creds.environment_id) {
    output.info(`Env ID:     ${creds.environment_id}`);
  }
  output.info('Auth:       API Key');
  console.log();

  process.stdout.write(output.printDetail(userInfo, false));
}

export async function status(): Promise<void> {
  try {
    const creds = loadFromFileOnly();
    output.success('Credentials found');
    output.info(`API URL:    ${creds.api_url}`);
    output.info(`API Key:    ${maskedApiKey(creds)}`);
    if (creds.environment_id) {
      output.info(`Env ID:     ${creds.environment_id}`);
    }
    const sp = createSpinner('Testing connection...');
    const client = new ApiClient(creds);
    try {
      await client.healthCheck();
      sp.stop();
      output.success('API connection OK');
    } catch (e) {
      sp.stop();
      const msg = e instanceof Error ? e.message : String(e);
      output.warning(`API unreachable: ${msg}`);
    }
  } catch {
    output.warning('Not authenticated.');
    output.info(
      'Run `flexprice auth set-api-key <KEY>` or set FLEXPRICE_API_KEY (see `--help`).'
    );
  }
}

function loadFromFileOnly(): Credentials {
  const p = credentialsPath();
  if (!fs.existsSync(p)) {
    throw new Error('missing');
  }
  return JSON.parse(fs.readFileSync(p, 'utf-8')) as Credentials;
}

export function logout(): void {
  deleteCredentials();
  output.success('Credentials removed. You are now logged out.');
}

export { DEFAULT_API_URL };
