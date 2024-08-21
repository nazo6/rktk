import { commands, KeyboardInfo, StateConfig } from "@/bindings";
import { unwrapped } from "@/utils";
import {
  Button,
  Input,
  Title2,
  Toast,
  ToastTitle,
  useToastController,
} from "@fluentui/react-components";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  Control,
  Controller,
  FieldValues,
  Path,
  useForm,
} from "react-hook-form";

export function KeyboardOptionsPage(props: { keyboardInfo: KeyboardInfo }) {
  const { data: keymapConfig, error: fetchError } = useQuery({
    queryKey: ["getKeymapConfig"],
    queryFn: async () => unwrapped(commands.getKeymapConfig)(null),
  });

  return keymapConfig
    ? (
      <KeyboardOptionsPageInner
        keymapConfig={keymapConfig}
        keyboardInfo={props.keyboardInfo}
      />
    )
    : fetchError
    ? <div>Failed to fetch keymap config: {fetchError.message}</div>
    : <div>Loading...</div>;
}

function KeyboardOptionsPageInner(
  props: { keymapConfig: StateConfig; keyboardInfo: KeyboardInfo },
) {
  const { dispatchToast } = useToastController();
  const queryClient = useQueryClient();

  const setKeymapConfig = useMutation({
    mutationFn: unwrapped(commands.setKeymapConfig),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getKeymapConfig"] });
      dispatchToast(
        <Toast>
          <ToastTitle>
            Updated keymap config
          </ToastTitle>
        </Toast>,
        { intent: "success" },
      );
    },
  });

  const { handleSubmit, control } = useForm<
    StateConfig
  >({
    defaultValues: props.keymapConfig,
  });

  const onSubmit = handleSubmit((val) => setKeymapConfig.mutate(val));

  return (
    <div className="p-2 flex justify-center w-full">
      <form
        className="w-full max-w-xl bg-white rounded-md p-2 flex flex-col gap-2"
        onSubmit={onSubmit}
      >
        <Title2>Mouse</Title2>
        <div className="grid grid-cols-3 items-center gap-y-2">
          <NumberForm
            name="mouse.auto_mouse_layer"
            title="Auto mouse layer"
            control={control}
          />
          <NumberForm
            name="mouse.auto_mouse_threshold"
            title="Auto mouse threshold"
            control={control}
          />
          <NumberForm
            name="mouse.auto_mouse_duration"
            title="Auto mouse duration (ms)"
            control={control}
          />
          <NumberForm
            name="mouse.scroll_divider_x"
            title="Scroll divider X"
            control={control}
          />
          <NumberForm
            name="mouse.scroll_divider_y"
            title="Scroll divider Y"
            control={control}
          />
        </div>

        <Title2>Key resolver</Title2>
        <div className="grid grid-cols-3 items-center gap-y-2">
          <NumberForm
            name="key_resolver.tap_threshold"
            title="Tap threshold"
            control={control}
          />
          <NumberForm
            name="key_resolver.tap_dance_threshold"
            title="Tap dance threshold"
            control={control}
          />
        </div>

        <Button type="button" onClick={onSubmit}>Update</Button>
      </form>
    </div>
  );
}

function NumberForm<T extends FieldValues>(props: {
  name: Path<T>;
  title: string;
  control: Control<T>;
}) {
  return (
    <>
      <p className="col-span-1">{props.title}</p>
      <Controller
        name={props.name}
        control={props.control}
        render={({ field }) => (
          <Input
            {...field}
            value={String(field.value)}
            className="col-span-2"
            type="number"
            onChange={(e) => field.onChange(parseInt(e.target.value))}
          />
        )}
      />
    </>
  );
}
