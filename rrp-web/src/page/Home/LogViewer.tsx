import { Connection } from "@/lib/connection";
import { useInfiniteQuery } from "@tanstack/react-query";
import { useEffect } from "react";

export function LogViewerPage(props: { connection: Connection }) {
  const { data, error, fetchNextPage } = useInfiniteQuery({
    queryKey: ["getLog"],
    queryFn: async () => {
      const data = await props.connection.client.get_log();
      const now = new Date().toISOString();
      return data.map((log) => ({ time: now, log }));
    },
    initialPageParam: 0,
    getNextPageParam: (_l, _a, pp) => pp + 1,
  });

  useEffect(() => {
    const interval = setInterval(() => {
      fetchNextPage();
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const logs = data?.pages.flat().reverse();

  return (
    <div className="px-2">
      {logs?.map((log, i) => <div key={i}>{log.time} {log.log}</div>)}
    </div>
  );
}
