import type { Repository, Concurrency } from "../../src/repository/repo";
import { getId } from "../../src/repository/repo";

export function makeInMemoryRepository<T>(): Repository<T> {
  const store = new Map<string, T>();
  return {
    async create(doc: T): Promise<T> {
      const id = getId(doc as any);
      store.set(id, doc);
      return doc;
    },
    async get(id: string): Promise<T | undefined> {
      return store.get(id);
    },
    async update(id: string, mutate: () => T, concurrency?: Concurrency): Promise<T> {
      const current = store.get(id);
      if (!current) throw new Error(`not found: ${id}`);
      if (concurrency && concurrency.expectedRevision !== undefined) {
        const rev = (current as any).revision ?? 0;
        if (rev !== concurrency.expectedRevision) {
          throw new Error(`revision conflict: expected ${concurrency.expectedRevision}, got ${rev}`);
        }
      }
      const next = mutate();
      store.set(id, next);
      return next;
    },
    async delete(id: string): Promise<boolean> {
      return store.delete(id);
    },
    async find() {
      return Array.from(store.values());
    },
  } as any;
}
