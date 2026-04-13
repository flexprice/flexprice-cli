import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';

/** Default FlexPrice API base URL when none is configured. */
export const DEFAULT_API_URL = 'https://api.cloud.flexprice.io';

export interface Credentials {
  api_url: string;
  api_key?: string;
  environment_id?: string;
}

function defaultCredentials(): Credentials {
  return {
    api_url: DEFAULT_API_URL,
    api_key: undefined,
    environment_id: undefined,
  };
}

export function credentialsPath(): string {
  return path.join(os.homedir(), '.flexprice', 'credentials.json');
}

function loadFromFile(): Credentials {
  const filePath = credentialsPath();
  if (!fs.existsSync(filePath)) {
    throw new Error('No credentials file found');
  }
  const content = fs.readFileSync(filePath, 'utf-8');
  return JSON.parse(content) as Credentials;
}

/** Load credentials: file defaults → env → CLI flags (matches Rust `Credentials::load`). */
export function loadCredentials(cliApiUrl?: string, cliApiKey?: string): Credentials {
  let creds: Credentials;
  try {
    creds = loadFromFile();
  } catch {
    creds = defaultCredentials();
  }

  const envUrl = process.env.FLEXPRICE_API_URL;
  if (envUrl && envUrl.length > 0) {
    creds.api_url = envUrl;
  }
  const envKey = process.env.FLEXPRICE_API_KEY;
  if (envKey && envKey.length > 0) {
    creds.api_key = envKey;
  }
  const envId = process.env.FLEXPRICE_ENVIRONMENT_ID;
  if (envId && envId.length > 0) {
    creds.environment_id = envId;
  }

  if (cliApiUrl !== undefined && cliApiUrl !== '') {
    creds.api_url = cliApiUrl;
  }
  if (cliApiKey !== undefined && cliApiKey !== '') {
    creds.api_key = cliApiKey;
  }

  return creds;
}

export function saveCredentials(creds: Credentials): void {
  const filePath = credentialsPath();
  const parent = path.dirname(filePath);
  fs.mkdirSync(parent, { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(creds, null, 2)}\n`, 'utf-8');
}

export function deleteCredentials(): void {
  const filePath = credentialsPath();
  if (fs.existsSync(filePath)) {
    fs.unlinkSync(filePath);
  }
}

export function isAuthenticated(creds: Credentials): boolean {
  return creds.api_key !== undefined && creds.api_key !== '';
}

export function getAuthHeader(creds: Credentials): { name: string; value: string } | undefined {
  if (!creds.api_key) return undefined;
  return { name: 'x-api-key', value: creds.api_key };
}

export function maskedApiKey(creds: Credentials): string {
  const key = creds.api_key;
  if (!key) return '(not set)';
  if (key.length > 8) {
    return `${key.slice(0, 4)}...${key.slice(-4)}`;
  }
  return '*'.repeat(key.length);
}
