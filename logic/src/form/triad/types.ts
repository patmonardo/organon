export type Command<T = any> = {
  kind: string;                 // e.g., "shape.instantiate"
  payload: T;
  meta?: Record<string, unknown>;
};

export type Event<T = any> = {
  kind: string;                 // e.g., "shape.instantiated"
  payload: T;
  meta?: Record<string, unknown>;
};

export interface EventBus {
  publish(event: Event): void;
  subscribe(kind: string, handler: (e: Event) => void): () => void;
}
