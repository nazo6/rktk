import { Connection } from "@/lib/connection";
import {
  Button,
  Checkbox,
  Input,
  Title2,
  Toast,
  ToastBody,
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
import { StateConfig } from "rktk-rrp-client-webhid";

export function KeyboardOptionsPage(props: { connection: Connection }) {
  const { data: keymapConfig, error: fetchError } = useQuery({
    queryKey: ["getKeymapConfig"],
    queryFn: async () => await props.connection.client.get_keymap_config(),
  });

  if (fetchError) console.error(fetchError);

  return keymapConfig
    ? (
      <KeyboardOptionsPageInner
        keymapConfig={keymapConfig}
        connection={props.connection}
      />
    )
    : fetchError
    ? <div>Failed to fetch keymap config: {fetchError.message}</div>
    : <></>;
}

function KeyboardOptionsPageInner(
  props: { keymapConfig: StateConfig; connection: Connection },
) {
  const { dispatchToast } = useToastController();
  const queryClient = useQueryClient();

  const setKeymapConfig = useMutation({
    mutationFn: async (conf: StateConfig) =>
      await props.connection.client.set_keymap_config(conf),
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
    onError: (e) => {
      dispatchToast(
        <Toast>
          <ToastTitle>
            Failed to update keymap config
          </ToastTitle>
          <ToastBody>
            {e.message}
          </ToastBody>
        </Toast>,
        { intent: "error" },
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
        className="w-full max-w-xl bg-white dark:bg-gray-800 rounded-md p-2 flex flex-col gap-2"
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
            name="key_resolver.tap_hold.threshold"
            title="Tap-hold threshold"
            control={control}
          />
          <BoolForm
            name="key_resolver.tap_hold.hold_on_other_key"
            title="Make Tap-hold hold on other key pressed"
            control={control}
          />
          <NumberForm
            name="key_resolver.tap_dance.threshold"
            title="Tap-dance threshold"
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

function BoolForm<T extends FieldValues>(props: {
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
          <Checkbox
            {...field}
            checked={field.value}
            className="col-span-2"
            onChange={(e) => field.onChange(e.target.checked)}
          />
        )}
      />
    </>
  );
}
