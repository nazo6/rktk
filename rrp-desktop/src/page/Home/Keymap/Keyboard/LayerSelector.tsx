import { ToggleButton } from "@fluentui/react-components";

export function LayerSelector(
  props: {
    layer: number;
    setLayer: (layer: number) => void;
    layerCount: number;
  },
) {
  return (
    <div className="flex flex-col">
      {Array.from(Array(props.layerCount)).map((_, i) => (
        <ToggleButton
          appearance="subtle"
          key={i}
          checked={i == props.layer}
          onClick={() => props.setLayer(i)}
          icon={<span>{i}</span>}
        />
      ))}
    </div>
  );
}
