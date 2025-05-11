import "./RackView.css";
import { Block } from "..";
import { Snapshot, BlockClickHandler } from "../../types";

export const RackView = ({
  data,
  onBlockClick
}: {
  data: Snapshot;
  onBlockClick?: BlockClickHandler;
}) => {
  const lines = Object.values(data.lines);

  return (
    <>
      <div className="logicline_line-container">
        {lines.map((line, idx) => {
          let active = true;

          const isBranch = line.steps.some((step) => Array.isArray(step));

          return (
            <div
              key={idx}
              className="logicline_line-wrapper"
              style={{
                height: isBranch ? "250px" : "130px"
              }}
            >
              <div className="logicline_line-title">{line.name}</div>
              {line.steps.map((step, idx) => {
                const isLastStep = idx === line.steps.length - 1;
                const nextStep =
                  idx < line.steps.length - 1 ? line.steps[idx + 1] : null;
                const nextIsBranch = Array.isArray(nextStep);

                const view_passed = active;

                if (Array.isArray(step)) {
                  const activeTop = step[0].passed;
                  const activeBottom = step[1].passed;

                  active = step.some((s) => s.passed);

                  return (
                    <div key={idx} className="logicline_branch-wrapper">
                      <div className="logicline_branch-lines">
                        <div
                          className={`logicline_branch-top  logicline_branch-top-left ${
                            !isLastStep
                              ? activeTop
                                ? "logicline_branch-top-right"
                                : "logicline_branch-top-right--disabled"
                              : ""
                          } ${
                            !view_passed
                              ? "logicline_branch-top-left--disabled"
                              : ""
                          }`}
                        >
                          <div
                            className={`logicline_arrow-left-top ${
                              !view_passed
                                ? "logicline_arrow-left-top--disabled"
                                : ""
                            }`}
                          />
                          <Block
                            step={step[0]}
                            onClick={onBlockClick}
                            view_passed={view_passed}
                          />

                          {!isLastStep && (
                            <div
                              className={`logicline_arrow-right-top ${
                                !activeTop
                                  ? "logicline_arrow-right-top--disabled"
                                  : ""
                              }`}
                            />
                          )}
                        </div>

                        <div
                          className={`logicline_branch-bottom logicline_branch-bottom-left ${
                            !isLastStep
                              ? activeBottom
                                ? "logicline_branch-bottom-right"
                                : "logicline_branch-bottom-right--disabled"
                              : ""
                          } ${
                            !view_passed
                              ? "logicline_branch-bottom-left--disabled"
                              : ""
                          }`}
                        >
                          <div
                            className={`logicline_arrow-left-bottom ${
                              !view_passed
                                ? "logicline_arrow-left-bottom--disabled"
                                : ""
                            }`}
                          />

                          <Block
                            step={step[1]}
                            onClick={onBlockClick}
                            view_passed={view_passed}
                          />
                          {!isLastStep && (
                            <div
                              className={`logicline_arrow-right-bottom ${
                                !activeBottom
                                  ? "logicline_arrow-right-bottom--disabled"
                                  : ""
                              }`}
                            />
                          )}
                        </div>
                      </div>

                      {!isLastStep && (
                        <div
                          className={`logicline_merge-line ${
                            !active ? "logicline_merge-line--disabled" : ""
                          }`}
                        />
                      )}
                    </div>
                  );
                } else {
                  active = step.passed;

                  return (
                    <div
                      key={idx}
                      className="logicline_block-group"
                      style={{
                        marginTop: isBranch ? "0" : "-1.5%"
                      }}
                    >
                      <Block
                        step={step}
                        onClick={onBlockClick}
                        view_passed={view_passed}
                      />
                      {!isLastStep &&
                        (nextIsBranch ? (
                          <div
                            className={`logicline_straight ${
                              active ? "" : "logicline_straight-disabled"
                            }`}
                          />
                        ) : (
                          <div
                            className={`${
                              active
                                ? "logicline_straight-arrow"
                                : "logicline_straight-arrow-disabled"
                            }`}
                          />
                        ))}
                    </div>
                  );
                }
              })}
            </div>
          );
        })}
      </div>
    </>
  );
};
