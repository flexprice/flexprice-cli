import * as fs from 'node:fs';
import { ApiClient } from '../api/client.js';
import type { Invoice, ListResponse } from '../api/models.js';
import * as output from '../utils/output.js';
import { createSpinner } from '../utils/spinner.js';
import { requireAuth } from './auth.js';
import type { GlobalOpts } from './types.js';

export async function invoicesList(globalOpts: GlobalOpts, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching invoices...');
  const resp = (await client.get('/v1/invoices')) as ListResponse<Invoice>;
  sp.stop();
  const rows = (resp.items ?? []).map((i) => ({
    ID: i.id ?? '',
    Customer: i.customer_id ?? '',
    Status: i.invoice_status ? output.statusBadge(i.invoice_status) : '',
    Payment: i.payment_status ? output.statusBadge(i.payment_status) : '',
    Amount: i.amount_due !== undefined ? i.amount_due.toFixed(2) : '',
    Currency: i.currency ?? '',
  }));
  process.stdout.write(output.printTable(rows, json));
}

export async function invoicesGet(globalOpts: GlobalOpts, id: string, json: boolean): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Fetching invoice...');
  const inv = (await client.get(`/v1/invoices/${id}`)) as Invoice;
  sp.stop();
  process.stdout.write(output.printDetail(inv, json));
}

export async function invoicesFinalize(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Finalizing invoice...');
  const inv = await client.postEmpty(`/v1/invoices/${id}/finalize`);
  sp.stop();
  output.success(`Invoice ${id} finalized.`);
  process.stdout.write(output.printDetail(inv, false));
}

export async function invoicesVoid(globalOpts: GlobalOpts, id: string): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Voiding invoice...');
  const inv = await client.postEmpty(`/v1/invoices/${id}/void`);
  sp.stop();
  output.success(`Invoice ${id} voided.`);
  process.stdout.write(output.printDetail(inv, false));
}

export async function invoicesPdf(
  globalOpts: GlobalOpts,
  id: string,
  outPath: string
): Promise<void> {
  const creds = requireAuth(globalOpts);
  const client = new ApiClient(creds);
  const sp = createSpinner('Downloading PDF...');
  const pdfContent = await client.getBuffer(`/v1/invoices/${id}/pdf`);
  fs.writeFileSync(outPath, pdfContent);
  sp.stop();
  output.success(`Invoice PDF saved to ${outPath}`);
}
