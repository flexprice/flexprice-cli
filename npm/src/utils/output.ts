import chalk from 'chalk';
import Table from 'cli-table3';

export function printTable(rows: Record<string, string>[], outputJson: boolean): string {
  if (outputJson) {
    return `${JSON.stringify(rows, null, 2)}\n`;
  }
  if (rows.length === 0) {
    return `  ${chalk.dim('No results found.')}\n`;
  }
  const cols = Object.keys(rows[0] ?? {});
  const table = new Table({
    head: cols.map((c) => chalk.bold.cyan(c)),
    style: { head: [] },
  });
  for (const row of rows) {
    table.push(cols.map((c) => row[c] ?? ''));
  }
  return `${table.toString()}\n`;
}

export function printDetail(item: unknown, outputJson: boolean): string {
  if (outputJson) {
    return `${JSON.stringify(item, null, 2)}\n`;
  }
  const json = JSON.stringify(item, null, 2);
  return colorizeJson(json);
}

function colorizeJson(json: string): string {
  let result = '';
  for (const line of json.split('\n')) {
    if (line.includes(':')) {
      const idx = line.indexOf(':');
      const key = line.slice(0, idx);
      const rest = line.slice(idx + 1);
      result += `${chalk.cyan(key)}:${rest}\n`;
    } else {
      result += `${chalk.dim(line)}\n`;
    }
  }
  return result;
}

export function success(msg: string): void {
  console.log(`  ${chalk.green.bold('✓')} ${msg}`);
}

export function error(msg: string): void {
  console.error(`  ${chalk.red.bold('✗')} ${msg}`);
}

export function warning(msg: string): void {
  console.error(`  ${chalk.yellow.bold('⚠')} ${msg}`);
}

export function info(msg: string): void {
  console.log(`  ${chalk.blue.bold('ℹ')} ${msg}`);
}

export function statusBadge(status: string): string {
  const s = status.toLowerCase();
  if (['active', 'published', 'paid', 'finalized'].includes(s)) {
    return chalk.green.bold(status);
  }
  if (['draft', 'pending'].includes(s)) {
    return chalk.yellow(status);
  }
  if (['cancelled', 'canceled', 'void', 'voided', 'inactive'].includes(s)) {
    return chalk.red(status);
  }
  if (['trialing', 'paused'].includes(s)) {
    return chalk.blue(status);
  }
  return status;
}
