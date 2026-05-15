import zhCN from './zh-CN.json';

type Messages = typeof zhCN;

function get<T extends object>(obj: T, path: string): string {
  const parts = path.split('.');
  let current: unknown = obj;
  for (const part of parts) {
    if (current == null || typeof current !== 'object') return path;
    current = (current as Record<string, unknown>)[part];
  }
  return typeof current === 'string' ? current : path;
}

export function t(key: string): string {
  return get(zhCN, key);
}

export default zhCN;
