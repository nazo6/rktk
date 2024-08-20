import { Select } from "@fluentui/react-components";

export function KeySelector<T>(
  props: {
    keys: Map<T, string>;
    selected: T;
    setSelected: (key: T) => void;
  },
) {
  return (
    <div className="flex flex-col">
      <Select
        value={props.keys.get(props.selected)}
        onChange={(e) => {
          const key = Array.from(props.keys.entries()).find(([_, label]) =>
            label == e.target.value
          )?.[0];
          if (key) {
            props.setSelected(key);
          }
        }}
      >
        {Array.from(props.keys.entries()).map(([key, label]) => (
          <option key={String(key)}>
            {label}
          </option>
        ))}
      </Select>
    </div>
  );
}
