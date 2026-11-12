import { site } from "#/site";

export type MetadataValues = {
  title?: string;
  description?: string;
  path?: string;
  image?: string;
  imageAlt?: string;
  type?: "website" | "article";
};

export function Metadata({
  title = site.title,
  description = site.description,
  path = "/",
  image = site.image,
  imageAlt = site.imageAlt,
  type = "website",
}: MetadataValues) {
  const pageTitle = title === site.title ? title : `${title} | ${site.name}`;
  const pageUrl = new URL(path, site.url).href;
  const imageUrl = new URL(image, site.url).href;

  return (
    <>
      <title>{pageTitle}</title>
      <meta name="description" content={description} />
      <link rel="canonical" href={pageUrl} />
      <meta property="og:type" content={type} />
      <meta property="og:title" content={pageTitle} />
      <meta property="og:site_name" content={site.name} />
      <meta property="og:description" content={description} />
      <meta property="og:url" content={pageUrl} />
      <meta property="og:image" content={imageUrl} />
      <meta property="og:image:width" content="1200" />
      <meta property="og:image:height" content="630" />
      <meta property="og:image:alt" content={imageAlt} />
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:title" content={pageTitle} />
      <meta name="twitter:description" content={description} />
      <meta name="twitter:site" content={site.twitter} />
      <meta name="twitter:creator" content={site.twitter} />
      <meta name="twitter:image" content={imageUrl} />
      <meta name="twitter:image:alt" content={imageAlt} />
    </>
  );
}
