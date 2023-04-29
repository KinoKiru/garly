import { defineConfig } from "unocss";
import presetWind from "@unocss/preset-wind";
import presetWebFonts from "@unocss/preset-web-fonts";
import presetIcons from "@unocss/preset-icons";

export default defineConfig({
  presets: [
    presetWind({
      dark: "class"
    }),
    presetIcons(),
    presetWebFonts({
      provider: "google",
      fonts: {
        sans: "Press Start 2P",
      },
    }),
  ],
});
