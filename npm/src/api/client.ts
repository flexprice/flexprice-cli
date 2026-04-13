import type { Credentials } from '../config/store.js';
import { DEFAULT_API_URL } from '../config/store.js';

const REQUEST_TIMEOUT_MS = 30_000;

interface ApiErrorBody {
  error?: string;
  message?: string;
  hint?: string;
}

export class ApiError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'ApiError';
  }
}

export class ApiClient {
  private readonly baseUrl: string;
  private readonly credentials: Credentials;

  constructor(credentials: Credentials) {
    this.credentials = credentials;
    const url =
      !credentials.api_url || credentials.api_url.trim() === ''
        ? DEFAULT_API_URL
        : credentials.api_url.trim().replace(/\/$/, '');
    this.baseUrl = url;
  }

  private url(path: string): string {
    return `${this.baseUrl}${path}`;
  }

  private applyHeaders(init: RequestInit): Headers {
    const headers = new Headers(init.headers);
    if (this.credentials.api_key) {
      headers.set('x-api-key', this.credentials.api_key);
    }
    if (this.credentials.environment_id) {
      headers.set('x-environment-id', this.credentials.environment_id);
    }
    if (init.body && !headers.has('Content-Type')) {
      headers.set('Content-Type', 'application/json');
    }
    return headers;
  }

  private async handleResponse<T>(response: Response): Promise<T> {
    const status = response.status;
    if (response.ok) {
      const body = (await response.json()) as T;
      return body;
    }

    const bodyText = await response.text();
    try {
      const apiErr = JSON.parse(bodyText) as ApiErrorBody;
      const msg = apiErr.error ?? apiErr.message ?? 'Unknown error';
      const reason = response.statusText || '';
      const hint = apiErr.hint;
      const errMsg = hint
        ? `${status} (${reason}): ${msg} — ${hint}`
        : `${status} (${reason}): ${msg}`;
      throw new ApiError(errMsg);
    } catch (e) {
      if (e instanceof ApiError) throw e;
      if (status === 401) {
        throw new ApiError(
          'Authentication failed. Run `flexprice auth set-api-key <KEY>` or check FLEXPRICE_API_KEY.'
        );
      }
      if (status === 403) {
        throw new ApiError(
          'Permission denied. Your credentials may not have access to this resource.'
        );
      }
      if (status === 404) {
        throw new ApiError('Resource not found. Verify the ID is correct.');
      }
      throw new ApiError(`${status}: ${bodyText}`);
    }
  }

  private async handleResponseText(response: Response): Promise<string> {
    if (response.ok) {
      return response.text();
    }
    const body = await response.text();
    throw new ApiError(`${response.status}: ${body}`);
  }

  async get<T>(path: string): Promise<T> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'GET',
        headers: this.applyHeaders({}),
        signal: controller.signal,
      });
      return this.handleResponse<T>(response);
    } catch (e) {
      if (e instanceof Error && e.name === 'AbortError') {
        throw new ApiError('Request failed: timeout');
      }
      throw e;
    } finally {
      clearTimeout(t);
    }
  }

  async getText(path: string): Promise<string> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'GET',
        headers: this.applyHeaders({}),
        signal: controller.signal,
      });
      return this.handleResponseText(response);
    } finally {
      clearTimeout(t);
    }
  }

  /** Binary-safe GET (e.g. invoice PDF). */
  async getBuffer(path: string): Promise<Uint8Array> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'GET',
        headers: this.applyHeaders({}),
        signal: controller.signal,
      });
      if (!response.ok) {
        const body = await response.text();
        throw new ApiError(`${response.status}: ${body}`);
      }
      const buf = await response.arrayBuffer();
      return new Uint8Array(buf);
    } finally {
      clearTimeout(t);
    }
  }

  async post<B>(path: string, body: B): Promise<unknown> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'POST',
        headers: this.applyHeaders({ body: JSON.stringify(body) }),
        body: JSON.stringify(body),
        signal: controller.signal,
      });
      return this.handleResponse<unknown>(response);
    } finally {
      clearTimeout(t);
    }
  }

  async postEmpty(path: string): Promise<unknown> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'POST',
        headers: this.applyHeaders({}),
        signal: controller.signal,
      });
      return this.handleResponse<unknown>(response);
    } finally {
      clearTimeout(t);
    }
  }

  async put<B>(path: string, body: B): Promise<unknown> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'PUT',
        headers: this.applyHeaders({ body: JSON.stringify(body) }),
        body: JSON.stringify(body),
        signal: controller.signal,
      });
      return this.handleResponse<unknown>(response);
    } finally {
      clearTimeout(t);
    }
  }

  async delete(path: string): Promise<unknown> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'DELETE',
        headers: this.applyHeaders({}),
        signal: controller.signal,
      });
      return this.handleResponse<unknown>(response);
    } finally {
      clearTimeout(t);
    }
  }

  async deleteEmpty(path: string): Promise<void> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url(path), {
        method: 'DELETE',
        headers: this.applyHeaders({}),
        signal: controller.signal,
      });
      if (response.ok) return;
      const body = await response.text();
      throw new ApiError(`${response.status}: ${body}`);
    } finally {
      clearTimeout(t);
    }
  }

  async healthCheck(): Promise<void> {
    const controller = new AbortController();
    const t = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(this.url('/health'), {
        method: 'GET',
        headers: this.applyHeaders({}),
        signal: controller.signal,
      });
      if (!response.ok) {
        throw new ApiError(`API returned status ${response.status}`);
      }
    } catch (e) {
      if (e instanceof ApiError) throw e;
      if (e instanceof Error && e.name === 'AbortError') {
        throw new ApiError('Cannot reach FlexPrice API (timeout)');
      }
      throw new ApiError('Cannot reach FlexPrice API');
    } finally {
      clearTimeout(t);
    }
  }
}
