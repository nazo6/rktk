import { themeAtom } from "@/App";
import {
  Button,
  makeStyles,
  Menu,
  MenuItem,
  MenuList,
  MenuPopover,
  MenuTrigger,
  Text,
  tokens,
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

const Theme = <T extends string>(props: { theme: T }) => (
  <p className="flex items-center gap-1">
    {props.theme === "dark"
      ? (
        <>
          <WeatherMoonFilled /> Dark
        </>
      )
      : props.theme === "light"
      ? (
        <>
          <WeatherSunnyFilled /> Light
        </>
      )
      : (
        <>
          <LaptopFilled /> System
        </>
      )}
  </p>
);

export function TitleBar(props: { children?: React.ReactNode }) {
  const [theme, setTheme] = useAtom(themeAtom);

  const styles = useStyles();
  return (
    <div
      className={"flex items-center py-2 px-2 gap-3 " + styles.title}
    >
      <Text className={styles.titleText} as="h1">rrp-client</Text>

      <div className="ml-auto flex items-center">
        {props.children}
      </div>

      <Menu>
        <MenuTrigger disableButtonEnhancement>
          <Button
            appearance="secondary"
            className="flex items-center !ml-auto !w-20"
            size="small"
          >
            <Theme theme={theme} />
          </Button>
        </MenuTrigger>

        <MenuPopover>
          <MenuList>
            <MenuItem onClick={() => setTheme("dark")}>
              <Theme theme="dark" />
            </MenuItem>
            <MenuItem onClick={() => setTheme("light")}>
              <Theme theme="light" />
            </MenuItem>
            <MenuItem onClick={() => setTheme("system")}>
              <Theme theme="system" />
            </MenuItem>
          </MenuList>
        </MenuPopover>
      </Menu>
    </div>
  );
}
