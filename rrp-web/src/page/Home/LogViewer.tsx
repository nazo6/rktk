import { Connection } from "@/lib/connection";
import { useInfiniteQuery, useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

export function LogViewerPage(props: { connection: Connection }) {
  const { data: deviceTimeToRealTime } = useQuery({
    queryKey: ["getNow"],
    queryFn: async () => {
      const deviceTime = await props.connection.client.get_now();
      const now = new Date().getTime();
      const timeMap = [Number(deviceTime), now] as const;

      return (deviceTime: number) => {
        return timeMap[1] + (deviceTime - timeMap[0]);
      };
    },
  });
  const { data, fetchNextPage } = useInfiniteQuery({
    enabled: !!deviceTimeToRealTime,
    queryKey: ["getLog"],
    queryFn: async () => {
      const data = await props.connection.client.get_log();
      return data.map((log) => {
        const date = new Date(deviceTimeToRealTime!(log.time));
        const time = `${String(date.getHours()).padStart(2, "0")}:${
          String(date.getMinutes()).padStart(2, "0")
        }:${String(date.getSeconds()).padStart(2, "0")}.${
          String(date.getMilliseconds()).padStart(3, "0")
        }`;
        return {
          time,
          message: log.message,
          level: log.level,
          line: log.line,
        };
      });
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
      {logs?.map((log, i) => (
        <div key={i}>{log.time} {log.level} {log.line} {log.message}</div>
      ))}
    </div>
  );
}
