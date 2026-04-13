import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Box, Text, useApp, useInput } from 'ink';
import { ApiClient } from '../api/client.js';
import type { Credentials } from '../config/store.js';
import { Theme } from './theme.js';

const TABS = [
  'Customers',
  'Plans',
  'Subscriptions',
  'Invoices',
  'Meters',
  'Wallets',
  'Features',
] as const;

const TAB_ENDPOINTS = [
  '/v1/customers',
  '/v1/plans',
  '/v1/subscriptions',
  '/v1/invoices',
  '/v1/meters',
  '/v1/wallets',
  '/v1/features',
] as const;

const MAX_SPARKLINE = 15;

function pickStatus(item: Record<string, unknown>): string {
  const s =
    item.status ??
    item.subscription_status ??
    item.invoice_status ??
    item.wallet_status;
  return typeof s === 'string' ? s : '';
}

function truncateId(id: string): string {
  return id.length > 10 ? `${id.slice(0, 8)}…` : id;
}

function formatListItem(item: Record<string, unknown>): string {
  const id = typeof item.id === 'string' ? truncateId(item.id) : '?';
  const nameVal =
    item.name ?? item.email ?? item.event_name ?? item.lookup_key ?? '-';
  const name = typeof nameVal === 'string' ? nameVal : '-';
  const status = pickStatus(item);
  return status === '' ? `${id}  ${name}` : `${id}  ${name}  [${status}]`;
}

function buildSparkline(counts: number[]): string {
  if (counts.length === 0) return '▁'.repeat(MAX_SPARKLINE);
  const max = Math.max(...counts, 1);
  return counts.map((n) => {
    const h = Math.round((n / max) * 7);
    return '▁▂▃▄▅▆▇█'[h] ?? '▁';
  }).join('');
}

export function DashboardApp({ creds }: { creds: Credentials }) {
  const { exit } = useApp();
  const client = useMemo(() => new ApiClient(creds), [creds]);
  const [activeTab, setActiveTab] = useState(0);
  const [listItems, setListItems] = useState<string[]>([]);
  const [itemsArray, setItemsArray] = useState<Record<string, unknown>[]>([]);
  const [detailText, setDetailText] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | undefined>();
  const [selected, setSelected] = useState(0);
  // Per-tab sparkline history: array of total_counts per load
  const [sparkHistory, setSparkHistory] = useState<number[][]>(
    Array.from({ length: TABS.length }, () => [])
  );
  const itemsLenRef = useRef(0);

  useEffect(() => {
    itemsLenRef.current = itemsArray.length;
  }, [itemsArray.length]);

  useEffect(() => {
    if (itemsArray.length === 0) return;
    const row = itemsArray[selected];
    setDetailText(row ? `${JSON.stringify(row, null, 2)}\n` : '');
  }, [selected, itemsArray]);

  const loadData = useCallback(async () => {
    setLoading(true);
    setError(undefined);
    setDetailText('');
    const endpoint = TAB_ENDPOINTS[activeTab];
    try {
      const body = await client.getText(endpoint);
      let parsed: unknown;
      try {
        parsed = JSON.parse(body) as unknown;
      } catch {
        setListItems(['(raw response)']);
        setItemsArray([]);
        setDetailText(`${body}\n`);
        setSelected(0);
        setLoading(false);
        return;
      }
      const obj = parsed as Record<string, unknown>;
      const itemsRaw = obj.items;
      const totalCount = typeof obj.total_count === 'number' ? obj.total_count : 0;

      // Update sparkline history for current tab
      setSparkHistory((prev) => {
        const updated = prev.map((tabHistory, i) => {
          if (i !== activeTab) return tabHistory;
          const next = [...tabHistory, totalCount];
          return next.length > MAX_SPARKLINE ? next.slice(next.length - MAX_SPARKLINE) : next;
        });
        return updated;
      });

      if (Array.isArray(itemsRaw)) {
        const rows = itemsRaw.filter(
          (x): x is Record<string, unknown> =>
            !!x && typeof x === 'object' && !Array.isArray(x)
        );
        if (rows.length === 0) {
          setListItems(['(no items)']);
          setItemsArray([]);
        } else {
          setListItems(rows.map((item) => formatListItem(item)));
          setItemsArray(rows);
        }
      } else {
        setListItems(['(no items)']);
        setItemsArray([]);
      }
      setSelected(0);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      setListItems([]);
      setItemsArray([]);
      setDetailText('');
      setSelected(0);
    }
    setLoading(false);
  }, [activeTab, client]);

  useEffect(() => {
    void loadData();
  }, [loadData]);

  useInput((input, key) => {
    if (input === 'q' || key.escape) {
      exit();
      return;
    }
    if (input === 'r') {
      void loadData();
      return;
    }
    if (key.tab || input === 'l') {
      setActiveTab((t) => (t + 1) % TABS.length);
      setSelected(0);
      return;
    }
    if ((key.shift && key.tab) || input === 'h') {
      setActiveTab((t) => (t === 0 ? TABS.length - 1 : t - 1));
      setSelected(0);
      return;
    }
    if (key.downArrow || input === 'j') {
      const len = itemsLenRef.current;
      if (len === 0) return;
      setSelected((i) => (i + 1) % len);
      return;
    }
    if (key.upArrow || input === 'k') {
      const len = itemsLenRef.current;
      if (len === 0) return;
      setSelected((i) => (i === 0 ? len - 1 : i - 1));
    }
  });

  const currentSparkData = sparkHistory[activeTab] ?? [];
  const sparkBars = buildSparkline(currentSparkData);

  const visibleCount = loading || error ? 0 : listItems.length;

  return (
    <Box flexDirection="column" width="100%" height="100%">
      <Box flexDirection="row" borderStyle="single" borderColor={Theme.BORDER} paddingX={1}>
        <Box flexGrow={1} flexDirection="column">
          <Text>
            <Text color={Theme.WARNING}>⚡ </Text>
            <Text bold color={Theme.PRIMARY}>
              FlexPrice
            </Text>
            <Text color={Theme.TEXT_DIM}> Dashboard</Text>
          </Text>
          <Text color={Theme.TEXT_MUTED}>     Usage-based billing, visualized.</Text>
        </Box>
        <Box
          flexDirection="column"
          borderLeft={true}
          borderStyle="single"
          borderColor={Theme.BORDER}
          paddingLeft={1}
        >
          <Text>
            <Text color={Theme.TEXT_DIM}> API: </Text>
            <Text color={Theme.ACCENT}>{creds.api_url}</Text>
          </Text>
          <Text>
            <Text color={Theme.TEXT_DIM}> Auth: </Text>
            <Text color={Theme.INFO}>API Key</Text>
          </Text>
        </Box>
      </Box>

      <Box flexGrow={1} flexDirection="row">
        <Box
          width={22}
          flexDirection="column"
          borderStyle="single"
          borderRight={true}
          borderColor={Theme.BORDER}
          paddingX={1}
        >
          <Text dimColor color={Theme.TEXT_DIM}>
            {' Navigate'}
          </Text>
          {TABS.map((name, i) => (
            <Text
              key={name}
              bold={i === activeTab}
              color={i === activeTab ? Theme.PRIMARY : Theme.TEXT_DIM}
            >
              {i === activeTab ? ' ▸ ' : '   '}
              {name}
            </Text>
          ))}
        </Box>

        <Box
          flexGrow={1}
          flexDirection="column"
          borderStyle="single"
          borderRight={true}
          borderColor={Theme.BORDER}
          paddingX={1}
        >
          <Text bold color={Theme.PRIMARY}>
            {' '}
            {TABS[activeTab]} {!loading && !error ? `(${visibleCount})` : ''}
          </Text>
          {loading ? (
            <Text color={Theme.WARNING}> ⏳ Loading...</Text>
          ) : error ? (
            <Text color={Theme.ERROR}> ✗ {error}</Text>
          ) : (
            listItems.map((line, i) => (
              <Text key={`${line}-${i}`} color={Theme.TEXT} inverse={i === selected}>
                {' '}
                {line}
              </Text>
            ))
          )}
        </Box>

        <Box flexGrow={1} flexDirection="column" paddingLeft={1}>
          <Box flexDirection="column" flexGrow={1} borderStyle="single" borderColor={Theme.BORDER} paddingX={1}>
            <Text bold color={Theme.ACCENT}>
              {' Detail '}
            </Text>
            <Text color={Theme.TEXT_DIM}>{detailText || ' '}</Text>
          </Box>
          <Box flexDirection="column" marginTop={1} borderStyle="single" borderColor={Theme.BORDER} paddingX={1}>
            <Text color={Theme.INFO}> Activity ({TABS[activeTab]})</Text>
            <Text color={Theme.ACCENT}>{sparkBars}</Text>
            {currentSparkData.length > 0 && (
              <Text color={Theme.TEXT_MUTED}>
                {' '}last count: {currentSparkData[currentSparkData.length - 1]}
              </Text>
            )}
          </Box>
        </Box>
      </Box>

      <Box borderStyle="single" borderTop={true} borderColor={Theme.BORDER} paddingX={1}>
        <Text>
          <Text color={Theme.PRIMARY}> Tab/h/l Switch </Text>
          <Text color={Theme.BORDER}> │ </Text>
          <Text color={Theme.TEXT_DIM}>↑/↓/j/k Navigate </Text>
          <Text color={Theme.BORDER}> │ </Text>
          <Text color={Theme.ACCENT}>r Refresh </Text>
          <Text color={Theme.BORDER}> │ </Text>
          <Text color={Theme.ERROR}>q Quit</Text>
        </Text>
      </Box>
    </Box>
  );
}

export async function runDashboard(creds: Credentials): Promise<void> {
  const { render } = await import('ink');
  const instance = render(<DashboardApp creds={creds} />, {
    exitOnCtrlC: true,
    patchConsole: true,
  });
  await instance.waitUntilExit();
}
