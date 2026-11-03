import katex from "katex";

function renderLatex(latex: string, display: boolean): string | null {
  try {
    return katex.renderToString(latex, {
      displayMode: display,
      throwOnError: true,
      trust: false,
      strict: "ignore",
      output: "htmlAndMathml",
    });
  } catch {
    return null;
  }
}

export function Formula({ latex, display = false }: { latex: string; display?: boolean }) {
  const html = renderLatex(latex, display);
  if (!html) {
    return <code className="math-fallback">{latex}</code>;
  }
  return (
    <span
      className={display ? "formula-display" : "formula-inline"}
      // KaTeX-HTML aus vertrauenswürdigem Lektionsinhalt. trust ist aus, Nutzereingaben laufen hier nicht durch.
      dangerouslySetInnerHTML={{ __html: html }}
    />
  );
}

export function MathText({ text }: { text: string }) {
  const parts = text.split(/(\$[^$]+\$)/g);
  return (
    <>
      {parts.map((part, index) => {
        if (part.startsWith("$") && part.endsWith("$") && part.length > 2) {
          return <Formula key={`${part}-${index}`} latex={part.slice(1, -1)} />;
        }
        return <span key={`${part}-${index}`}>{part}</span>;
      })}
    </>
  );
}
