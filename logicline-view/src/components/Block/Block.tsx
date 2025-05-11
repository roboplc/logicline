import "./Block.css";
import { Step, BlockClickHandler, InputKind } from "../../types";

export const Block = ({
  step,
  view_passed,
  onClick
}: {
  step: Step;
  view_passed: boolean;
  onClick?: BlockClickHandler;
}) => {
  const inputs = Array.isArray(step.input) ? step.input : [step.input];

  function formattedValues(value: unknown) {
    if (typeof value === "object" && value !== null) {
      const entries = Object.entries(value);

      const entryLimit = 3;
      const limitedEntries = entries.slice(0, entryLimit);
      const formattedEntries = limitedEntries.map(([key, val]) => {
        const combined = `${key}:${JSON.stringify(val).replace(/["/]/g, "")}`;

        const formattedCombined =
          combined.length > 20 ? combined.slice(0, 20) + "..." : combined;

        return (
          <div key={key} className="logicline_block-field">
            <span key={key} title={combined}>
              {formattedCombined}
            </span>
          </div>
        );
      });

      const hiddenEntries = entries.slice(entryLimit);
      if (hiddenEntries.length > 0) {
        const hiddenEntriesFormatted = hiddenEntries
          .map(
            ([key, val]) => `${key}:${JSON.stringify(val).replace(/["/]/g, "")}`
          )
          .join(", ");
        formattedEntries.push(
          <div key="more" className="logicline_block-tooltip">
            <span title={hiddenEntriesFormatted}>...</span>
          </div>
        );
      }

      return formattedEntries;
    }

    const valueStr = String(value);
    return valueStr.length > 20 ? valueStr.slice(0, 20) + "..." : valueStr;
  }

  return (
    <div className="logicline_block-wrapper">
      {step.input_kind === InputKind.External && (
        <>
          <div
            className={`logicline_external ${
              view_passed ? "" : "logicline_external--disabled"
            }`}
          />
          <div
            className={`logicline_external-line ${
              view_passed ? "" : "logicline_external-line--disabled"
            }`}
          >
            <div
              className={`logicline_external-arrow-tip ${
                view_passed ? "" : "logicline_external-arrow-tip--disabled"
              }`}
            />
          </div>
        </>
      )}

      <div
        className="logicline_block"
        onClick={() => {
          if (onClick) {
            onClick(step);
          }
        }}
      >
        <div
          className={`logicline_block-header ${
            view_passed ? "" : "logicline_block-disabled"
          }`}
        >
          {step.name}
        </div>
        <div
          className="logicline_block-body"
          style={{
            padding: inputs.every((input) => input === null)
              ? "9px 6px"
              : "0 6px"
          }}
        >
          {inputs.map((inputValue, idx) => (
            <div key={idx} className="logicline_block-input">
              <div className="logicline_block-value">
                {inputValue !== null ? formattedValues(inputValue) : ""}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
