import { Input, Select } from "@fluentui/react-components";
import { LayerOp } from "../../bindings";

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

export function LayerKeySelector(
  props: {
    selected: LayerOp;
    setSelected: (key: LayerOp) => void;
    layerCount: number;
  },
) {
  const layerOpType = Object.keys(props.selected)[0];
  const layerOpLayer = props.selected[layerOpType as keyof LayerOp] as number;

  return (
    <div className="flex gap-2">
      <Select
        className="flex-grow"
        onChange={(e) => {
          const key = e.target.value as keyof LayerOp;
          props.setSelected({ [key]: layerOpLayer } as any);
        }}
      >
        <option key="Toggle">Toggle</option>
        <option key="Momentary">Momentary</option>
      </Select>
      <Input
        type="number"
        value={layerOpLayer as any}
        onChange={(_, v) => {
          const l = parseInt(v.value);
          if (l && l >= 0 && l < props.layerCount) {
            props.setSelected({ [layerOpType]: l } as any);
          }
        }}
      />
    </div>
  );
}
