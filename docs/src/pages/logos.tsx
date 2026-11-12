import { Metadata } from "#/components/metadata";
import { Download } from "lucide-react";
import styles from "./logos.module.css";

const assets = [
  { name: "Logo", file: "comfit-logo", width: 765, height: 244 },
  { name: "Icon", file: "comfit-icon", width: 240, height: 244 },
] as const;
const appearances = ["light", "dark"] as const;

export default function LogosPage() {
  return (
    <div className={styles.page}>
      <Metadata
        title="Logos"
        description="Download Comfit logos and icons in SVG format for light and dark backgrounds."
        path="/logos"
      />
      <header className={styles.header}>
        <h1>Logos</h1>
        <p>Download the Comfit logo and icon as SVGs, with variants for light and dark backgrounds.</p>
      </header>
      {assets.map(({ name, file, width, height }) => (
        <section key={file} className={styles.section} aria-labelledby={file}>
          <h2 id={file}>{name}</h2>
          <div className={styles.assets}>
            {appearances.map((appearance) => {
              const filename = `${file}-${appearance}.svg`;
              const label = `${appearance === "light" ? "Light" : "Dark"} background`;
              return (
                <figure key={appearance}>
                  <div className={styles.preview} data-appearance={appearance}>
                    <img
                      className={name === "Logo" ? styles.logo : styles.icon}
                      src={`/logos/${filename}`}
                      alt={`Comfit ${name.toLowerCase()} for ${appearance} backgrounds`}
                      width={width}
                      height={height}
                    />
                  </div>
                  <figcaption className={styles.caption}>
                    <span>{label}</span>
                    <a
                      href={`/logos/${filename}`}
                      download={filename}
                      aria-label={`Download ${name.toLowerCase()} for ${appearance} backgrounds (SVG)`}
                    >
                      <Download size={16} aria-hidden="true" />
                      Download SVG
                    </a>
                  </figcaption>
                </figure>
              );
            })}
          </div>
        </section>
      ))}
    </div>
  );
}

export const getConfig = async () => {
  return { render: "static" } as const;
};
