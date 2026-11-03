import type { Block } from "@/lib/types";
import { Formula, MathText } from "./MathText";

export function Blocks({ blocks }: { blocks: Block[] }) {
  return (
    <div className="space-y-4 text-[1.05rem] leading-7">
      {blocks.map((block, index) => {
        if (block.type === "heading") {
          return (
            <h3 key={index} className="pt-2 text-xl text-ink">
              {block.text}
            </h3>
          );
        }
        if (block.type === "math") {
          return <Formula key={index} latex={block.latex} display />;
        }
        if (block.type === "bullets") {
          return (
            <ul key={index} className="list-disc space-y-2 pl-5">
              {block.items.map((item) => (
                <li key={item}>
                  <MathText text={item} />
                </li>
              ))}
            </ul>
          );
        }
        return (
          <p key={index}>
            <MathText text={block.text} />
          </p>
        );
      })}
    </div>
  );
}
