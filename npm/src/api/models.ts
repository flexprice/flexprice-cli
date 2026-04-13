// Generic list wrapper

export interface ListResponse<T> {
  items?: T[];
  total_count?: number;
}

export interface Customer {
  id?: string;
  name?: string;
  email?: string;
  external_id?: string;
  status?: string;
  created_at?: string;
}

export interface Plan {
  id?: string;
  name?: string;
  description?: string;
  status?: string;
  created_at?: string;
}

export interface Subscription {
  id?: string;
  customer_id?: string;
  plan_id?: string;
  subscription_status?: string;
  current_period_start?: string;
  current_period_end?: string;
  created_at?: string;
}

export interface Invoice {
  id?: string;
  customer_id?: string;
  subscription_id?: string;
  invoice_status?: string;
  payment_status?: string;
  amount_due?: number;
  currency?: string;
  created_at?: string;
}

export interface Meter {
  id?: string;
  name?: string;
  event_name?: string;
  aggregation?: string;
  status?: string;
  created_at?: string;
}

export interface Event {
  id?: string;
  event_name?: string;
  external_customer_id?: string;
  timestamp?: string;
  properties?: unknown;
}

export interface Wallet {
  id?: string;
  customer_id?: string;
  balance?: number;
  currency?: string;
  wallet_status?: string;
  created_at?: string;
}

export interface WalletBalance {
  balance?: number;
  real_time_balance?: number;
  currency?: string;
}

export interface Feature {
  id?: string;
  name?: string;
  lookup_key?: string;
  /** API JSON field `type` */
  type?: string;
  status?: string;
  created_at?: string;
}

export interface Entitlement {
  id?: string;
  plan_id?: string;
  feature_id?: string;
  feature_type?: string;
  is_enabled?: boolean;
  usage_limit?: number;
  created_at?: string;
}

export type JsonValue = Record<string, unknown> | unknown[] | string | number | boolean | null;
