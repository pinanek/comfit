import { Home } from "#/features/home/home";

export default function HomePage() {
  return <Home />;
}

export const getConfig = async () => {
  return {
    render: "static",
  } as const;
};
