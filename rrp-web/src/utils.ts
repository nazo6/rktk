import { Result } from "./bindings";

export function unwrapped<A extends unknown[], T, E extends string>(
  fn: (...args: A) => Promise<Result<T, E>>,
) {
  return async (...args: A) => {
    const res = await fn(...args);
    if (res.status === "ok") return res.data;
    else throw Error(res.error);
  };
}
