import { themeAtom } from "@/App";
import {
  makeStyles,
  Text,
  tokens,
  Toolbar,
  ToolbarRadioButton,
  ToolbarRadioGroup,
  typographyStyles,
} from "@fluentui/react-components";
import {
  LaptopFilled,
  WeatherMoonFilled,
  WeatherSunnyFilled,
} from "@fluentui/react-icons";
import { useAtom } from "jotai";

const useStyles = makeStyles({
  titleText: typographyStyles.title3,
  title: {
    backgroundColor: tokens.colorBrandBackground,
    color: tokens.colorNeutralForegroundOnBrand,
  },
});

export function TitleBar(props: { children?: React.ReactNode }) {
  const [theme, setTheme] = useAtom(themeAtom);

  const styles = useStyles();
  return (
    <div
      className={"flex items-center py-2 px-2 gap-2 " + styles.title}
    >
      <Text className={styles.titleText} as="h1">rrp-client</Text>
      <Toolbar
        checkedValues={{ theme: [theme] }}
        onCheckedValueChange={(_, data) =>
          setTheme(data.checkedItems[0] as any)}
      >
        <ToolbarRadioGroup>
          <ToolbarRadioButton
            name="textOptions"
            value="light"
            icon={<WeatherSunnyFilled />}
          />
          <ToolbarRadioButton
            name="theme"
            value="dark"
            icon={<WeatherMoonFilled />}
          />
          <ToolbarRadioButton
            name="theme"
            value="system"
            icon={<LaptopFilled />}
          />
        </ToolbarRadioGroup>
      </Toolbar>
      {props.children}
    </div>
  );
}
