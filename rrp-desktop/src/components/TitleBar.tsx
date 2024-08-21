import {
  makeStyles,
  Text,
  tokens,
  typographyStyles,
} from "@fluentui/react-components";

const useStyles = makeStyles({
  titleText: typographyStyles.title3,
  title: {
    backgroundColor: tokens.colorBrandBackground,
    color: tokens.colorNeutralForegroundOnBrand,
  },
});

export function TitleBar(props: { children?: React.ReactNode }) {
  const styles = useStyles();
  return (
    <div
      className={"flex items-center py-2 px-2 gap-2 " + styles.title}
    >
      <Text className={styles.titleText} as="h1">rrp-client</Text>
      {props.children}
    </div>
  );
}
